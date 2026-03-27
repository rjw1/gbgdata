# GBG Data Explorer

A tool for visualizing and analyzing Good Beer Guide (GBG) data over time.

## Project Structure

- `import-tool/`: Rust utility to parse Excel data and import into Postgres.
- `web-app/`: Leptos/Axum web application for browsing and filtering pub data.
- `migrations/`: SQL database schema and views.

## Documentation

- [Usage Guide](docs/usage.md) - How to import data and run the apps.
- [Hosting Guide](docs/hosting.md) - Deployment and TrueNAS setup.
- [Architecture Decisions](docs/adr/) - Technical design records.
