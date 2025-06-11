FROM rust:latest

WORKDIR /usr/src/app

COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

RUN cargo build --release

CMD ["./target/release/atmos"]
