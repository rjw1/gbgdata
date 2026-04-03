# Design Spec: Private Mode for GBG Data Explorer

## 1. Overview
This feature allows the site owner to restrict access to the application so that only logged-in users can view pub data and statistical information. It uses a hybrid configuration model (environment variables + database settings) to provide both robustness and flexibility.

## 2. Requirements
- **Restriction**: When Private Mode is active, all data-heavy pages (Home, Explore, Pub Detail, Near Me, My Visits, Rankings) are inaccessible to unauthenticated users.
- **Exceptions**: Essential pages like `/login`, `/register`, `/about`, and static assets (`/pkg/*`, `/assets/*`) remain public.
- **Redirection**: Unauthenticated users trying to access restricted pages are redirected to `/login`.
- **Hybrid Configuration**:
  - `PRIVATE_MODE=true` in `.env`: Site is permanently private (Owner/DB cannot override to public).
  - No env var or `PRIVATE_MODE=false`: Site visibility depends on the database setting.
- **Management**: A new "Settings" tab in the Admin Dashboard for owners and admins to toggle Private Mode (if not hard-locked by env var).
- **User Feedback**: A "PRIVATE" badge in the navigation bar when the mode is active.

## 3. Architecture & Data Model

### 3.1 Database Migration
New table `site_settings` to store site-wide configurations.

```sql
CREATE TABLE site_settings (
    id SERIAL PRIMARY KEY,
    private_mode BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_by UUID REFERENCES users(id)
);

-- Initialize with default public state
INSERT INTO site_settings (private_mode) VALUES (FALSE);
```

### 3.2 Server-Side Logic
A new server function `get_site_settings` and `update_site_settings` will be added to `server.rs`.

**Private Mode Resolution (Pseudo-code):**
```rust
fn is_private_mode_active() -> bool {
    let env_private = std::env::var("PRIVATE_MODE").unwrap_or_default() == "true";
    if env_private {
        return true;
    }
    // Fallback to database setting (cached or fetched per request)
    db::get_private_mode_setting()
}
```

### 3.3 Access Control Middleware
Modify or supplement `admin_auth_middleware` in `server.rs` to handle site-wide redirection.

```rust
// Rules:
// 1. If path is in ALLOWED_PUBLIC_PATHS, continue.
// 2. If is_private_mode_active() AND user is None, redirect to /login.
// 3. Continue to standard route handling (including admin checks).
```

## 4. Components & UI

### 4.1 Admin Dashboard
- **New Tab**: "Settings" (using existing tab structure).
- **Content**: A toggle for "Private Mode" with a description.
- **Enforcement**: Disable the toggle if the environment variable override is active.

### 4.2 Navigation Bar
- **Badge**: Add a small `<span class="badge-private">PRIVATE</span>` next to the main links when `is_private_mode_active()` is true.
- **Styling**: Ensure it's clear but not intrusive (e.g., small red or orange pill).

## 5. Implementation Strategy
1. **Migration**: Create the `site_settings` table.
2. **Models**: Add `SiteSettings` struct to `models.rs`.
3. **Server Logic**: Implement `get_site_config` and `update_site_config` server functions.
4. **Middleware**: Update `admin_auth_middleware` to handle site-wide redirection.
5. **UI**: Add the Settings tab to `AdminDashboard` and the badge to `App`'s nav.

## 6. Testing
- **E2E Tests**:
  - Verify that restricted pages redirect to `/login` when Private Mode is enabled via DB.
  - Verify that restricted pages redirect to `/login` when Private Mode is enabled via Env Var.
  - Verify that `/about` and `/login` remain accessible in Private Mode.
  - Verify that authenticated users can see data even in Private Mode.
  - Verify that non-admins cannot see the Settings tab.
