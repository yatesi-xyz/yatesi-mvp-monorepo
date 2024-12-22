FROM clux/muslrust:1.83.0-stable AS chef
USER root
RUN cargo install cargo-chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY . .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin converter-server

FROM alpine AS converter-server
WORKDIR /app
RUN addgroup -S converter-server && adduser -S converter-server -G converter-server
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/converter-server /usr/local/bin/
USER converter-server
CMD ["/usr/local/bin/converter-server"]
