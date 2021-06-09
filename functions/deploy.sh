set -e

cargo build --release --target=x86_64-pc-windows-gnu
cp ../target/x86_64-pc-windows-gnu/release/league-of-clash-functions.exe .

# func azure functionapp publish league-of-clash