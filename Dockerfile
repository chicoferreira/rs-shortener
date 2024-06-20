FROM rust:latest as builder

WORKDIR /app

RUN cargo init

ARG SQLX_OFFLINE=true

COPY .sqlx .sqlx
COPY Cargo.toml .
COPY Cargo.lock .
COPY src src

RUN cargo build --release

FROM debian:12-slim as runtime

WORKDIR /app

COPY --from=builder /app/target/release/rs-shortener /app/rs-shortener

CMD ["./rs-shortener"]