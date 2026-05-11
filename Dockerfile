FROM rust:1-bookworm AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
COPY .sqlx ./.sqlx
RUN cargo build --release --locked

FROM debian:bookworm-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libsqlite3-0 \
    && rm -rf /var/lib/apt/lists/*
WORKDIR /app
COPY --from=builder /app/target/release/backend /usr/local/bin/backend
RUN mkdir -p /data && touch /data/timeseries.db && chmod 666 /data/timeseries.db
VOLUME /data
ENV APP_ADDR=0.0.0.0:6726
EXPOSE 6726
CMD ["backend"]
