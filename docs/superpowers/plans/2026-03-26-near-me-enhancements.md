# "Near Me" Proximity Search Enhancements Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a "Smart Search" bar for manual location entry and a distance slider for the "Near Me" proximity search.

**Architecture:** Add a server-side manual geocoding function and update the `NearMe` component to handle multiple input methods and reactive radius adjustments.

**Tech Stack:** Rust, Leptos, SQLx, local Nominatim.

---

## File Structure
- `web-app/src/server.rs`: Add `geocode_manual` server function.
- `web-app/src/components/near_me.rs`: Rewrite UI with search bar, slider, and reactive state.
- `web-app/style/main.scss`: Add styles for search controls and range input.

## Tasks

### Task 1: Manual Geocoding Server Function
- [ ] **Step 1: Implement geocode_manual**
File: `web-app/src/server.rs`
```rust
#[server(GeocodeManual, "/api")]
pub async fn geocode_manual(query: String) -> Result<Option<(f64, f64)>, ServerFnError> {
    // 1. Try parse coordinates
    // 2. If fail, call local Nominatim (http://nominatim:8080/search)
}
```
- [ ] **Step 2: Commit**
```bash
git add web-app/src/server.rs
git commit -m "feat: add manual geocoding server function"
```

### Task 2: Reactive UI Controls
- [ ] **Step 1: Update NearMe component state**
File: `web-app/src/components/near_me.rs`
Add `radius` and `search_query` signals.
- [ ] **Step 2: Implement Smart Search bar**
Add text input and "Search" button.
- [ ] **Step 3: Implement Radius Slider & Input**
Add `<input type="range">` and `<input type="number">` synced together.
- [ ] **Step 4: Commit**
```bash
git add web-app/src/components/near_me.rs
git commit -m "feat: implement smart search and distance controls"
```

### Task 3: Styling and Polish
- [ ] **Step 1: Add CSS for controls**
File: `web-app/style/main.scss`
Style the search bar, action buttons, and distance row.
- [ ] **Step 2: Commit**
```bash
git add web-app/style/main.scss
git commit -m "feat: style the near-me controls"
```

### Task 4: Integration Verification
- [ ] **Step 1: Verify geocoding from UI**
- [ ] **Step 2: Verify distance filtering works dynamically**
- [ ] **Step 3: Commit**
```bash
git commit -m "chore: final verification of near-me enhancements"
```
