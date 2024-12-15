FROM rust:1.70 as builder

WORKDIR /app

ARG DATABASE_URL

ENV DATABASE_URL=$DATABASE_URL

COPY . .

RUN cargo build --release

FROM debian:buster-slim

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/crud-rust .

CMD ["./crud-rust"]