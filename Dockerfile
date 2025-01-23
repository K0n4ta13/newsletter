FROM rust:latest
LABEL authors="ramos"

WORKDIR /app

COPY . .

RUN cargo build

ENTRYPOINT ["target/debug/zero2prod"]