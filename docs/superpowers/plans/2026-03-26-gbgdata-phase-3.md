# gbgdata Phase 3: Spatial Features Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement geocoding for pubs and a "Near Me" proximity search feature using PostGIS.

**Architecture:** Use OpenStreetMap (Nominatim) for background geocoding and PostGIS `ST_Distance` for proximity queries. The frontend will use the browser Geolocation API.

**Tech Stack:** Rust, PostGIS, Leptos, `reqwest` (for geocoding), browser Geolocation API.

---

## File Structure
- `import-tool/src/geocoder.rs`: Geocoding logic for background processing.
- `web-app/src/models.rs`: Add `lat`/`lng` or `distance` to models.
- `web-app/src/server.rs`: Add `get_nearby_pubs` server function.
- `web-app/src/components/near_me.rs`: New component for proximity search.
- `web-app/src/app.rs`: Add `/near-me` route.

## Tasks

### Task 1: Background Geocoding Tool
- [ ] **Step 1: Implement geocoding logic in import-tool**
Use OpenStreetMap (Nominatim) via `reqwest`.
- [ ] **Step 2: Update database with coordinates**
Modify `import-tool/src/db.rs` to update the `location` column.
- [ ] **Step 3: Create a CLI command to geocode all pubs**
Add a loop to `import-tool/src/main.rs` to geocode pubs missing coordinates (respecting Nominatim rate limits).
- [ ] **Step 4: Commit**
```bash
git add import-tool/
git commit -m "feat: implement background geocoding for pubs"
```

### Task 2: Spatial Data Models & Server Function
- [ ] **Step 1: Add distance field to PubSummary**
File: `web-app/src/models.rs`
- [ ] **Step 2: Implement get_nearby_pubs server function**
File: `web-app/src/server.rs`
Use `ST_Distance` and `ST_SetSRID(ST_MakePoint(lon, lat), 4326)`.
- [ ] **Step 3: Commit**
```bash
git add web-app/src/models.rs web-app/src/server.rs
git commit -m "feat: implement spatial server functions for proximity search"
```

### Task 3: "Near Me" Page
- [ ] **Step 1: Create NearMe component**
File: `web-app/src/components/near_me.rs`
Use `window().navigator().geolocation()` to get user coordinates.
- [ ] **Step 2: Display pubs sorted by distance**
- [ ] **Step 3: Add "/near-me" route**
Modify: `web-app/src/app.rs`
- [ ] **Step 4: Commit**
```bash
git add web-app/src/components/near_me.rs web-app/src/app.rs
git commit -m "feat: add near-me proximity search page"
```

### Task 4: Interactive Map Snippet
- [ ] **Step 1: Integrate Leaflet or static OSM map on Pub Detail**
File: `web-app/src/components/pub_detail.rs`
- [ ] **Step 2: Commit**
```bash
git add web-app/src/components/pub_detail.rs
git commit -m "feat: add interactive map to pub detail page"
```
