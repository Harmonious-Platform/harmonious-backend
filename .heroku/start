curl https://sh.rustup.rs -sSf > rustup.sh
chmod u+x rustup.sh
./rustup.sh -y --default-toolchain "stable"
rm rustup.sh
source $HOME/.cargo/env
cargo install diesel_cli --no-default-features --features postgres
diesel setup
diesel migration generate users
diesel migration generate invitations
diesel migration run
cargo run
