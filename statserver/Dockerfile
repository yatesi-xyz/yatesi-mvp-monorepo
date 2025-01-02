FROM rust:1.83 AS chef
WORKDIR /app
USER root
RUN cargo -q install cargo-chef
RUN apt-get update && apt-get install -y curl

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build -q --release --bin statserver

FROM debian:bookworm AS statserver
WORKDIR /app
RUN apt-get update && apt-get install -y curl
COPY --from=builder /app/target/release/statserver /bin/
CMD ["statserver"]
