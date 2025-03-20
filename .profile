export PATH=$PATH:~/.cargo/bin/:~/.rustup/toolchains/stable-x86_64-unknown-linux-musl/bin/:~/.local/state/rizin/build/binrz/rizin
export MANPAGER=less
export XDG_RUNTIME_DIR=$HOME
export XDG_CONFIG_PATH=$HOME/.config/

# A somewhat buggy implementation of a dialog menu, good enough for something you'll only use at boot time
desktop

# Utility functions
adm() {
  sudo apk add $1
  sudo apk add $1-doc
}

ad() {
  sudo apk add $1
}

# Utility Aliases
alias "pw"="sudo poweroff"
alias "rb"="sudo reboot"
