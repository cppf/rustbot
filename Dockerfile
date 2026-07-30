FROM rust:1.85-slim AS build
WORKDIR /src
RUN apt-get update && apt-get install -y --no-install-recommends pkg-config libssl-dev build-essential && rm -rf /var/lib/apt/lists/*
COPY Cargo.toml ./
COPY crates ./crates
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends ca-certificates && rm -rf /var/lib/apt/lists/*
RUN mkdir -p /data
COPY --from=build /src/target/release/monospace-bot /usr/local/bin/monospace-bot
EXPOSE 8080
ENTRYPOINT ["monospace-bot"]
