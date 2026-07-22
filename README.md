Sway Dotfiles for Alpine Linux
------------------------------
These are my dotfiles for Alpine Linux. They include:
- sway (compositor)
- grimshot (screenshots)
- pipewire (audio)
- bluez (bluetooth)
- iwd (wifi)
- greetd with tuigreet (login manager)
- xdg-desktop-portal (desktop portal support)
- rofi (app launcher)
- waybar (status bar)
- foot (terminal)
- helix (code editor)

Installing:
-----------
> [!WARNING]
> The installation process will wipe your configurations for iwd and greetd whilst also overwriting your .profile <br>

You can use `ansible-playbook playbook.yml --ask-become-pass` to install this configuration on your Alpine machine. You could also execute `sway-run` to start the desktop enviroment if you don't like tuigreet.

Hibernation:
------------
I can't be bothered to add support for GRUB, which is the default bootloader on all UEFI devices, to the hibernation option so I recommend skipping it if you have a device made in the past decade.

Sound:
------
Your audio input might feel the effects of echoing. To fix that, you will need to set up your applications to use the echo cancel source as your input device and maybe fiddle around with your volume settings.

Screenshots:
------------
![first](https://raw.githubusercontent.com/devilish-crow/dotfiles/main/images/first.png)
![second](https://raw.githubusercontent.com/devilish-crow/dotfiles/main/images/second.png)
![third](https://raw.githubusercontent.com/devilish-crow/dotfiles/main/images/third.png)
