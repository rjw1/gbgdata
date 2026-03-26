# Design Spec: "Near Me" Proximity Search Enhancements

## 1. Executive Summary
The **"Near Me" Enhancements** upgrade the existing proximity search to allow users greater control and flexibility. It introduces a "Smart Search" bar for manual location entry (Town, Postcode, or Coordinates) and a dual-input system (slider and text) for adjusting the search radius.

## 2. Goals & Success Criteria
- **Manual Location Entry:** Allow users to search for pubs near a specific place without using GPS.
- **Flexible Distance:** Provide intuitive controls to adjust the search radius from 1km to 50km.
- **Smart Parsing:** Automatically distinguish between coordinate pairs and text-based place names.
- **Improved UX:** Maintain a fast, reactive interface that updates the results as settings change.

## 3. Architecture & Routing
This feature modifies the existing `/near-me` route and introduces a new server-side geocoding utility.

### 3.1 Server Functions
- **`geocode_manual(query: String)`:**
    - Parses coordinates if in `lat, lon` format.
    - Queries the local Nominatim instance (`http://nominatim:8080`) for place names/postcodes.
    - Returns `Result<Option<(f64, f64)>, ServerFnError>`.

### 3.2 `NearMe` Component State
- `radius`: `RwSignal<f64>` (in meters, default 5000.0).
- `center_coords`: `RwSignal<Option<(f64, f64)>>`.
- `search_query`: `RwSignal<String>`.

## 4. UI Components
### 4.1 Smart Search Bar
- A single text input field.
- A "Search" button to trigger manual geocoding.
- A "Use My GPS" icon/button to trigger browser geolocation.

### 4.2 Radius Controls
- **Slider:** 1-50 range.
- **Number Input:** Synced with the slider for precise entry.
- Label showing the current radius in kilometers.

### 4.3 Results Grid
- Displays `PubCard` components sorted by distance (existing component).

## 5. Implementation Details
- **Outcode Logic:** Local Nominatim will handle full postcodes and outcodes naturally.
- **Coordinate Regex:** A simple regex will check for `^-?\d+\.?\d*,\s*-?\d+\.?\d*$` to identify manual coordinate entry.
- **Docker Connectivity:** Ensure the `web` service has `NOMINATIM_URL` set to the internal container address.

## 6. Testing Strategy
- **Input Parsing:** Test with various strings: "Godalming", "GU7 1RG", "51.1, -0.6", "Invalid String".
- **Sync Logic:** Verify that changing the slider updates the text input and vice versa.
- **Radius Filtering:** Confirm that increasing the radius actually pulls in more distant pubs from the database.
