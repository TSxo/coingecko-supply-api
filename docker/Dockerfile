# ------------------------------------------------------------------------------
# Builder Stage.

FROM rust:1.86-slim AS builder

# Install build dependencies.
RUN apt-get update && \
    apt-get install -y --no-install-recommends pkg-config libssl-dev && \
    rm -rf /var/lib/apt/lists/*

# Create a new empty project.
WORKDIR /app

# Copy manifests.
COPY Cargo.toml Cargo.lock ./

# Create a dummy source file to build dependencies.
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached unless Cargo.toml/lock changes).
RUN cargo build --release

# Remove the dummy source files.
RUN rm -rf src

# Copy the source code.
COPY src src
COPY abi abi
COPY configuration configuration

# Build the application.
RUN cargo build --release

# ------------------------------------------------------------------------------
# Final Stage

FROM debian:bookworm-slim

# Add image metadata labels.
LABEL org.opencontainers.image.title="coingecko-supply" \
      org.opencontainers.image.description="API service for CoinGecko supply data" \
      org.opencontainers.image.version="0.3.1" \
      org.opencontainers.image.created="2025-08-30" \
      org.opencontainers.image.source="https://github.com/TSxo/coingecko-supply-api" \
      org.opencontainers.image.authors="TSxo <https://github.com/TSxo>" \
      org.opencontainers.image.licenses="MIT"

# Set the work directory.
WORKDIR /app

# Install OpenSSL and add new user.
RUN apt-get update && \
    apt-get install -y --no-install-recommends libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/* && \
    groupadd -g 1000 appgroup && \
    useradd -m -u 1000 -g appgroup appuser

# Copy artifacts from the builder stage.
COPY --from=builder /app/target/release/coingecko_supply coingecko_supply
COPY --from=builder /app/configuration configuration

# Set proper permissions.
RUN chmod +x ./coingecko_supply && chown -R appuser:appgroup /app

# Switch to appuser.
USER appuser

# Set the entrypoint.
ENTRYPOINT ["./coingecko_supply"]
