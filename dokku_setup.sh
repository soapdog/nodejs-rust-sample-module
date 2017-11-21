curl https://sh.rustup.rs -o /tmp/rustup-init.sh
wait
sh /tmp/rustup-init.sh -y
export PATH="$HOME/.cargo/bin:$PATH"
npm i -g neon-cli
neon build