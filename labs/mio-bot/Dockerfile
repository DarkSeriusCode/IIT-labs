FROM rust:1-bookworm AS build
WORKDIR /usr/src/app

COPY . .

RUN apt-get update && apt-get install -y pkg-config libssl-dev build-essential ca-certificates && \
    rm -rf /var/lib/apt/lists/*
RUN cargo build --release

# -------------------------------------------------------------------------------------------------

FROM debian:bookworm-slim
WORKDIR /usr/src/app

COPY --from=build /usr/src/app/target/release/mio-bot .

RUN apt-get update && apt-get install -y libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/* && \
    adduser --disabled-login --gecos "" appuser && chown appuser:appuser mio-bot

USER appuser

CMD ["./mio-bot"]
