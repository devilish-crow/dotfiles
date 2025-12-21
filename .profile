export PATH=~/.cargo/bin/:~/.rustup/toolchains/stable-x86_64-unknown-linux-musl/bin/:$PATH
export MANPAGER=less

# Utility functions
adm() {
  doas apk add $1
  doas apk add $1-doc
}

ad() {
  doas apk add $1
}

# Utility Aliases
alias "pw"="sudo poweroff"
alias "rb"="sudo reboot"
alias "ls"="ls --color"
alias "rp"="doas adb start-server && adb reverse tcp:53203 tcp:53203"
