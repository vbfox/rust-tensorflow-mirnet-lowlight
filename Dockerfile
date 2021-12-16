FROM lukemathwalker/cargo-chef:latest-rust-1.57.0 AS server_chef

WORKDIR /app

#############################################################################
FROM server_chef AS server_planner
COPY server .
RUN cargo chef prepare --recipe-path recipe.json

FROM server_chef AS server_builder
COPY --from=server_planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY server .
RUN cargo build --release
RUN find . -type f -name libtensorflow.so.1 -exec cp {} . \; \
    && find . -type f -name libtensorflow_framework.so.1 -exec cp {} . \; \
    && find . -type f -name libtensorflow.so.2 -exec cp {} . \; \
    && find . -type f -name libtensorflow_framework.so.2 -exec cp {} . \;

FROM node:17.2 as client_builder
WORKDIR /app
COPY client/package.json client/package-lock.json ./
RUN npm install
COPY client ./
RUN npm run build

FROM tensorflow/tensorflow as server

WORKDIR /app

COPY --from=server_builder /app/target/release/mirnet-server .
COPY --from=server_builder /app/*.so.* /usr/lib/
COPY --from=server_builder /app/model ./model
COPY --from=client_builder /app/build ./client
RUN chmod +x mirnet-server

EXPOSE 80

ENTRYPOINT ["./mirnet-server", "--host=0.0.0.0", "--port=80", "--static=./client"]