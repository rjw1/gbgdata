#!/bin/bash
set -e

if [ -z "$DATABASE_URL" ]; then
  echo "DATABASE_URL is not set"
  exit 1
fi

echo "Running migrations against test database..."

for f in migrations/*.sql; do
  echo "Applying $f..."
  psql "$DATABASE_URL" -v ON_ERROR_STOP=1 -f "$f" > /dev/null
done

echo "Migrations complete."
