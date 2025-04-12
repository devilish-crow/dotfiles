Sway Dotfiles for Alpine Linux
------------------------------
These are my dotfiles for Alpine Linux. They include:
- sway (as a tiling window manager and wireless headphone input redirecter)
- grim & slurp (for taking screenshots)
- pipewire (for audio)
- bluez (for bluetooth support)
- pulsemixer (for controlling audio)
- greetd with tuigreet (login manager)
- xdg-desktop-portal (Desktop portal support)
- rofi (stylised app launcher)
- waybar (status bar)
- foot (terminal)

Installing:
-----------
WARNING: The installation process will wipe your ~/.profile and /etc/greetd/config.toml <br>
You can use the `install` ash script to install this configuration on your Alpine machine. You could also execute `sway-run` to start the desktop enviroment.

Networking:
-----------
Impala does not currently support connecting to or changing the connected wireless network, you will need to press Ctrl+Alt+T to open your default shell in foot and then type `iwctl` to open the cli frontend of iwd.

Sound:
------
Your audio input might feel the effects of echoing. To fix that, you will need to set up your applications to use the echo cancel source as your input device and maybe fiddle around with your volume settings. You can open the volume management app by clicking on the sound icon.

Screenshots:
------------
![desktop wallpaper](https://cdn.discordapp.com/attachments/1140271425719107766/1358188174207226027/screenshot-04-06_00.png?ex=67f2eece&is=67f19d4e&hm=18793015382caa7d179c04f7ba9433344bc9e86a39a97ab725405944be63f444&)
![app launcher](https://cdn.discordapp.com/attachments/1140271425719107766/1358188360497102858/screenshot-04-06_00.png?ex=67f2eefb&is=67f19d7b&hm=356d8051e193be066a8072c356a1d05ca7116852eb5756dfcb79d1c377d77754&)
![system management apps](https://cdn.discordapp.com/attachments/1140271425719107766/1358188459390402713/screenshot-04-06_00.png?ex=67f2ef12&is=67f19d92&hm=7abf4e37bd13d9d196a8834828dfa90fd52046e0d9a6ec39faec2ef138d8e49e&)

Kiosk Mode:
-----------
This setup can be used for kiosk systems. The included `install.sh` script can be easily modified to install kiosk desktop enviroments.
