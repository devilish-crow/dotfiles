#!/bin/ash
printf "Please enter your username: "
read USER
while [ -z "$USER" ]; do
    printf "Username cannot be empty. Please enter a valid username: "
    read USER
done

echo "Installing core packages for system setup..."
doas apk update && apk upgrade
doas apk add linux-firmware mesa-dri-gallium iwd openresolv seatd greetd greetd-tuigreet xwayland sway swaybg waybar grimshot clipman pipewire pipewire-pulse pipewire-echo-cancel wireplumber flatpak bubblewrap || { echo "Installation failed! Exiting."; exit 1; }
doas apk add font-jetbrains-mono-nerd htop pulsemixer foot impala cargo firefox syncthing || echo "Failed to install extra tools!"

doas -u $USER curl https://raw.githubusercontent.com/cargo-bins/cargo-binstall/c85f0eeb20318afaf27164c73f5411082b5b8675/install-from-binstall-release.sh | ash
export PATH=$PATH:~/.cargo/bin/

printf "Do you need a code editor? (yes/no): "
read neovim
if [ "$neovim" = "yes" ]; then 
 doas apk add neovim build-base
fi

printf "Do you want bluetooth? (yes/no): "
read bluetooth
if [ "$bluetooth" = "yes" ]; then
    echo "Setting up bluetooth related daemons..."
    doas rfkill unblock bluetooth 2> /dev/null
    doas apk add bluez bluez-openrc pipewire-spa-bluez wireplumber-bluez playerctl || { echo "Failed to install Bluetooth tools!"; exit 1; }
    doas rc-update add bluetooth
    cargo binstall bluetui || echo "Failed to install bluetui!"
fi

printf "Do you want to enable screen sharing? (yes/no): "
read portals
if [ "$portals" = "yes" ]; then
    echo "Setting up desktop portals"
    doas apk add xdg-desktop-portal xdg-desktop-portal-wlr || { echo "Failed to install desktop portals!"; exit 1; }
fi

printf "Do you want to enable automatic backround updates? (yes/no): "
read update
if [ "$update" = "yes" ]; then
    echo "Writing auto update cronjob"
    cat <<EOF doas tee /etc/periodic/hourly/update > /dev/null
#!/bin/ash
apk update && apk upgrade
flatpak update -y
EOF
    doas chmod +x /etc/periodic/hourly/update
fi

echo "Setting up user..."
doas cp .profile ~/
doas addgroup $USER wheel

echo "Cleaning up old configs"
doas rm /etc/init.d/wpa_* 2> /dev/null
doas rm /etc/network/interfaces 2> /dev/null
doas rm /etc/greetd/config.toml 2> /dev/null

echo "Setting up network interfaces"
cat <<EOF | doas tee /etc/network/interfaces > /dev/null
auto lo
allow-hotplug eth0
allow-hotplug usb0

iface lo inet loopback
iface eth0 inet dhcp
iface usb0 inet dhcp
EOF

echo "Configuring wireless settings..."
doas rm /etc/iwd/main.conf 2> /dev/null
cat <<EOF | doas tee /etc/iwd/main.conf > /dev/null
[General]
EnableNetworkConfiguration=True

[Network]
NameResolvingService=resolvconf
EOF
doas rc-update add iwd boot

echo "Setting up the greeter"
doas cp sway /usr/bin/sway-run
doas mkdir -p /etc/greetd 2> /dev/null
cat <<EOF | doas tee /etc/greetd/config.toml > /dev/null
[terminal]
vt = 7

[default_session]
command = "tuigreet --cmd sway-run --time"
user = "$USER"
EOF
rc-update add greetd default

echo "Installation complete. You now have a desktop! :yeppie:"
