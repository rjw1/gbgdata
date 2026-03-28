# Design Spec: User Engagement and Advanced Admin Features

## 1. Objective
Enhance the GBG Data Explorer by introducing a "Normal User" role for visit tracking, a suggestion system for community data improvement, and advanced administrative tools like bulk editing and missing data reports.

## 2. Success Criteria
- Support for "Normal Users" who can mark pubs as visited and manage their own visit history.
- An "Invite-Only" registration system for both users and admins.
- User-driven "Suggested Updates" with a side-by-side admin triage interface.
- Advanced admin reports for identifying pubs with missing data (coordinates, external IDs, etc.).
- Bulk editing capabilities for towns, outcodes, and regions.
- Support for Flickr photos with mandatory CC license verification and attribution.
- Passkey (WebAuthn) support for passwordless login.
- Comprehensive visit exports in JSON, CSV, and Parquet.
- Map-based visualization of a user's visit history.

## 3. Architecture

### 3.1 Database Schema (New & Modified)

#### `users` Table (Modified)
- `role`: VARCHAR(10) (e.g., 'admin', 'user')
- `totp_setup_completed`: BOOLEAN (Default: FALSE)

#### `user_invites` Table
- `id`: UUID (Primary Key)
- `role`: VARCHAR(10) (e.g., 'admin', 'user')
- `expires_at`: TIMESTAMPTZ
- `created_by`: UUID (References users.id)
- `used_at`: TIMESTAMPTZ (NULL if unused)

#### `user_visits` Table
- `id`: UUID (Primary Key)
- `user_id`: UUID (References users.id)
- `pub_id`: UUID (References pubs.id)
- `visit_date`: DATE
- `notes`: TEXT
- UNIQUE (user_id, pub_id, visit_date)

#### `suggested_updates` Table
- `id`: UUID (Primary Key)
- `pub_id`: UUID (References pubs.id)
- `user_id`: UUID (References users.id)
- `status`: VARCHAR(10) ('pending', 'approved', 'rejected')
- `suggested_data`: JSONB (Stores changed fields)
- `created_at`: TIMESTAMPTZ
- `processed_at`: TIMESTAMPTZ
- `processed_by`: UUID (References users.id)

#### `pub_photos` Table
- `id`: UUID (Primary Key)
- `pub_id`: UUID (References pubs.id)
- `user_id`: UUID (References users.id, uploader)
- `flickr_id`: TEXT
- `image_url`: TEXT
- `owner_name`: TEXT
- `license_type`: TEXT
- `license_url`: TEXT
- `original_url`: TEXT (Backlink to source)
- `is_cc_licensed`: BOOLEAN

#### `user_credentials` (Passkeys/WebAuthn)
- `id`: UUID (Primary Key)
- `user_id`: UUID (References users.id)
- `credential_id`: BYTEA
- `public_key`: BYTEA
- `sign_count`: BIGINT
- `transports`: TEXT[]

### 3.2 Security Model
- **Invite System**: Registration requires a valid, unexpired token.
- **Forced MFA**: New users are redirected to `/setup-mfa` after initial login if `totp_setup_completed` is FALSE.
- **Passkeys**: WebAuthn standard for primary or secondary authentication.
- **Role-Based Access**:
    - `admin`: Full editing, bulk actions, triage suggestions, missing info reports.
    - `user`: Suggesting updates, visit tracking, exports.

## 4. Components

### 4.1 User Features
- **Visit Tracking**:
    - **Pub Page**: Badge showing "Visited X times, last on YYYY-MM-DD". "Log Visit" button.
    - **My Visits Page (`/my-visits`)**: List of all visits with filtering and a Map view (Leaflet).
    - **Exports**: Buttons for CSV, JSON, and Parquet.
- **Suggesting Updates**:
    - "Suggest Update" button on pub detail pages.
    - Form to edit basic pub info, with validation.

### 4.2 Administrative Features
- **Invite Management**: Admin UI to generate invite links.
- **Triage UI (`/admin/suggestions`)**: Side-by-side comparison of current vs. suggested values.
- **Missing Info Report (`/admin/reports/missing`)**: Filters for Region/Town/Outcode to find pubs with missing IDs, coords, or years.
- **Bulk Edit**:
    - **Dedicated Page**: Apply changes to all pubs in a Town/Outcode/Region.
    - **Selection Mode**: Checkboxes in search results for multi-select and bulk action.
- **Flickr Integration**:
    - "Fetch from Flickr" button using ID/URL.
    - Server-side validation of CC license via Flickr API.
    - Manual entry fallback.

### 4.3 General
- **External Links**: Search links (Google, WhatPub, Untappd) when IDs are missing using specific query patterns (Name+Postcode for Google/WhatPub, Name+Town for Untappd).
- **Google Place Finder**: Link to ID finder tool in the edit form.

## 5. Implementation Strategy

### Phase 1: Authentication & Roles
- Migration for new tables and `role` column.
- Update `auth.rs` and `server.rs` to support roles and TOTP setup flow.
- Implement invite link generation and registration.

### Phase 2: User Engagement (Visits & Suggestions)
- Implement `user_visits` tracking and "My Visits" page (List + Map).
- Implement "Suggest Update" flow and admin triage UI.
- Implement visit exports (CSV, JSON, Parquet).

### Phase 3: Advanced Admin Tools
- Implement Flickr photo integration with CC license check.
- Implement Missing Data reports (page + inline icons).
- Implement Bulk Editing (dedicated page + list selection).

### Phase 4: Passkeys (WebAuthn)
- Integrate WebAuthn (e.g., using `webauthn-rs`).
- Implement registration and login flows.

### Phase 5: Testing, ADRs, and Docs
- **ADRs**: Document significant architectural decisions for Passkeys, Bulk Editing, and the Suggestion System.
- **Tests**: 
    - Unit tests for visit exports and Flickr API parsing.
    - Integration tests for role-based access control.
    - Playwright end-to-end tests for the visit tracking and suggestion flows.
- **Docs**: Update `docs/usage.md` and `docs/hosting.md` with user registration and administrative features.

## 6. Risks & Constraints
- **Flickr API**: Requires an API key and handling rate limits.
- **Passkey Complexity**: WebAuthn can be tricky to implement correctly across all browsers.
- **Bulk Actions Performance**: Large updates could trigger long `REFRESH MATERIALIZED VIEW` times.

## 7. Open Questions
- Should "Normal Users" be able to delete their own visit history? *Decision: Yes, on the My Visits page.*
- What happens if an admin modifies a pub while a suggestion is pending? *Decision: The triage view will show the updated "Current" value.*
