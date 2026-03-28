# Design Spec: Community Suggestion Form

**Date:** 2026-03-28
**Status:** Draft
**Topic:** User interface for logged-in users to propose data updates for pubs.

---

## 1. Objective
Empower the community to help maintain data accuracy by providing a streamlined way to suggest corrections for pub status, links, location, and history.

## 2. User Interface Designs

### 2.1 `PubDetail` Entry Point
- **Button**: "Suggest Update"
- **Placement**: Header section, next to "Edit" (for admins) or as the primary action (for non-admins).
- **Visibility**: Visible only to authenticated users.

### 2.2 Suggestion Modal (Quick Actions)
A multi-category modal that allows users to focus on specific types of corrections.

#### 2.2.1 Action Menu
Upon opening, the user sees buttons for:
- **"Report as Closed"**
- **"Add/Fix External Links"**
- **"Correct Address/Location"**
- **"Update GBG History"**
- **"General Correction"**

#### 2.2.2 Category Forms
- **Closed Status**: Toggle for `closed` + "Reason/Notes" textarea.
- **Links**: Inputs for `whatpub_id`, `google_maps_id`, and `untappd_id`.
- **Location**: Inputs for `address`, `town`, `region`, and `postcode`.
- **History**: A clickable year grid (1972-current) to suggest missing inclusion years.
- **General**: A single "Additional Information" textarea.

### 2.3 Submission & Confirmation
- **Action**: "Submit Suggestion" button.
- **Payload**: A JSON object containing the modified pub fields.
- **Feedback**: A "Thank you" message and automatic modal closure.

## 3. Data Flow & Integration

### 3.1 Server Functions
- **`SuggestUpdate(pub_id, suggested_data)`**: Already exists in `server.rs`. Takes the `Uuid` and `serde_json::Value`.
- **`GetPubDetail(id)`**: Used to pre-populate the suggestion state with current data.

### 3.2 State Management
- Initialize a local `PubDetail` state when the modal opens.
- Track which fields the user has modified.
- Merge modifications into the payload sent to the server.

## 4. Security & Validation
- **Authentication**: Mandatory active session.
- **Rate Limiting**: (Future) Prevent spam by limiting suggestions per user per day.
- **License**: Users must agree that their suggestions are contributed under the site's open license (implied).

## 5. Implementation Phases
1. **Phase 1**: Add the "Suggest Update" button to `PubDetail`.
2. **Phase 2**: Implement the Category-based Modal UI.
3. **Phase 3**: Connect the "Submit" action to the `SuggestUpdate` server function.
4. **Phase 4**: Add styling and success feedback.
