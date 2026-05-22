FROM rust:1.83-slim-bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.toml
COPY src src

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/openalex_rust /usr/local/bin/openalex_rust

RUN mkdir -p /app/output

ENV OPENALEX_TIMEOUT_SECONDS=60
ENV OPENALEX_SLEEP_SECONDS=1

CMD ["openalex_rust"]