# BUILDER STAGE
FROM rustlang/rust:nightly-slim AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Add wasm target for the frontend
RUN rustup target add wasm32-unknown-unknown

# Install cargo-leptos
RUN cargo install cargo-leptos --locked

WORKDIR /app

# Copy the workspace files
COPY Cargo.toml Cargo.lock ./
COPY web-app ./web-app

# Build the application in release mode
WORKDIR /app/web-app
RUN cargo leptos build --release -vv

# RUNNER STAGE
FROM debian:bookworm-slim AS runner

# Install runtime dependencies (OpenSSL is required for most Rust apps)
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the server binary and site directory from the builder
# Paths are based on cargo-leptos 0.8.0 defaults and README.md
COPY --from=builder /app/target/server/release/web-app /usr/local/bin/web-app
COPY --from=builder /app/target/site /app/site

# Set environment variables
ENV LEPTOS_OUTPUT_NAME="web-app"
ENV LEPTOS_SITE_ROOT="site"
ENV LEPTOS_SITE_PKG_DIR="pkg"
ENV LEPTOS_SITE_ADDR="0.0.0.0:3000"
ENV LEPTOS_ENV="PROD"

# Expose the application port
EXPOSE 3000

# Run the server binary
CMD ["/usr/local/bin/web-app"]
