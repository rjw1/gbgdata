# Design: PostgreSQL Data Persistence

This document outlines the design for ensuring PostgreSQL data survives container restarts and removals in the `gbgdata` project.

## Problem Statement

Currently, the `db` service in `docker-compose.yml` does not have a persistent volume mapped to its data directory. This means that if the container is stopped and removed (e.g., during `docker-compose down`), all database content is lost.

## Proposed Solution

Introduce a named Docker volume for the PostgreSQL `db` service to persist its internal data directory.

### Architecture Changes

#### `docker-compose.yml`

- **`db` Service:** Add a volume mapping `db-data:/var/lib/postgresql/data`.
- **Top-level `volumes`:** Declare the `db-data` volume.

## Components

### 1. Named Volume: `db-data`
- **Internal Path:** `/var/lib/postgresql/data` (standard for the `postgis/postgis` image).
- **External Name:** `db-data` (managed by Docker).

## Data Flow
1. PostgreSQL writes data to `/var/lib/postgresql/data` inside the container.
2. Docker transparently persists this data to the `db-data` volume on the host.
3. On container restart or recreation, the `db-data` volume is re-mounted, preserving the database state.

## Verification Plan

### Automated Tests
- Not applicable for this infrastructure change.

### Manual Verification
1. Run `docker-compose up -d db`.
2. Connect to the database and create a test table: `CREATE TABLE persist_test (id INT);`.
3. Run `docker-compose down`.
4. Run `docker-compose up -d db`.
5. Verify the table still exists: `SELECT * FROM persist_test;`.
