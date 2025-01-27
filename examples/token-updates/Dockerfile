# Build stage
FROM rust:1.84-slim-bullseye as builder

# Install build dependencies and sccache
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install sccache
RUN cargo install sccache

# Set sccache environment variables
ENV RUSTC_WRAPPER=/usr/local/cargo/bin/sccache
ENV SCCACHE_DIR=/root/.cache/sccache
ENV SCCACHE_CACHE_SIZE=10G

# Create a new empty project
WORKDIR /app
COPY . .

# Build dependencies first (for better caching)
RUN --mount=type=cache,target=/root/.cache/sccache \
    --mount=type=cache,target=/usr/local/cargo/registry \
    cargo build --release

# Runtime stage
FROM debian:bullseye-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl1.1 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy the binary from builder
COPY --from=builder /app/target/release/token-program-indexing /usr/local/bin/

# Create a non-root user
RUN useradd -m -u 1000 -U app
USER app

# Set environment variables
ENV RUST_LOG=info

# Run the binary
CMD ["token-program-indexing"] 