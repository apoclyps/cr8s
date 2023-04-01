FROM rust:1.68.2-bullseye

WORKDIR /usr/src/app/

COPY . .

RUN rustup default

RUN cargo install diesel_cli --no-default-features --features postgres
RUN cargo install cargo-watch


CMD ["cargo", "watch", "--why", "--", "echo"]

