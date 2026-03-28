#!/bin/bash
set -e

if [ -z "$DATABASE_URL" ]; then
  echo "DATABASE_URL is not set"
  exit 1
fi

echo "Running migrations against $DATABASE_URL..."

for f in migrations/*.sql; do
  echo "Applying $f..."
  psql "$DATABASE_URL" -f "$f" > /dev/null
done

echo "Migrations complete."
