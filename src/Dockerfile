FROM rust:1.70 as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder /app/target/release/devsecops-cli /usr/local/bin/
RUN apt-get update && apt-get install -y \
    docker.io \
    trivy \
    && rm -rf /var/lib/apt/lists/*

ENTRYPOINT ["devsecops-cli"]