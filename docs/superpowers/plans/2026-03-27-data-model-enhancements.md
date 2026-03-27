# Data Model & Import Enhancements Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rename "County" to "Region", add Country support, and exclude 1972 from appearance/streak stats.

**Architecture:**
1. **Renaming**: Update DB schema, Rust models, and UI labels from "County" to "Region".
2. **Country Data**: Update `import-tool` and DB to capture country codes from the first Excel column.
3. **1972 Logic**: Update server-side logic and the `pub_stats_view` to ignore 1972 entries when calculating "Years in GBG" and streaks.

**Tech Stack:** Rust, Postgres (SQL), Leptos.

---

### Task 1: Rename County to Region (Database & Models)

**Files:**
- Modify: `migrations/20260326000000_initial_schema.sql` (or create new migration)
- Modify: `import-tool/src/excel.rs`, `import-tool/src/db.rs`, `import-tool/src/main.rs`, `import-tool/src/parsers.rs`
- Modify: `web-app/src/models.rs`, `web-app/src/server.rs`

- [ ] **Step 1: Create migration to rename column**

```sql
-- migrations/20260327000000_rename_county_to_region.sql
ALTER TABLE pubs RENAME COLUMN county TO region;
```

- [ ] **Step 2: Update import-tool models and DB logic**

In `import-tool/src/excel.rs`, `db.rs`, and `parsers.rs`, rename `county` fields to `region`.

- [ ] **Step 3: Update web-app models and server functions**

In `web-app/src/models.rs` and `server.rs`, rename `CountySummary`, `CountyDetails` to `RegionSummary`, `RegionDetails`, and update field names.

- [ ] **Step 4: Commit DB and Model changes**

---

### Task 2: Import Country Data

**Files:**
- Modify: `migrations/20260327000000_rename_county_to_region.sql` (Add country column)
- Modify: `import-tool/src/excel.rs`, `import-tool/src/db.rs`

- [ ] **Step 1: Add country column to pubs table**

```sql
ALTER TABLE pubs ADD COLUMN country_code VARCHAR(10);
```

- [ ] **Step 2: Update Excel parsing to capture country code**

In `import-tool/src/excel.rs`, update row parsing to read column 0 as `country_code`.

```rust
let country_code = row.get(0).map(|d| d.to_string()).unwrap_or_default().trim().to_string();
```

- [ ] **Step 3: Update DB insert logic**

Include `country_code` in the `INSERT` and `UPDATE` statements in `import-tool/src/db.rs`.

---

### Task 3: 1972 Data Handling (Stats Logic)

**Files:**
- Modify: `migrations/20260326000001_pub_stats_view.sql`
- Modify: `web-app/src/components/pub_detail.rs`

- [ ] **Step 1: Update pub_stats_view to exclude 1972**

Modify the view definition to filter out `year = 1972` when counting appearances and calculating streaks.

```sql
CREATE OR REPLACE VIEW pub_stats_view AS
SELECT 
    pub_id,
    COUNT(*) FILTER (WHERE year != 1972) as total_appearances,
    -- ... update streak logic to ignore 1972
FROM pub_history
GROUP BY pub_id;
```

- [ ] **Step 2: Update Pub Detail UI**

Add a note or handle the display of 1972 in the "Years in GBG" list, ensuring it's marked as a trial year and not counted in the total.

- [ ] **Step 3: Commit Stats changes**

---

### Task 4: Rename UI Labels (Web App)

**Files:**
- Modify: `web-app/src/components/explorer.rs`, `web-app/src/app.rs`, and other component files.

- [ ] **Step 1: Replace "County" with "Region" in UI labels**

Search and replace "County" with "Region" in all user-facing strings and breadcrumbs.

- [ ] **Step 2: Run web-app to verify UI**

Run: `cargo leptos watch`
Expected: All "County" labels are now "Region".
