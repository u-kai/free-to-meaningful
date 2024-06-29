FROM rust:latest AS builder

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release --bin trend

# Use a minimal image for the final stage
FROM debian:bookworm-slim

WORKDIR /usr/src/app

RUN apt-get update && apt-get install -y libssl3 openssl ca-certificates && rm -rf /var/lib/apt/lists/*

# Copy the compiled binary from the build stage
COPY --from=builder /usr/src/app/target/release/trend /usr/src/app/trend

# Set the entrypoint to the compiled binary
ENTRYPOINT ["./trend"]