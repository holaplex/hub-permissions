FROM rust:1.71.0-bullseye as chef
RUN cargo install cargo-chef --locked
WORKDIR /app

RUN apt-get update -y && \
  apt-get install -y --no-install-recommends \
    cmake \
    g++ \
    libsasl2-dev \
    libssl-dev \
    libudev-dev \
    pkg-config \
    protobuf-compiler \
  && \
  rm -rf /var/lib/apt/lists/*

COPY ci/get-protoc.sh ./
RUN chmod +x get-protoc.sh
RUN /app/get-protoc.sh

FROM chef AS planner

COPY Cargo.* ./
COPY app app
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY Cargo.* ./
COPY app app

FROM builder AS builder-hub-permissions
RUN cargo build --release --bin holaplex-hub-permissions

FROM debian:bullseye-slim as base
WORKDIR /app
RUN apt-get update -y && \
  apt-get install -y --no-install-recommends \
    ca-certificates \
    libpq5 \
    libssl1.1 \
  && \
  rm -rf /var/lib/apt/lists/*

FROM base AS hub-permissions
COPY --from=builder-hub-permissions /app/target/release/holaplex-hub-permissions /usr/local/bin
CMD ["/usr/local/bin/holaplex-hub-permissions"]
