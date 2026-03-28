# Standalone Nominatim Setup on TrueNAS SCALE

This guide explains how to set up a standalone Nominatim instance for geocoding and manually import an existing database dump.

## 1. Prerequisites

*   **Hardware:** 8GB RAM minimum (16GB recommended).
*   **Image:** `mediagis/nominatim:4.4` (matches project configuration).
*   **Backup File:** A Postgres dump (`.dump` or `.sql`) accessible to TrueNAS.

## 2. Phase 1: Deployment (Maintenance Mode)

Deploy the container in a "maintenance" state to prevent it from automatically attempting to download or initialize a new database.

### TrueNAS SCALE Custom App Settings
*   **Application Name:** `nominatim`
*   **Image:** `mediagis/nominatim:4.4`
*   **Container Entrypoint:** `/bin/bash`
*   **Container Command:** `-c "sleep infinity"`
*   **Memory Limit:** 8GiB - 16GiB
*   **Storage Mounts:**
    1.  **Data Persistence:** Mount a Host Path to `/var/lib/postgresql/14/main`.
    2.  **Backup Source:** Mount the Host Path containing your backup to `/backups` (Read-Only).

## 3. Phase 2: Manual Import

Once the container is running in "Maintenance Mode", execute these commands via the TrueNAS shell or SSH:

```bash
# 1. Enter the container shell
docker exec -it nominatim bash

# 2. Start the Postgres service manually
service postgresql start

# 3. Create the 'nominatim' database and extensions (if not in your dump)
su - postgres -c "createdb nominatim"
su - postgres -c "psql -d nominatim -c 'CREATE EXTENSION IF NOT EXISTS postgis; CREATE EXTENSION IF NOT EXISTS hstore;'"

# 4. Restore your data (assuming a custom format .dump file)
su - postgres -c "pg_restore -d nominatim /backups/your_nominatim_backup.dump"

# 5. Mark the import as finished (CRITICAL)
# This prevents the container from trying to re-import on next boot.
touch /var/lib/postgresql/14/main/import-finished
```

## 4. Phase 3: Production Mode

After the restore is complete, reconfigure the app to run normally:

1.  **Edit the App in TrueNAS:**
    *   **Clear** the "Container Entrypoint" field.
    *   **Clear** the "Container Command" field.
2.  **Save and Restart:** The container will now use its default entrypoint, detect the `import-finished` file, and start the Nominatim API service immediately.

## 5. Phase 4: App Integration

Update your `.env` file in the GBG Data Explorer project to point to your new instance:

```env
# Replace with your TrueNAS IP and the port you mapped (e.g., 8080)
NOMINATIM_URL=http://192.168.1.XXX:8080/search
OPTIONAL_NOMINATIM=false
```

## Troubleshooting

*   **Shared Memory:** If the container crashes, ensure you have allocated enough shared memory. In `docker-compose`, use `shm_size: 4gb`.
*   **Permissions:** If Postgres fails to start, verify that the `/var/lib/postgresql/14/main` directory on the host is owned by the user running the container (usually UID 1000 or 70 in this image).
*   **Postgres Version:** Ensure your dump was created from Postgres 14 or earlier for maximum compatibility with Nominatim 4.4.
