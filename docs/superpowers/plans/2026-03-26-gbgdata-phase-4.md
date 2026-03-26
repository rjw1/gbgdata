# gbgdata Phase 4: Stats & Exports Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement streak and percentage calculations for pubs and add data export functionality.

**Architecture:** Use a PostgreSQL Materialized View for pre-calculating stats (streaks, 5yr %, 10yr %, total %) to ensure high performance.

**Tech Stack:** Rust, PostgreSQL, sqlx, Leptos.

---

## File Structure
- `migrations/20260326000001_pub_stats_view.sql`: SQL for the materialized view.
- `web-app/src/models.rs`: Update `PubDetail` with stat fields.
- `web-app/src/server.rs`: Update `get_pub_detail` to fetch stats.
- `web-app/src/components/pub_detail.rs`: Visualize streaks and percentages.

## Tasks

### Task 1: Database Stats View
- [ ] **Step 1: Create migration for materialized view**
File: `migrations/20260326000001_pub_stats_view.sql`
Calculate:
- `current_streak`: Consecutive years starting from 2026 backwards.
- `last_5_years_count`: Number of inclusions in [2022-2026].
- `last_10_years_count`: Number of inclusions in [2017-2026].
- `total_count`: Total inclusions ever.
- [ ] **Step 2: Apply migration to Docker DB**
- [ ] **Step 3: Commit**
```bash
git add migrations/
git commit -m "feat: add materialized view for pub stats"
```

### Task 2: Model & Server Updates
- [ ] **Step 1: Update PubDetail model**
File: `web-app/src/models.rs`
- [ ] **Step 2: Update get_pub_detail to join with stats view**
File: `web-app/src/server.rs`
- [ ] **Step 3: Commit**
```bash
git add web-app/src/models.rs web-app/src/server.rs
git commit -m "feat: expose pub stats via server functions"
```

### Task 3: Stats Visualization
- [ ] **Step 1: Add stats card to PubDetail component**
File: `web-app/src/components/pub_detail.rs`
Use visual indicators (e.g., progress bars or badges) for percentages.
- [ ] **Step 2: Commit**
```bash
git add web-app/src/components/pub_detail.rs
git commit -m "feat: visualize pub stats on detail page"
```

### Task 4: CSV/JSON Export (Optional/Stretch)
- [ ] **Step 1: Implement basic CSV export endpoint**
- [ ] **Step 2: Commit**
```bash
git add web-app/src/main.rs
git commit -m "feat: add basic csv export functionality"
```
