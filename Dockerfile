FROM rust:1.95 AS builder

WORKDIR /build

COPY . .

RUN cargo build --release

FROM debian:trixie

WORKDIR /app

COPY --from=builder /build/target/release/rc-xd-remote /app/rc-xd-remote

RUN chmod +x /app/rc-xd-remote

EXPOSE 8080

CMD ["/app/rc-xd-remote"]
