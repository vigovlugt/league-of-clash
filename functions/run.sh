set -e

cargo build --release
cp ../target/release/league-of-clash-functions .

export RUST_LOG=debug

func start