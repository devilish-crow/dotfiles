export PATH=~/.cargo/bin/:~/.rustup/toolchains/stable-x86_64-unknown-linux-musl/bin/:$PATH
export MANPAGER=less

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
