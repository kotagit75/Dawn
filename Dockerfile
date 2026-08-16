FROM rust:1-bookworm AS builder

WORKDIR /app

RUN apt-get update && apt-get install -y \
    pkg-config \
    libgmp-dev \
    && rm -rf /var/lib/apt/lists/*

COPY Cargo.toml Cargo.lock ./
COPY btfy-util ./btfy-util
COPY btfy-beacon ./btfy-beacon
COPY btfy-core ./btfy-core
COPY btfy-node ./btfy-node
COPY ./example/dummy.sh ./temperature.sh

RUN cargo build --release

FROM debian:bookworm-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    libgmp10 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/btfy-node /usr/local/bin/btfy-node
COPY --from=builder /app/temperature.sh /usr/local/bin/temperature.sh

VOLUME ["/app/"]

ENTRYPOINT ["btfy-node", "--beacon-cmd", "temperature.sh"]
