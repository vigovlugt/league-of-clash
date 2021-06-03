set -e

export PKG_CONFIG_ALLOW_CROSS=1
export OPENSSL_STATIC=true
export OPENSSL_DIR=/musl

cargo build --release --target=x86_64-unknown-linux-musl
cp ../target/x86_64-unknown-linux-musl/release/league-of-clash-functions .

func azure functionapp publish league-of-clash