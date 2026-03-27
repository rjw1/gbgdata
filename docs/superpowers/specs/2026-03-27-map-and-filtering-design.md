# Design Spec: Map View & Filtering

Enhance the explorer with a visual map representation of pub locations and a granular "open/closed" status filter.

## 1. Map View Integration

### 1.1 Goal
Provide a visual, interactive map for users to explore pub locations within a Region, Town, or Outcode.

### 1.2 Library Selection
- **Leaflet**: Use Leaflet (via a Leptos wrapper or direct JS interop) for its light footprint and reliability.
- **Provider**: OpenStreetMap (OSM) for map tiles.

### 1.3 Components
- **`MapView` Component**: A reusable component that takes a list of pub coordinates and renders markers on the map.
- **`Explorer` Integration**: The map will be available on both the Region and Town/Outcode pages, flanking the list view or as a separate tab.

### 1.4 Data Flow
- Pubs with `latitude` and `longitude` will be passed as a signal to the `MapView`.
- Clicking a marker will open a popup with the pub's name and a link to its detail page.

## 2. "Show Only Open Pubs" Toggle

### 2.1 Filtering Logic
A global toggle on the Explorer pages will allow users to filter out pubs marked as `closed = true`.
- **Default State**: Show all pubs (both open and closed).
- **Persistence**: The toggle state will be maintained in the URL query parameters (e.g., `?open_only=true`) for shareability.

### 2.2 Server-Side Support
- Update server functions (`get_pubs_by_location`, `get_county_details`) to accept an `open_only` boolean.
- Modify SQL queries to include `AND (closed = false OR $2 = false)` in the `WHERE` clause.

## 3. Implementation Details

### 3.1 UI Design
- **Toggle Location**: Next to the sort and filter options on the explorer pages.
- **Map Toggle**: A button to switch between "List View" and "Map View" on mobile, or a side-by-side layout on desktop.

### 3.2 Performance
- **Marker Clustering**: If a region has >1000 pubs, use a clustering plugin (e.g., Leaflet.markercluster) to maintain performance.

## 4. Testing Strategy
- **Filtering**: Verify that when "Open Only" is toggled, no closed pubs appear in the list or on the map.
- **Map Accuracy**: Check that a sample of pubs are plotted in the correct geographic locations.
- **Responsive Layout**: Ensure the map doesn't break the layout on small screens.
