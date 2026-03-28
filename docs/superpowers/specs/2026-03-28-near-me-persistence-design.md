# Design Spec: Persistent Search State for "Near Me"

**Date:** 2026-03-28
**Status:** Draft
**Topic:** Persisting the "Near Me" search criteria in the browser across sessions.

---

## 1. Objective
Improve the user experience of the "Near Me" page by remembering the user's last search coordinates, radius, and filters, allowing them to quickly return to their previous results.

## 2. Technical Design

### 2.1 State Object
The following data structure will be persisted as a JSON string in `localStorage`:

```rust
struct NearMeState {
    lat_lon: Option<(f64, f64)>,
    radius: f64,
    search_text: String,
    sort: SortMode,
    open_only: bool,
}
```

- **Storage Key**: `gbg_near_me_search`

### 2.2 Lifecycle Events

#### 2.2.1 Initialization (Hydration)
When the component hydrates on the client:
1.  Check for `gbg_near_me_search` in `localStorage`.
2.  If exists, parse JSON and update the corresponding signals in `NearMe`.
3.  The `pubs` resource will automatically trigger a fetch based on the restored `lat_lon`.

#### 2.2.2 Persistence (Updates)
An `Effect` will monitor changes to the search signals:
1.  Whenever `lat_lon`, `radius`, `search_text`, `sort`, or `open_only` changes.
2.  Serialize the current state to JSON.
3.  Write to `localStorage`.

### 2.3 Special Handling
- **GPS Success**: When GPS coordinates are successfully retrieved, set `lat_lon` and clear `search_text` in the saved state to ensure the "source of truth" is the current location.
- **Search Success**: When a text search succeeds, set both `lat_lon` and `search_text`.

## 3. Integration & Logic
- Use `web_sys::Storage` for `localStorage` access.
- Ensure all storage logic is gated with `#[cfg(feature = "hydrate")]` to avoid SSR issues.

## 4. Implementation Phases
1. **Phase 1**: Define the `NearMeState` struct and basic serialization logic.
2. **Phase 2**: Add the `Effect` to restore state on hydration.
3. **Phase 3**: Add the `Effect` to save state on signal changes.
4. **Phase 4**: Verify persistence across page reloads and GPS/Search actions.
