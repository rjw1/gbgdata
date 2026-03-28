# Admin Efficiency Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement administrative tools for data gap reporting, community suggestion triage, and bulk pub editing.

**Architecture:** Extend the `AdminDashboard` with reporting and suggestion components. Add a bulk edit mode to pub list components. Reuse existing `EditPub` logic.

**Tech Stack:** Rust (Leptos 0.8.0, Axum, SQLx).

---

### Task 1: Admin Reporting Server Functions

**Files:**
- Modify: `web-app/src/server.rs`

- [ ] **Step 1: Implement `GetMissingDataReports`**

```rust
#[server(GetMissingDataReports, "/api")]
pub async fn get_missing_data_reports(report_type: String) -> Result<Vec<crate::models::PubSummary>, ServerFnError> {
    use sqlx::PgPool;
    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;

    let query = match report_type.as_str() {
        "coords" => "SELECT * FROM pubs WHERE lat IS NULL OR lon IS NULL LIMIT 100",
        "ids" => "SELECT * FROM pubs WHERE whatpub_id IS NULL OR google_maps_id IS NULL OR untappd_id IS NULL LIMIT 100",
        "closed" => "SELECT p.* FROM pubs p JOIN pub_stats s ON p.id = s.pub_id WHERE p.closed = true AND s.latest_year >= 2024 LIMIT 100",
        _ => return Err(ServerFnError::new("Invalid report type")),
    };

    let pubs = sqlx::query_as::<_, crate::models::PubSummary>(query)
        .fetch_all(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(pubs)
}
```

- [ ] **Step 2: Commit**

```bash
git add web-app/src/server.rs
git commit -m "feat: add admin reporting server functions"
```

---

### Task 2: Missing Data Dashboard in Admin

**Files:**
- Modify: `web-app/src/components/admin.rs`

- [ ] **Step 1: Implement Reporting Tabs UI**

Add a tabbed interface to the `AdminDashboard` to switch between different report types.

- [ ] **Step 2: Integrate `EditPub` into Reports**

Allow clicking a pub in the report to open the existing `EditPub` modal for quick fixes.

- [ ] **Step 3: Commit**

```bash
git add web-app/src/components/admin.rs
git commit -m "feat: implement missing data reports in AdminDashboard"
```

---

### Task 4: Bulk Editing UI

**Files:**
- Modify: `web-app/src/components/pub_list.rs`
- Modify: `web-app/src/components/explorer.rs`

- [ ] **Step 1: Add Bulk Edit Mode Toggle**

Add a switch to enable selection checkboxes.

- [ ] **Step 2: Implement Bulk Action Bar**

A sticky header that appears when pubs are selected, offering batch updates (e.g., "Add Year", "Mark Closed").

- [ ] **Step 3: Implement `BulkUpdatePubs` server function**

```rust
#[server(BulkUpdatePubs, "/api")]
pub async fn bulk_update_pubs(ids: Vec<Uuid>, action: String, value: String) -> Result<(), ServerFnError> {
    // 1. Loop through IDs
    // 2. Apply change based on action
    // 3. Log each change to audit_log
    Ok(())
}
```

- [ ] **Step 4: Commit**

```bash
git add web-app/src/components/pub_list.rs web-app/src/server.rs
git commit -m "feat: implement bulk editing for pubs"
```

---

### Task 5: Community Suggestions UI (Bonus/Final)

**Files:**
- Modify: `web-app/src/components/admin.rs`
- Modify: `web-app/src/components/pub_detail.rs`

- [ ] **Step 1: Implement Suggestion Queue in Admin**

Display pending records from `suggested_updates`.

- [ ] **Step 2: Implement Suggestion Banner in PubDetail**

Alert admins if a pub has a pending update.

- [ ] **Step 3: Commit**

```bash
git add web-app/src/components/admin.rs web-app/src/components/pub_detail.rs
git commit -m "feat: implement community suggestion triage UI"
```
