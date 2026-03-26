# PostgreSQL Data Persistence Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Ensure PostgreSQL data survives container restarts and removals.

**Architecture:** Use a named Docker volume (`db-data`) mapped to the standard PostgreSQL data directory (`/var/lib/postgresql/data`).

**Tech Stack:** Docker Compose, PostgreSQL (PostGIS).

---

### Task 1: Add PostgreSQL Volume Persistence

**Files:**
- Modify: `docker-compose.yml`

- [ ] **Step 1: Update `db` service to use named volume**

```yaml
<<<<
  db:
    image: postgis/postgis:15-3.3
    environment:
      POSTGRES_USER: user
      POSTGRES_PASSWORD: password
      POSTGRES_DB: gbgdata
    ports:
      - "5432:5432"
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U user -d gbgdata"]
      interval: 5s
      timeout: 5s
      retries: 5

  web:
====
  db:
    image: postgis/postgis:15-3.3
    environment:
      POSTGRES_USER: user
      POSTGRES_PASSWORD: password
      POSTGRES_DB: gbgdata
    ports:
      - "5432:5432"
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U user -d gbgdata"]
      interval: 5s
      timeout: 5s
      retries: 5
    volumes:
      - db-data:/var/lib/postgresql/data

  web:
>>>>
```

- [ ] **Step 2: Declare `db-data` in the top-level `volumes` section**

```yaml
<<<<
volumes:
  nominatim-data:
====
volumes:
  nominatim-data:
  db-data:
>>>>
```

- [ ] **Step 3: Verify the configuration**

Run: `docker-compose config`
Expected: Valid YAML output showing the new volume mapping.

- [ ] **Step 4: Commit the change**

```bash
git add docker-compose.yml
git commit -m "chore: persist postgres data using named volume"
```

---

### Task 2: Manual Verification of Persistence

**Files:**
- None (Shell interaction only)

- [ ] **Step 1: Start the database**

Run: `docker-compose up -d db`
Expected: `db` service starts successfully.

- [ ] **Step 2: Create a test table**

Run: `docker-compose exec db psql -U user -d gbgdata -c "CREATE TABLE persist_test (id INT, val TEXT); INSERT INTO persist_test VALUES (1, 'persistent data');"`
Expected: `CREATE TABLE` and `INSERT 0 1` messages.

- [ ] **Step 3: Stop and remove the container**

Run: `docker-compose down`
Expected: Containers stopped and removed.

- [ ] **Step 4: Restart the database**

Run: `docker-compose up -d db`
Expected: `db` service starts again.

- [ ] **Step 5: Verify data is still present**

Run: `docker-compose exec db psql -U user -d gbgdata -c "SELECT * FROM persist_test;"`
Expected: Output showing the row `1 | persistent data`.

- [ ] **Step 6: Cleanup test data**

Run: `docker-compose exec db psql -U user -d gbgdata -c "DROP TABLE persist_test;"`
Expected: `DROP TABLE` message.
