FROM alpine:latest
RUN apk add --no-cache sqlite curl build-base
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo install sqlx-cli --no-default-features --features sqlite

CMD ["tail", "-f", "/dev/null"]
