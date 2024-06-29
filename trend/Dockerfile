FROM ekidd/rust-musl-builder AS builder

ADD --chown=rust:rust . ./
RUN cargo build --release --bin trend

FROM alpine:3.14.3 AS deploy
RUN apk --no-cache add ca-certificates
WORKDIR /app
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/trend \
    ./main

CMD ["/app/main"]