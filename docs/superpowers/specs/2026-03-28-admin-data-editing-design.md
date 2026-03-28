# Design Spec: Administrative Data Editing

## 1. Objective
Add a secure administrative interface to allow authorized users to edit pub data and historical records.

## 2. Success Criteria
- Secure authentication system with local user accounts.
- Mandatory Multi-Factor Authentication (MFA) via TOTP.
- Full CRUD (Create, Read, Update, Delete) capabilities for pubs and their Good Beer Guide (GBG) history.
- Audit logging for all administrative changes.
- CLI tool to bootstrap the first administrator.
- Immediate update of statistics and rankings after data modification.

## 3. Architecture

### 3.1 Database Schema
New tables and modifications to support authentication and auditing:

#### `users` Table
- `id`: UUID (Primary Key)
- `username`: VARCHAR(50) (Unique)
- `password_hash`: TEXT (Argon2id)
- `totp_secret_enc`: BYTEA (Encrypted TOTP secret)
- `recovery_codes_hash`: TEXT[] (Hashed one-time recovery codes)
- `created_at`: TIMESTAMPTZ
- `last_login`: TIMESTAMPTZ

#### `user_sessions` Table
- Managed by `tower-sessions` with `sqlx-store` for persistence.

#### `audit_log` Table
- `id`: SERIAL (Primary Key)
- `user_id`: UUID (References users.id)
- `action`: VARCHAR(20) (e.g., 'UPDATE_PUB', 'DELETE_PUB', 'EDIT_HISTORY')
- `entity_type`: VARCHAR(20) (e.g., 'pub', 'gbg_history')
- `entity_id`: UUID/INT
- `old_value`: JSONB (State before change)
- `new_value`: JSONB (State after change)
- `timestamp`: TIMESTAMPTZ

### 3.2 Security Model
- **Password Hashing**: Argon2id with recommended parameters.
- **2FA (MFA)**: Mandatory Time-based One-Time Password (TOTP) using `totp-rs`.
- **Session Management**: Secure, HTTP-only cookies managed via `tower-sessions`.
- **Encryption**: `pgcrypto` or a Rust-side encryption library for sensitive data like `totp_secret`.
- **Access Control**: Server-side functions (`#[server]`) will verify session validity and user roles (initially all users are admins) before performing mutations.

## 4. Components

### 4.1 CLI Bootstrap (`import-tool`)
- `import-tool create-admin --username <name> --password <pass>`
- Generates a random salt and hashes the password.
- Generates a TOTP secret and 5 recovery codes.
- Outputs the TOTP setup URI (compatible with Google Authenticator, Authy, etc.).
- Persists the new user to the database.

### 4.2 Web Login Flow
1. **Step 1**: Username and Password submission.
2. **Step 2**: If credentials match, redirect to a TOTP challenge page.
3. **Step 3**: On valid TOTP (or recovery code), create a session and redirect to the dashboard.

### 4.3 Admin UI (`web-app`)
- **Dashboard (`/admin`)**:
    - List of recent audit log entries.
    - User management (adding/removing other admins).
- **Inline Pub Editing**:
    - "Edit" button visible to authenticated admins on the Pub Detail page (`/pub/:id`).
    - Opens a form to modify all fields in the `pubs` table (name, address, town, region, country_code, postcode, closed status, and coordinates).
    - Map-based coordinate selector (clicking on the map to set lat/lon).
- **History Management**:
    - Grid view of all years (1973–Present) with checkboxes for presence in the GBG.
    - Special handling for the 1972 trial year (as per project mandates, it remains excluded from stats but should be visible for completeness).

## 5. Implementation Strategy

### Phase 1: Authentication Core
- Add dependencies to `web-app/Cargo.toml`.
- Implement user and session tables.
- Create CLI bootstrap command in `import-tool`.
- Implement server-side login and 2FA verification logic.

### Phase 2: Administrative UI
- Build the login and 2FA challenge components.
- Create the `/admin` dashboard.
- Implement the "Edit Pub" form and coordinate picker.
- Implement the history management interface.

### Phase 3: Audit & Integrity
- Integrate audit logging into all mutation functions.
- Ensure `pub_stats` view is refreshed after any data change.
- Add end-to-end tests for the admin flow.

## 6. Risks & Constraints
- **Security**: Hardening the session management is critical.
- **Complexity**: Synchronizing coordinates between the form and the map component.
- **Performance**: Frequent `REFRESH MATERIALIZED VIEW CONCURRENTLY pub_stats` should be handled carefully to avoid locking.

## 7. Open Questions
- Should we support bulk editing (e.g., closing all pubs in a town at once)? *Decision: No, focused on individual pub accuracy for now.*
- How should recovery codes be handled after use? *Decision: They are one-time use and should be removed from the database once used.*
