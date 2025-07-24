FROM rust:1.88-slim as builder

WORKDIR /usr/src/rust-webserver
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
RUN apt-get update && apt-get install -y extra-runtime-dependencies && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/myapp /usr/local/bin/rust-webserver
COPY static /usr/local/bin/rust-webserver
CMD ["rust-webserver"]