set -e

cargo build --release
cp ../target/release/league_of_clash_functions .

func start