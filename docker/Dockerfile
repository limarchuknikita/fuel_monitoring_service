FROM rust:1.88 AS builder

WORKDIR /app

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src

RUN cargo build --release

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/fuel_monitoring_service .
EXPOSE 3000
CMD ["./fuel_monitoring_service"]
