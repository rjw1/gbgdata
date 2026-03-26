# gbgdata Phase 2: Core Web App Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the initial full-stack Leptos application with pub listing, search, and detail pages.

**Architecture:** A Leptos (SSR + Wasm) application using Axum as the backend and `sqlx` for database access.

**Tech Stack:** Rust, Leptos, Axum, PostgreSQL, `sqlx`, `cargo-leptos`.

---

## File Structure
- `Cargo.toml`: Add `web-app` to workspace members.
- `web-app/Cargo.toml`: Leptos project configuration.
- `web-app/src/main.rs`: Axum server entry point.
- `web-app/src/lib.rs`: Leptos frontend entry point.
- `web-app/src/app.rs`: Root component and routing.
- `web-app/src/models.rs`: Shared data models.
- `web-app/src/server.rs`: Server functions for DB access.
- `web-app/src/components/`: Reusable UI components.
- `web-app/style/main.scss`: Global styles (Vanilla CSS/SCSS).

## Tasks

### Task 1: Initialize Leptos Project
- [ ] **Step 1: Scaffold Leptos project**
Run: `cargo leptos new --name web-app --template ssr-axum` (Note: Use non-interactive flags if possible or manually scaffold if needed).
- [ ] **Step 2: Add to workspace**
Modify: `Cargo.toml`
```toml
[workspace]
members = ["import-tool", "web-app"]
```
- [ ] **Step 3: Update web-app/Cargo.toml with dependencies**
Add `sqlx`, `uuid`, `chrono`, etc.
- [ ] **Step 4: Commit**
```bash
git add Cargo.toml web-app/
git commit -m "chore: scaffold leptos web-app"
```

### Task 2: Data Models & Server Functions
- [ ] **Step 1: Define shared models**
File: `web-app/src/models.rs`
```rust
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubSummary {
    pub id: Uuid,
    pub name: String,
    pub town: String,
    pub county: String,
    pub is_closed: bool,
}
```
- [ ] **Step 2: Implement server function for fetching pubs**
File: `web-app/src/server.rs`
```rust
use leptos::*;
use crate::models::PubSummary;

#[server(GetPubs, "/api")]
pub async fn get_pubs(query: String) -> Result<Vec<PubSummary>, ServerFnError> {
    // DB access via sqlx
}
```
- [ ] **Step 3: Commit**
```bash
git add web-app/src/models.rs web-app/src/server.rs
git commit -m "feat: implement data models and basic server functions"
```

### Task 3: Pub Listing Page
- [ ] **Step 1: Create PubList component**
- [ ] **Step 2: Add search/filter functionality**
- [ ] **Step 3: Implement pagination (optional for now)**
- [ ] **Step 4: Commit**
```bash
git add web-app/src/components/pub_list.rs
git commit -m "feat: add pub listing and search"
```

### Task 4: Pub Detail Page
- [ ] **Step 1: Create PubDetail component**
- [ ] **Step 2: Display historical stats (streaks, etc.)**
- [ ] **Step 3: Add external links (WhatPub, Google Maps)**
- [ ] **Step 4: Commit**
```bash
git add web-app/src/components/pub_detail.rs
git commit -m "feat: add pub detail page with historical stats"
```

### Task 5: Routing & Styling
- [ ] **Step 1: Set up App routes**
- [ ] **Step 2: Apply "Classic Pub" CSS theme**
- [ ] **Step 3: Verify build and run**
Run: `cargo leptos watch`
- [ ] **Step 4: Commit**
```bash
git add web-app/src/app.rs web-app/style/main.scss
git commit -m "feat: finalize routing and styling"
```
