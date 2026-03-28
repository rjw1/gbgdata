# Design Spec: User Engagement (Visit Tracking & My Visits)

**Date:** 2026-03-28
**Status:** Draft
**Topic:** Implementation of user visit tracking, a personal visit history dashboard, and data exports.

---

## 1. Objective
Enable logged-in users to track their visits to pubs, view their personal history through a dashboard and interactive map/list, and export their data for external use.

## 2. User Interface Designs

### 2.1 `PubDetail` "My Activity" Section
A new UI component on the pub details page, visible only to authenticated users.
- **Placement**: Above "External Links", below "Historical Data".
- **Summary**: "You have visited this pub **{count}** times. Last visit: **{date}**" (or "Never" if count is 0).
- **Primary Action**: "Log Visit" button (Blue/Primary styling).

### 2.2 Log Visit Modal
A modal dialog triggered by the "Log Visit" button.
- **Fields**:
    - `visit_date`: Date input (default: Today).
    - `notes`: Optional textarea for personal visit notes.
- **Actions**: "Save Visit" (dispatches `LogVisit` server function) and "Cancel".
- **Validation**: Ensure date is not in the future.

### 2.3 `/my-visits` Page (User Dashboard)
A new top-level route accessible via the main navigation for logged-in users.

#### 2.3.1 Dashboard Header
Four cards displaying high-level statistics:
- **Unique Pubs**: Count of distinct `pub_id` values in `user_visits`.
- **Total Visits**: Count of all records in `user_visits`.
- **Visit Streak**: Longest run of consecutive years with at least one visit recorded (calculated from `visit_date`).
- **Top Region**: The region with the highest number of unique pubs visited.

#### 2.3.2 View Toggle & Controls
- **Toggle**: "List" | "Map" buttons to switch the main view.
- **Export Menu**: A dropdown or button group for "Export as CSV", "Export as JSON", and "Export as Parquet".

#### 2.3.3 List View
A table displaying the user's visit history:
- **Columns**: `Date`, `Pub Name`, `Town`, `Region`, `Notes`, `Actions`.
- **Actions**: "Delete" (with confirmation) to remove a visit record.

#### 2.3.4 Map View
A Leaflet-based map displaying markers for all visited pubs.
- **Popup**: Displays Pub Name, Date of last visit, and a link to the `PubDetail` page.
- **Integration**: Reuse logic from `NearMe` and `Explorer` components.

## 3. Data Flow & Integration

### 3.1 Server Functions
- **`LogVisit`**: Already exists in `server.rs`.
- **`GetUserVisits`**: Already exists in `server.rs`. Returns `Vec<VisitRecord>`.
- **`GetPubVisitStatus`**: Already exists in `server.rs`. Returns visit count and last visit date.
- **`DeleteVisit`**: **To be implemented.** Allows a user to remove a specific visit entry.
- **`ExportUserVisits`**: Already exists in `server.rs`. Needs to be integrated into the UI.

### 3.2 State Management
- Use `Resource` in Leptos to fetch visit data.
- Trigger resource refreshes after successful "Log Visit" or "Delete Visit" actions.

## 4. Security & Access Control
- All visit tracking features require an active session.
- Users can only view, edit, or delete their own visit history (`WHERE user_id = $1`).
- The `/my-visits` route redirects to `/login` if no user is authenticated.

## 5. Implementation Phases
1. **Phase 1**: Add "My Activity" section and "Log Visit" modal to `PubDetail`.
2. **Phase 2**: Implement the `/my-visits` route with the Dashboard Header and List View.
3. **Phase 3**: Implement the Map View toggle and integration.
4. **Phase 4**: Add Export buttons and "Delete Visit" functionality.
