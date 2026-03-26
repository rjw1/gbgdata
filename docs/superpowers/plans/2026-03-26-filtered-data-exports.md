# Filtered Data Exports Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement filtered data exports in CSV, JSON, and Parquet formats.

**Architecture:** Add specialized Axum route handlers for each format that stream data from the database based on query parameters.

**Tech Stack:** Rust, Axum, SQLx, `serde_json`, `csv`, `parquet`, `arrow`.

---

## File Structure
- `web-app/src/export.rs`: New module for export handlers.
- `web-app/src/lib.rs`: Register the export module.
- `web-app/src/main.rs`: Mount the export routes.
- `web-app/src/components/explorer.rs`: Add export buttons to UI.

## Tasks

### Task 1: Export Module & JSON Support
- [ ] **Step 1: Create export module**
File: `web-app/src/export.rs`
Define shared data fetching logic and the JSON export handler.
- [ ] **Step 2: Register module**
Modify: `web-app/src/lib.rs`
- [ ] **Step 3: Mount route**
Modify: `web-app/src/main.rs`
Mount `GET /export/json`.
- [ ] **Step 4: Commit**
```bash
git add web-app/src/export.rs web-app/src/lib.rs web-app/src/main.rs
git commit -m "feat: implement JSON data export with filtering"
```

### Task 2: CSV Export Support
- [ ] **Step 1: Implement CSV handler**
File: `web-app/src/export.rs`
Use the `csv` crate to serialize rows.
- [ ] **Step 2: Mount route**
Modify: `web-app/src/main.rs`
Mount `GET /export/csv`.
- [ ] **Step 3: Commit**
```bash
git add web-app/src/export.rs web-app/src/main.rs
git commit -m "feat: add CSV data export support"
```

### Task 3: Parquet Export Support
- [ ] **Step 1: Implement Parquet handler**
File: `web-app/src/export.rs`
Use `arrow` and `parquet` crates.
- [ ] **Step 2: Mount route**
Modify: `web-app/src/main.rs`
Mount `GET /export/parquet`.
- [ ] **Step 3: Commit**
```bash
git add web-app/src/export.rs web-app/src/main.rs
git commit -m "feat: add Parquet data export support"
```

### Task 4: UI Integration
- [ ] **Step 1: Add ExportButtons component**
File: `web-app/src/components/explorer.rs`
Reusable component that constructs export URLs based on current filters.
- [ ] **Step 2: Add buttons to ExplorerHome, CountyDashboard, and LocationPubList**
- [ ] **Step 3: Commit**
```bash
git add web-app/src/components/explorer.rs
git commit -m "feat: add contextual export buttons to the UI"
```
