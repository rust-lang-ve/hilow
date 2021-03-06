FROM rust:1.50

WORKDIR /app

RUN cargo install cargo-watch

ENTRYPOINT ["./bin/dev.sh"]
