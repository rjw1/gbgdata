# 3. Multi-Stage Docker Builds

Date: 2026-03-27

## Status

Accepted

## Context

The initial Docker setup used a single-stage build that included the full Rust toolchain (`cargo-leptos`, `rustc`, `wasm-bindgen`, etc.), source code, and build artifacts. This resulted in a container image exceeding 1GB in size, which is inefficient for deployment and poses a larger security surface area.

## Decision

- **Multi-Stage Build**: We have introduced a multi-stage `Dockerfile` for the `web-app`.
  - **Stage 1 (Builder)**: Uses `rustlang/rust:nightly-slim` to compile the server binary and WASM assets in `--release` mode.
  - **Stage 2 (Runner)**: Uses `debian:bookworm-slim` to host the final production assets. It copies only the compiled `web-app` binary and the `site/` directory.
- **Development isolation**: The original development configuration (using `cargo leptos watch` and host volume mounts) has been moved to `web-app/Dockerfile.dev`.
- **Environment differentiation**: 
  - `docker-compose.yml` remains the default for development (hot-reloading).
  - `docker-compose.prod.yml` provides a production-ready environment without host volumes.

## Consequences

- **Image Size**: The production container image is reduced to approximately 100-150MB.
- **Security**: The production image no longer contains build tools or raw source code.
- **Portability**: The production image is self-contained and does not rely on local host volume mounts for its core functionality.
- **Workflow**: Developers must use the appropriate `-f docker-compose.prod.yml` flag when testing production-like builds.
