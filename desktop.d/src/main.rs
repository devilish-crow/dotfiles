use std::{fs, process::Command, env};
use ratatui::{
    Terminal, 
    backend::CrosstermBackend, 
    widgets::{Block, Borders, List, ListItem, Paragraph, BorderType}, 
    layout::{Alignment, Layout, Constraint, Direction}, 
    style::{Style, Color}
};
use crossterm::{execute, event::{self, KeyCode}, terminal::{enable_raw_mode, disable_raw_mode, LeaveAlternateScreen, Clear, ClearType}};

#[derive(Debug, Clone)]
struct DesktopEntry {
    name: String,
    description: String,
    start_command: String,
}

fn parse_script(file_path: &str) -> Option<DesktopEntry> {
    let content = fs::read_to_string(file_path).ok()?;
    let mut name = String::new();
    let mut description = String::new();
    let mut start_command = String::new();

    for line in content.lines() {
        if line.starts_with("name=") {
            name = line.split('=').nth(1)?.replace("\"", "");
        } else if line.starts_with("description=") {
            description = line.split('=').nth(1)?.replace("\"", "");
        } else if line.starts_with("start()") {
            start_command = format!("source '{}' && start", file_path);
        }
    }

    if !name.is_empty() && !start_command.is_empty() {
        Some(DesktopEntry { name, description, start_command })
    } else {
        None
    }
}

fn get_desktop_entries(config_dir: &str) -> Vec<DesktopEntry> {
    let mut entries = Vec::new();
    if let Ok(entries_iter) = fs::read_dir(config_dir) {
        for entry in entries_iter.filter_map(|e| e.ok()) {
            let path = entry.path();
            if path.is_file() {
                if let Some(desktop) = parse_script(path.to_str().unwrap()) {
                    entries.push(desktop);
                }
            }
        }
    }

    entries.push(DesktopEntry {
        name: "Exit".to_string(),
        description: "Exit DeskBoot".to_string(),
        start_command: String::new(),
    });

    entries
}

fn run_menu() -> Option<DesktopEntry> {
    let config_dir = format!("{}/.config/desktop.d", env::var("HOME").unwrap());
    let desktops = get_desktop_entries(&config_dir);

    if desktops.is_empty() {
        println!("No desktop entries found in ~/.config/desktop.d/");
        return None;
    }

    enable_raw_mode().unwrap();
    let backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(backend).unwrap();
    terminal.clear().unwrap();
    let mut selected: usize = 0;

    loop {
        terminal.draw(|f| {
            let size = f.size();
            let available_height = size.height.saturating_sub(2); // 2 rows for title and instructions
            let list_height = (desktops.len() as u16 + 2).min(available_height);

            let title = Paragraph::new("DeskBoot Menu")
                .style(Style::default())
                .alignment(Alignment::Center);

            let instructions = Paragraph::new("Press [Enter] to select")
                .style(Style::default().fg(Color::DarkGray))
                .alignment(Alignment::Center);

            let items: Vec<ListItem> = desktops.iter().enumerate().map(|(i, d)| {
                let style = if i == selected {
                    Style::default().fg(Color::Black).bg(Color::White)
                } else {
                    Style::default()
                };
                ListItem::new(format!("{} - {}", d.name, d.description)).style(style)
            }).collect();

            let list = List::new(items)
                .block(Block::default()
                    .borders(Borders::ALL)
                    .border_type(BorderType::Double));  // Use double-line borders for a thicker appearance

            let layout = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),          // Title
                    Constraint::Length(list_height), // List with dynamic height
                    Constraint::Length(1),          // Instructions
                ].as_ref())
                .split(size);

            f.render_widget(title, layout[0]);
            f.render_widget(list, layout[1]);
            f.render_widget(instructions, layout[2]);
        }).unwrap();

        if let event::Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Up => {
                    if selected > 0 {
                        selected -= 1;
                    }
                }
                KeyCode::Down => {
                    if selected < desktops.len() - 1 {
                        selected += 1;
                    }
                }
                KeyCode::Enter => {
                    let chosen_desktop = desktops[selected].clone();
                    disable_raw_mode().unwrap();
                    execute!(terminal.backend_mut(), Clear(ClearType::All), LeaveAlternateScreen).unwrap();

                    // If the exit option was selected, return None
                    if chosen_desktop.name == "Exit" {
                        return None;
                    }

                    return Some(chosen_desktop);
                }
                KeyCode::Esc | KeyCode::Char('q') => {
                    disable_raw_mode().unwrap();
                    execute!(terminal.backend_mut(), Clear(ClearType::All), LeaveAlternateScreen).unwrap();
                    return None;
                }
                _ => {}
            }
        }
    }
}

fn main() {
    if let Some(desktop) = run_menu() {
        execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
        let mut child = Command::new("sh")
            .arg("-c")
            .arg(&desktop.start_command)
            .spawn()
            .expect("Failed to start desktop environment");
        let _ = child.wait();

        execute!(std::io::stdout(), Clear(ClearType::All)).unwrap();
    }
}
