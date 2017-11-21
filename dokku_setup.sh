curl -o /tmp/rustup-init.sh "https://sh.rustup.rs"
wait
sh /tmp/rustup-init.sh -y
export PATH="$HOME/.cargo/bin:$PATH"
npm i -g neon-cli
neon build