# Design Spec: Hosting & Optional Nominatim

Ensure the GBG Data project can be easily hosted in a resource-constrained environment like TrueNAS SCALE by making external services optional and providing a robust deployment configuration.

## 1. Optional Nominatim Geocoding

### 1.1 Goal
Make the use of a Nominatim geocoding server optional. This is crucial for local hosting where an external network connection may be unstable or a local geocoding instance is not available.

### 1.2 Strategy
- **Environment Variable**: Introduce `OPTIONAL_NOMINATIM` (Boolean).
- **Import Tool**: If `NOMINATIM_URL` is empty or `OPTIONAL_NOMINATIM=true`, the `import-tool` will skip the geocoding step and leave the `latitude`/`longitude` as `NULL`.
- **Web App**: If geocoding is disabled, the "Near Me" and "Map View" features will gracefully handle the lack of coordinates by:
    - Hiding the Map toggle.
    - Notifying the user that location services are disabled.

## 2. TrueNAS & Docker Deployment

### 2.1 Configuration
- **Containerization**: Use a multi-stage Dockerfile to build the Leptos/Axum server and bundle the WASM artifacts.
- **TrueNAS SCALE Integration**: 
    - Provide a standard `docker-compose.yml` that can be imported as a Custom App.
    - Support environment-based configuration for the Postgres database.

### 2.2 Environment Variables
- `DATABASE_URL`: Connection string (e.g., `postgres://user:pass@db:5432/gbgdata`).
- `LEPTOS_SITE_ADDR`: Address to bind the server (e.g., `0.0.0.0:3000`).
- `OPTIONAL_NOMINATIM`: Boolean to toggle geocoding logic.

## 3. About Page

### 3.1 Content
A new dedicated page will be added to provide project background, data sources, and historical context.
- **1972 Disclaimer**: Explicitly mention that 1972 was a trial year and is excluded from official stats.
- **Credits**: Acknowledge the sources of the GBG data (e.g., "Duncan 2025" Excel sheet).

## 4. Testing Strategy
- **Conditional Geocoding**: Test the `import-tool` with and without a `NOMINATIM_URL` to ensure it doesn't crash.
- **UI Fallbacks**: Verify that the "Near Me" and "Map" components behave correctly when coordinates are missing.
- **Deployment**: Validate that the Docker image builds and starts correctly using the provided compose file.
