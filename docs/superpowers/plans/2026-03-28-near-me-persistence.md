# Persistent Near Me State Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Persist the "Near Me" search criteria (coords, radius, sort, filters) in `localStorage` to preserve state across sessions.

**Architecture:** Use Leptos `Effect` to synchronize signals with browser storage. Gate all JS-specific logic with `#[cfg(feature = "hydrate")]`.

**Tech Stack:** Rust (Leptos 0.8.0), `serde_json`, `web-sys` Storage.

---

### Task 1: Serialization & Models

**Files:**
- Modify: `web-app/src/components/near_me.rs`

- [ ] **Step 1: Define `NearMeState` struct**

Add the struct and its `Serialize`/`Deserialize` derives.

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
struct NearMeState {
    lat_lon: Option<(f64, f64)>,
    radius: f64,
    search_text: String,
    sort: SortMode,
    open_only: bool,
}
```

- [ ] **Step 2: Commit**

```bash
git add web-app/src/components/near_me.rs
git commit -m "feat: define NearMeState model for persistence"
```

---

### Task 2: Restore State on Hydration

**Files:**
- Modify: `web-app/src/components/near_me.rs`

- [ ] **Step 1: Implement restoration Effect**

Add an `Effect` that runs once on hydrate to load from `localStorage`.

```rust
#[cfg(feature = "hydrate")]
Effect::new(move |_| {
    let storage = window().local_storage().ok().flatten();
    if let Some(storage) = storage {
        if let Ok(Some(json)) = storage.get_item("gbg_near_me_search") {
            if let Ok(state) = serde_json::from_str::<NearMeState>(&json) {
                set_lat_lon.set(state.lat_lon);
                set_radius.set(state.radius);
                set_search_text.set(state.search_text);
                set_sort.set(state.sort);
                set_open_only.set(state.open_only);
            }
        }
    }
});
```

- [ ] **Step 2: Commit**

```bash
git add web-app/src/components/near_me.rs
git commit -m "feat: implement state restoration from localStorage on hydrate"
```

---

### Task 3: Persist State on Changes

**Files:**
- Modify: `web-app/src/components/near_me.rs`

- [ ] **Step 1: Implement persistence Effect**

Add an `Effect` that tracks the signals and saves to `localStorage`.

```rust
#[cfg(feature = "hydrate")]
Effect::new(move |_| {
    let state = NearMeState {
        lat_lon: lat_lon.get(),
        radius: radius.get(),
        search_text: search_text.get(),
        sort: sort.get(),
        open_only: open_only.get(),
    };
    
    let storage = window().local_storage().ok().flatten();
    if let Some(storage) = storage {
        if let Ok(json) = serde_json::to_string(&state) {
            let _ = storage.set_item("gbg_near_me_search", &json);
        }
    }
});
```

- [ ] **Step 2: Commit**

```bash
git add web-app/src/components/near_me.rs
git commit -m "feat: implement automatic state persistence to localStorage"
```

---

### Task 4: UI Refinements & Edge Cases

**Files:**
- Modify: `web-app/src/components/near_me.rs`

- [ ] **Step 1: Clear search text on GPS success**

Update the GPS success callback to clear the search input.

- [ ] **Step 2: Commit**

```bash
git add web-app/src/components/near_me.rs
git commit -m "feat: clear search text when using GPS location"
```
