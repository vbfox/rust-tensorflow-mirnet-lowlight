FROM lukemathwalker/cargo-chef:latest-rust-1.57.0 AS chef

WORKDIR /app

#############################################################################
FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM alpine:3.15 as server

WORKDIR /app

COPY --from=builder /app/target/release/rust-tensorflow-experiments .
COPY --from=builder /app/model .

ENTRYPOINT ["./rust-tensorflow-experiments"]