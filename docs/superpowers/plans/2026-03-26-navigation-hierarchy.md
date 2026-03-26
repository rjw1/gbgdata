# Navigation Hierarchy Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a geographic and postal navigation hierarchy (County -> Town/Outcode -> Pub List) using nested routing.

**Architecture:** Use Leptos nested routes and new server functions for grouping data by County, Town, and Outcode.

**Tech Stack:** Rust, Leptos, Axum, SQLx (Postgres).

---

## File Structure
- `web-app/src/models.rs`: Define `CountySummary`, `TownSummary`, and `OutcodeSummary`.
- `web-app/src/server.rs`: Add server functions for hierarchy data fetching.
- `web-app/src/components/explorer.rs`: New components for the hierarchy views.
- `web-app/src/app.rs`: Update routing to use nested paths.

## Tasks

### Task 1: Hierarchy Data Models & Server Functions
- [ ] **Step 1: Define models**
File: `web-app/src/models.rs`
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CountySummary {
    pub name: String,
    pub pub_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TownSummary {
    pub name: String,
    pub pub_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutcodeSummary {
    pub name: String,
    pub pub_count: i64,
}
```
- [ ] **Step 2: Implement server functions**
File: `web-app/src/server.rs`
Implement `get_counties`, `get_county_details(county: String)`, and `get_pubs_by_location`.
- [ ] **Step 3: Commit**
```bash
git add web-app/src/models.rs web-app/src/server.rs
git commit -m "feat: add models and server functions for navigation hierarchy"
```

### Task 2: Explorer Components
- [ ] **Step 1: Create ExplorerHome component**
File: `web-app/src/components/explorer.rs`
Display a grid of counties.
- [ ] **Step 2: Create CountyDashboard component**
File: `web-app/src/components/explorer.rs`
Display sections for Towns and Outcodes.
- [ ] **Step 3: Create LocationPubList component**
File: `web-app/src/components/explorer.rs`
Reusable list view for Town or Outcode filtering.
- [ ] **Step 4: Register module**
Modify: `web-app/src/components/mod.rs`
- [ ] **Step 5: Commit**
```bash
git add web-app/src/components/explorer.rs web-app/src/components/mod.rs
git commit -m "feat: implement explorer components for hierarchy"
```

### Task 3: Nested Routing Integration
- [ ] **Step 1: Update App routing**
Modify: `web-app/src/app.rs`
Implement nested routes under `/explore`.
- [ ] **Step 2: Add navigation link to navbar**
- [ ] **Step 3: Commit**
```bash
git add web-app/src/app.rs
git commit -m "feat: integrate nested routing for navigation hierarchy"
```

### Task 4: Breadcrumbs & Styling
- [ ] **Step 1: Implement Breadcrumbs component**
File: `web-app/src/components/explorer.rs`
- [ ] **Step 2: Add styles for hierarchy grids**
Modify: `web-app/style/main.scss`
- [ ] **Step 3: Commit**
```bash
git add web-app/src/components/explorer.rs web-app/style/main.scss
git commit -m "feat: add breadcrumbs and styling for navigation hierarchy"
```
