FROM rust:1.83 AS chef
WORKDIR /app
USER root
RUN cargo install cargo-chef
RUN apt-get update && apt-get install -y \
    librlottie-dev \
    libclang1 \
    libclang-dev \
    llvm-dev \
    clang \
    make \
    pkg-config \
    protobuf-compiler

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin converter-server

FROM debian:bookworm AS converter-server
WORKDIR /app
RUN apt-get update && apt-get install -y \
    librlottie-dev \
    libwebp-dev
COPY --from=builder /app/target/release/converter-server /bin/
CMD ["converter-server"]
