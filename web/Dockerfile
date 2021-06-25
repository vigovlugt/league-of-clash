FROM rust as planner
WORKDIR app
RUN cargo install cargo-chef 
COPY . .
RUN cargo chef prepare  --recipe-path recipe.json

FROM rust as cacher
WORKDIR app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

FROM rust as builder
WORKDIR app
COPY . .
# Copy over the cached dependencies
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin league-of-clash-web

FROM debian:buster-slim AS runtime
WORKDIR app
RUN apt-get update -y \
    && apt-get autoremove -y \
    && apt-get clean -y \
    && rm -rf /var/lib/apt/lists/*
WORKDIR app
COPY --from=builder /app/target/release/league-of-clash-web league-of-clash-web
ENTRYPOINT ["./league-of-clash-web"]