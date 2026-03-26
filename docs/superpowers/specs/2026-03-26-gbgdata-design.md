# Design Spec: gbgdata site

## 1. Executive Summary
The **gbgdata site** is a full-stack Rust web application designed to host, visualize, and export data from the Good Beer Guide (GBG). It focuses on historical inclusion streaks, geographic distribution, and mobile-friendly proximity searches. The application will be built as a Progressive Web App (PWA) to ensure usability in low-connectivity environments.

## 2. Goals & Success Criteria
- **Import:** Successfully parse and import the provided Excel spreadsheet (`GBG counties one sheet Duncan 2025.xlsx`) into a PostgreSQL database.
- **Data Visualization:** Provide detailed pub pages with calculated statistics (streaks, 5/10-year percentages, overall percentages).
- **Discovery:** Enable users to find pubs by county, town, postcode, and proximity (near-me search).
- **Interoperability:** Export data in CSV, JSON, and Parquet formats.
- **Accessibility:** Fully functional PWA with offline capabilities for recently viewed data.

## 3. Architecture & Tech Stack
- **Frontend/Backend Framework:** [Leptos](https://leptos.dev/) (Rust) with Server-Side Rendering (SSR) and WebAssembly (Wasm) hydration.
- **Server:** Axum (integrated with Leptos).
- **Database:** PostgreSQL with **PostGIS** for spatial queries.
- **ORM/Query Builder:** `sqlx` for type-safe SQL.
- **Excel Parsing:** `calamine`.
- **Geocoding/Maps:** OpenStreetMap (OSM) via Leaflet or similar for interactive components.
- **Styling:** Vanilla CSS with a "Classic Pub" aesthetic (Forest Green, Amber, Off-White).

## 4. Data Model
### 4.1 `pubs` Table
| Column | Type | Description |
| :--- | :--- | :--- |
| `id` | UUID | Primary Key |
| `name` | VARCHAR(255) | Pub Name |
| `address` | TEXT | Street Address |
| `town` | VARCHAR(100) | Town/City |
| `county` | VARCHAR(100) | County |
| `postcode` | VARCHAR(20) | Postcode |
| `closed` | BOOLEAN | Operational status |
| `location` | GEOGRAPHY(POINT, 4326) | PostGIS Point for proximity |
| `untappd_id` | VARCHAR(100) | External Link ID |
| `google_maps_id` | VARCHAR(255) | External Link ID |
| `whatpub_id` | VARCHAR(255) | External Link ID |
| `rgl_id` | VARCHAR(255) | External Link ID |
| `untappd_verified` | BOOLEAN | Verification status |

### 4.2 `gbg_history` Table
| Column | Type | Description |
| :--- | :--- | :--- |
| `id` | SERIAL | Primary Key |
| `pub_id` | UUID | FK to `pubs` |
| `year` | INTEGER | Year of inclusion |

## 5. Components & UI
- **Home Page:** Global search and quick links to geographic categories.
- **Pub Detail Page:** 
    - Header with current status.
    - Statistics card (Streak, 5yr %, 10yr %, Overall %).
    - History timeline/grid.
    - Interactive map snippet.
    - External links section (WhatPub, Google Maps, Untappd, RGL).
- **Proximity Search:** List view sorted by distance from the user's current location.
- **Export Toolbar:** Simple UI to trigger CSV/JSON/Parquet downloads for filtered views.

## 6. Implementation Phases
1. **Phase 1: Tooling & Data Import:** CLI tool to parse Excel and seed Postgres/PostGIS.
2. **Phase 2: Core Web App:** Basic Leptos site with pub listing and detail pages.
3. **Phase 3: Spatial Features:** Geocoding (OSM) and "near me" proximity search.
4. **Phase 4: Stats & Exports:** Implementing the calculation engine and file exporters.
5. **Phase 5: PWA & Polish:** Adding manifest, service worker, and refining the "Classic Pub" CSS.

## 7. Future Considerations (Stretch Goals)
- User-reported closures and name changes.
- Logged-in user "check-ins" or visits.
- RDF/SPARQL endpoints for Linked Data.
- Multi-year spreadsheet imports.
