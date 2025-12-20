Sway Dotfiles for Alpine Linux
------------------------------
These are my dotfiles for Alpine Linux. They include:
- sway (as a tiling window manager and wireless headphone input redirecter)
- grimshot (for taking screenshots)
- pipewire (for audio)
- bluez (for bluetooth support)
- iwd (for wifi)
- pulsemixer (for controlling audio)
- greetd with tuigreet (login manager)
- xdg-desktop-portal (Desktop portal support)
- rofi (stylised app launcher)
- waybar (status bar)
- foot (terminal)
- neovim (code editor for rust and assembly, I'll add more LSPs when I'll need them)

Installing:
-----------
WARNING: The installation process will wipe your ~/.profile and /etc/greetd/config.toml <br>
You can use the `./install` ash script to install this configuration on your Alpine machine. You could also execute `sway-run` to start the desktop enviroment if you don't like tuigreet.

Sound:
------
Your audio input might feel the effects of echoing. To fix that, you will need to set up your applications to use the echo cancel source as your input device and maybe fiddle around with your volume settings. You can open the volume management app by clicking on the yellow sound icon at the top right corner of your screen.

Screenshots:
------------
![desktop wallpaper](https://raw.githubusercontent.com/devilish-crow/dotfiles/main/images/desktop.png)
![app launcher](https://raw.githubusercontent.com/devilish-crow/dotfiles/main/images/launcher.png)
![app_launcher_alt](https://raw.githubusercontent.com/devilish-crow/dotfiles/main/images/launcher_second.png)
![system management apps](https://raw.githubusercontent.com/devilish-crow/dotfiles/main/images/management.png)
