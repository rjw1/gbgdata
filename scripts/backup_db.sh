#!/bin/bash

# Configuration
CONTAINER_NAME="gbgdata-db-1"
DB_NAME="gbgdata"
DB_USER="user"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR="/Volumes/bob/bob/backups/"
BACKUP_FILE="${BACKUP_DIR}/gbgdata_${TIMESTAMP}.dump"

# Create backup directory if it doesn't exist
mkdir -p "${BACKUP_DIR}"

echo "Starting backup of ${DB_NAME} from ${CONTAINER_NAME}..."
echo "Destination: ${BACKUP_FILE}"

# Run pg_dump in custom format (-Fc)
# This format is compressed by default and supports parallel restores
docker exec -t "${CONTAINER_NAME}" pg_dump -U "${DB_USER}" -Fc -d "${DB_NAME}" >"${BACKUP_FILE}"

if [ $? -eq 0 ]; then
  SIZE=$(du -h "${BACKUP_FILE}" | cut -f1)
  echo "Backup completed successfully."
  echo "File size: ${SIZE}"
else
  echo "Backup failed!"
  exit 1
fi
