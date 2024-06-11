FROM rust:1.78.0-slim as dev

RUN apt-get update && apt-get install -y pkg-config libssl-dev

# ユーザー,グループを作成
RUN groupadd -g 1000 rust && useradd -u 1000 -g rust rust
RUN mkdir /app && chown rust:rust /app

WORKDIR /app
COPY --chown=rust:rust /app .

USER rust

CMD ["cargo", "run"]

FROM dev as builder

WORKDIR /app
RUN cargo build --release

FROM debian:12.5-slim as prod

RUN apt-get update && apt-get install -y pkg-config libssl-dev

# ユーザー,グループを作成
RUN groupadd -g 1000 rust && useradd -u 1000 -g rust rust
RUN mkdir /app && chown rust:rust /app

WORKDIR /app
COPY --chown=rust:rust --from=builder /app/target/release/rust_scraping .

CMD ["./rust_scraping"]