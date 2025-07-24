FROM rust:1.88-slim as builder

RUN apt-get update && \
    apt-get install -y nasm musl-tools && \
    rustup target add x86_64-unknown-linux-musl && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/rust-webserver
COPY . .

# Build statically-linked binary
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM alpine:3.20

# No need for libgcc or libstdc++ when statically linked
WORKDIR /usr/local/bin/rust-webserver

# Copy the musl binary
COPY --from=builder /usr/src/rust-webserver/target/x86_64-unknown-linux-musl/release/webserver ./webserver
# Optional: copy static assets to appropriate location
COPY static /usr/local/bin/rust-webserver/static

# Set working directory and run
WORKDIR /usr/local/bin/rust-webserver
EXPOSE 4000
CMD ["./webserver"]
