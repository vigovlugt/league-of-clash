set -e

cargo build --release
cp ../target/release/league-of-clash-functions .

func start