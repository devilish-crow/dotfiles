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
- neovim (code editor for rust and assembly, I'll add more LSPs when I'll need them)

Installing:
-----------
WARNING: The installation process will wipe your ~/.profile and /etc/greetd/config.toml <br>
You can use the `install` ash script to install this configuration on your Alpine machine. You could also execute `sway-run` to start the desktop enviroment if you don't like tuigreet.

Networking:
-----------
Impala does not currently support connecting to or changing the connected wireless network, you will need to press Ctrl+Alt+T to open your default shell in foot and then use the cli frontend of the internet wireless daemon, iwcli.

Sound:
------
Your audio input might feel the effects of echoing. To fix that, you will need to set up your applications to use the echo cancel source as your input device and maybe fiddle around with your volume settings. You can open the volume management app by clicking on the yellow sound icon at the top right corner of your screen.

Screenshots:
------------
![desktop wallpaper](https://media.discordapp.net/attachments/1140271425719107766/1358188174207226027/screenshot-04-06_00.png?ex=683d6b8e&is=683c1a0e&hm=83f0be878256bcbeca31d9697482af7387f1c57c1f77f9cd9743e094e77407cc&=&format=webp&quality=lossless&width=802&height=451)
![app launcher](https://media.discordapp.net/attachments/1140271425719107766/1358188360497102858/screenshot-04-06_00.png?ex=683d6bbb&is=683c1a3b&hm=5c65e6e7f12e2d370eb6f8363e1b6638963986fabedd4f2b73f862c56f248969&=&format=webp&quality=lossless&width=802&height=451)
![system management apps](https://media.discordapp.net/attachments/1140271425719107766/1358188459390402713/screenshot-04-06_00.png?ex=683d6bd2&is=683c1a52&hm=c58fba106488d2ad0c55f8d60f0e10f1cc8a028d0d488dc270a73ace1e757d9f&=&format=webp&quality=lossless&width=802&height=451)

Kiosk Mode:
-----------
This setup can be used for kiosk systems in case you need better reliability. If you select it, cagebreak will be installed instead of sway. This exists just because I couldn't get my RPI's GPU to work with sway. :/
