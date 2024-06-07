FROM rust:1.78.0-slim

WORKDIR /app
COPY . /app

CMD ["bash"]