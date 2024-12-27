FROM rust:bookworm AS builder

WORKDIR /usr/src/app

COPY .cargo/config.toml ~/.cargo/config.toml

COPY . .

RUN cargo build --bin server --release

FROM debian:bookworm-slim

WORKDIR /app

ENV http_proxy=http://127.0.0.1:7890
ENV https_proxy=http://127.0.0.1:7890

RUN apt-get update && \
    apt-get install -y libssl-dev && \
    rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/app/target/release/server .

CMD [ "/app/server" ]



