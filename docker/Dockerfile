FROM rust:1.78.0-slim as dev

# ユーザー,グループを作成
RUN groupadd -g 1000 rust && useradd -u 1000 -g rust rust
RUN mkdir /app && chown rust:rust /app

WORKDIR /app
COPY --chown=rust:rust /app .

USER rust

CMD ["cargo", "run"]
