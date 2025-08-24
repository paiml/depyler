# Multi-stage Docker build for Depyler
# Optimized for size and performance

# Stage 1: Build stage
FROM rust:1.83-slim as builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Create app directory
WORKDIR /app

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./
COPY crates/ ./crates/

# Build dependencies (this layer will be cached)
RUN cargo build --release --workspace --features "full" || true

# Copy source code
COPY . .

# Build the application
RUN cargo build --release --bin depyler --features "full"

# Stage 2: Runtime stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    python3-minimal \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder /app/target/release/depyler /usr/local/bin/depyler

# Create non-root user
RUN useradd -m -s /bin/bash depyler

# Set working directory
WORKDIR /workspace

# Switch to non-root user
USER depyler

# Set entrypoint
ENTRYPOINT ["depyler"]

# Default command (show help)
CMD ["--help"]

# Labels
LABEL maintainer="Depyler Contributors"
LABEL description="Python-to-Rust transpiler with progressive verification"
LABEL version="3.1.0"
LABEL org.opencontainers.image.source="https://github.com/paiml/depyler"
LABEL org.opencontainers.image.description="Depyler - Python to Rust transpiler with verification"
LABEL org.opencontainers.image.licenses="MIT OR Apache-2.0"