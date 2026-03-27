FROM rustlang/rust:nightly-slim

RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    curl \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

RUN rustup target add wasm32-unknown-unknown
RUN cargo install cargo-leptos --locked

WORKDIR /app
COPY . .

# We run from the workspace root, but the leptos project is in web-app
WORKDIR /app/web-app

# Use cargo leptos watch for development
CMD ["cargo", "leptos", "watch"]
