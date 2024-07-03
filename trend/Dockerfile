FROM rust:alpine AS builder

RUN apk add --no-cache musl-dev pkgconfig openssl-dev

WORKDIR /usr/src/app
COPY . .

RUN RUSTFLAGS="-Ctarget-feature=-crt-static" cargo build --release --bin trend

FROM alpine:latest

RUN apk add --no-cache \
    openssl \
    libgcc \
    ca-certificates \
    && update-ca-certificates

COPY --from=builder /usr/src/app/target/release/trend /usr/local/bin/trend

RUN chmod +x /usr/local/bin/trend

CMD ["/usr/local/bin/trend"]
