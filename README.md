Sway Dotfiles for Alpine Linux
------------------------------
These are my dotfiles for Alpine Linux. They include:
- sway (as a tiling window manager)
- grim & slurp (for taking screenshots)
- pipewire (for audio)
- pulsemixer (for controlling audio)
- greetd with tuigreet (login manager)
- xdg-desktop-portal (Wayland portal support)
- rofi (stylised app launcher)
- waybar (status bar)
- foot (terminal)

Installing:
-----------
WARNING: The installation process will wipe your ~/.profile and /etc/greetd/config.toml
You can use the `install` bash script to install this configuration on your Alpine machine. You could also execute `sway-run` to start the desktop enviroment.

Screenshots:
------------
![desktop wallpaper](https://cdn.discordapp.com/attachments/1140271425719107766/1357789011535728753/screenshot-04-04_21.png?ex=67f17b0e&is=67f0298e&hm=ba64efd4b934b9f6705db52db051b44954ca85e1b02d2aa4e55e955f79e5a4b9&)
![app launcher](https://cdn.discordapp.com/attachments/1140271425719107766/1357789257477128342/screenshot-04-04_21.png?ex=67f17b49&is=67f029c9&hm=6979e858212f7c4d653aeea903378c37099c2ed1ea0c1162de12a8d02453ef9b&)
![system management apps](https://cdn.discordapp.com/attachments/1140271425719107766/1357789473760346232/screenshot-04-04_21.png?ex=67f17b7d&is=67f029fd&hm=b3a8107752489269cdb6e5e3d077a39fb1734fc1539d5191f3e2fe0df2d8103c&)

Kiosk Mode:
-----------
This setup can be used for kiosk systems. The included `install.sh` script can be easily modified to install kiosk desktop enviroments.
