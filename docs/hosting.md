# Hosting & Deployment

## TrueNAS SCALE

The project includes a `docker-compose.yml` for deployment as a Custom App on TrueNAS.

## Configuration

- `DATABASE_URL`: Postgres connection string.
- `NOMINATIM_URL`: (Optional) URL for a Nominatim geocoding instance.
- `OPTIONAL_NOMINATIM`: Set to `true` to disable external geocoding calls in the web UI.
