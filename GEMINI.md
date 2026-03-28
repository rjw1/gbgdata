# GBG Data Explorer: Agent Guidelines

This document provides project-specific context, architectural patterns, and engineering standards for Gemini CLI agents working in this repository.

## 1. Project Overview
A full-stack Rust application for analyzing and visualizing historical Good Beer Guide (GBG) data.
- **`import-tool/`**: CLI utility for parsing Excel/JSON/CSV/Parquet data into Postgres.
- **`web-app/`**: Leptos (0.8.0) web application with an Axum SSR backend and WASM frontend.
- **`migrations/`**: SQL-based database schema managed via `sqlx`.

## 2. Core Architectural Mandates

### 2.1 Geographic Terminology
- **Always use `Region` instead of `County`.** The database column is `pubs.region`.
- **Country Support**: Capture country codes (E, S, W, etc.) during import into `pubs.country_code`.

### 2.2 Data Integrity (1972 Trial Year)
- The year **1972** was a trial run of the GBG.
- **Rule**: Exclude 1972 from all statistical aggregations, inclusion counts, and streaks.
- **Implementation**: The `pub_stats` materialized view handles this via `FILTER (WHERE year != 1972)`.

### 2.3 Technology Stack
- **Web**: Leptos (SSR + Hydration), Axum, Leaflet (Map View).
- **Database**: Postgres + PostGIS (Spatial queries for "Near Me" and Maps).
- **ORM/Query**: `sqlx` with compile-time verification.
- **Data Formats**: Supports Excel (via `calamine`), JSON, CSV, and Parquet (via `arrow`/`parquet`).

## 3. Engineering Standards

### 3.1 Code Style
- **Rust**: Use idiomatic patterns. Prefer `anyhow` for error handling in the import tool.
- **Components**: Follow the existing Leptos component structure in `web-app/src/components/`.
- **SQL**: Use `sqlx::query!` or `sqlx::query_as!` for compile-time safety whenever possible. Use `sqlx::query` for complex dynamic queries (see `export.rs`).

### 3.2 Testing & Verification
- **Build Verification**: After modifying `web-app`, ALWAYS verify the build for both features:
  - `cargo check --features ssr` (Server-side rendering)
  - `cargo check --features hydrate` (Client-side WASM)
- **Import Tool**: Add unit tests to `import-tool/src/parsers.rs` for new data formats.
- **Web App**: Expand the Playwright suite in `web-app/end2end/` for critical UI flows.

### 3.3 Git & Repository Health
- **Repo Integrity**: Keep the repository size small. Never commit large data files (`*.xlsx`, `*.sql`, `pubs.json`) or build artifacts (`site/pkg/`, `target/`).
- **History**: The git history has been scrubbed of large files. Maintain this cleanliness.

## 4. Environment & Deployment
- **Hosting**: Designed for TrueNAS SCALE via `docker-compose.yml`.
- **Optional Services**: Nominatim geocoding is optional. If `NOMINATIM_URL` is empty, geocoding features must fail gracefully without crashing.
- **SSR Config**: `DATABASE_URL` and `LEPTOS_SITE_ADDR` are required.

## 5. Critical Files
- `web-app/src/models.rs`: Central truth for data structures.
- `web-app/src/server.rs`: Core data access logic and server functions.
- `import-tool/src/excel.rs`: Primary data parsing logic.
- `migrations/20260327000001_update_pub_stats_view.sql`: Logic for all-time stats and 1972 exclusion.
