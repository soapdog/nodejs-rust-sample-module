curl "https://sh.rustup.rs" -o rustup_install.sh
wait
sh ./rustup_install.sh -y 
export PATH="$HOME/.cargo/bin:$PATH"
npm i -g neon-cli
neon build