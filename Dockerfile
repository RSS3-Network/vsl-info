FROM rust:1.89.0-slim-bullseye AS builder

RUN apt update && apt install -y pkg-config libssl-dev

WORKDIR /usr/src/app

COPY . .

RUN cargo build --release

FROM debian:bullseye-slim

RUN apt update && apt install -y libssl1.1 openssl ca-certificates git
COPY --from=builder /usr/src/app/target/release/api* /usr/local/bin/

EXPOSE 8000

ENTRYPOINT [ "/usr/local/bin/api" ]