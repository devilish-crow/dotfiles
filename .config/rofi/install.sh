# This is an optional package to add additional menus to the system
sudo apk add font-jetbrains-mono
sudo git clone https://github.com/adi1090x/rofi/ /tmp/
sudo mv /tmp/rofi/fonts/* /usr/share/fonts/
sudo rm -rf /tmp/rofi
