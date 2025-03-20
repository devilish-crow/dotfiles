name="Sway"
description="Wayland compositor with PipeWire and D-Bus"

start() {
  # Session
  export XDG_SESSION_TYPE=wayland
  export XDG_SESSION_DESKTOP=sway
  export XDG_CURRENT_DESKTOP=sway

  # Wayland stuff
  export MOZ_ENABLE_WAYLAND=1
  export QT_QPA_PLATFORM=wayland
  export SDL_VIDEODRIVER=wayland
  export _JAVA_AWT_WM_NONREPARENTING=1

  # Launch Sway with dbus and pipewirea
  dbus-run-session sway "$@" &
  sleep 2
  bash /usr/libexec/pipewire-launcher
}
