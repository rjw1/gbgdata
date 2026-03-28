# Administrative Data Editing Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a secure administrative interface to allow authorized users to edit pub data and historical records with mandatory 2FA.

**Architecture:** Use `tower-sessions` for session management, `argon2` for password hashing, and `totp-rs` for MFA. Implement a dedicated `/admin` dashboard and inline editing on pub detail pages, with all changes logged to an `audit_log` table.

**Tech Stack:** Leptos (SSR/Hydrate), Axum, SQLx, Argon2, TOTP-rs, Tower-Sessions.

---

### Task 1: Database Schema

**Files:**
- Create: `migrations/20260328000000_admin_auth_and_audit.sql`

- [ ] **Step 1: Create migration for auth and audit tables**

```sql
-- migrations/20260328000000_admin_auth_and_audit.sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    totp_secret_enc BYTEA NOT NULL,
    recovery_codes_hash TEXT[] NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    last_login TIMESTAMPTZ
);

CREATE TABLE sessions (
    id TEXT PRIMARY KEY,
    data BYTEA NOT NULL,
    expiry_date TIMESTAMPTZ NOT NULL
);

CREATE TABLE audit_log (
    id SERIAL PRIMARY KEY,
    user_id UUID REFERENCES users(id),
    action VARCHAR(50) NOT NULL,
    entity_type VARCHAR(20) NOT NULL,
    entity_id UUID NOT NULL,
    old_value JSONB,
    new_value JSONB,
    timestamp TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
```

- [ ] **Step 2: Run migrations**

Run: `sqlx migrate run` (or equivalent via docker if database is remote)
Expected: Success

- [ ] **Step 3: Commit**

```bash
git add migrations/20260328000000_admin_auth_and_audit.sql
git commit -m "feat: add auth and audit tables to schema"
```

---

### Task 2: Backend Dependencies

**Files:**
- Modify: `web-app/Cargo.toml`

- [ ] **Step 1: Add required dependencies**

```toml
# web-app/Cargo.toml
[dependencies]
# ... existing
argon2 = { version = "0.5", features = ["std"] }
totp-rs = { version = "6.0", features = ["qr"] }
tower-sessions = { version = "0.13", optional = true }
tower-sessions-sqlx-store = { version = "0.13", features = ["postgres"], optional = true }
base64 = "0.22"
rand = "0.8"

[features]
# ...
ssr = [
    # ... existing
    "dep:tower-sessions",
    "dep:tower-sessions-sqlx-store",
]
```

- [ ] **Step 2: Verify build**

Run: `cargo check --features ssr`
Expected: Success

- [ ] **Step 3: Commit**

```bash
git add web-app/Cargo.toml
git commit -m "chore: add auth and session dependencies"
```

---

### Task 3: CLI Admin Bootstrapping

**Files:**
- Modify: `import-tool/src/db.rs`
- Modify: `import-tool/src/main.rs`

- [ ] **Step 1: Add user creation logic to db.rs**

```rust
// import-tool/src/db.rs
pub async fn create_user(pool: &sqlx::PgPool, username: &str, password_hash: &str, totp_secret_enc: &[u8], recovery_codes: Vec<String>) -> Result<()> {
    sqlx::query!(
        "INSERT INTO users (username, password_hash, totp_secret_enc, recovery_codes_hash) VALUES ($1, $2, $3, $4)",
        username, password_hash, totp_secret_enc, &recovery_codes
    )
    .execute(pool)
    .await?;
    Ok(())
}
```

- [ ] **Step 2: Add create-admin command to main.rs**

```rust
// import-tool/src/main.rs
// Add Argon2 and TOTP logic here to generate the initial user.
// Show the TOTP URI to the user.
```

- [ ] **Step 3: Test CLI bootstrap**

Run: `cargo run -p import-tool -- create-admin --username admin --password secret`
Expected: Output TOTP URI and success message.

- [ ] **Step 4: Commit**

```bash
git add import-tool/src/db.rs import-tool/src/main.rs
git commit -m "feat: add CLI tool for bootstrapping admin users"
```

---

### Task 4: Server-Side Auth Logic

**Files:**
- Create: `web-app/src/auth.rs`
- Modify: `web-app/src/lib.rs`
- Modify: `web-app/src/main.rs` (Axum setup)

- [ ] **Step 1: Implement password and TOTP verification**

```rust
// web-app/src/auth.rs
// verify_password, verify_totp, create_session, get_current_user
```

- [ ] **Step 2: Register tower-sessions in Axum main.rs**

- [ ] **Step 3: Commit**

```bash
git add web-app/src/auth.rs web-app/src/lib.rs web-app/src/main.rs
git commit -m "feat: implement server-side auth logic and sessions"
```

---

### Task 5: Login UI

**Files:**
- Create: `web-app/src/components/login.rs`
- Modify: `web-app/src/app.rs`

- [ ] **Step 1: Create LoginForm component**

- [ ] **Step 2: Create TotpChallenge component**

- [ ] **Step 3: Add routes to app.rs**

- [ ] **Step 4: Commit**

```bash
git add web-app/src/components/login.rs web-app/src/app.rs
git commit -m "feat: add login and 2FA challenge UI"
```

---

### Task 6: Edit Pub Form

**Files:**
- Create: `web-app/src/components/edit_pub.rs`
- Modify: `web-app/src/server.rs`

- [ ] **Step 1: Implement UpdatePub server function with audit logging**

- [ ] **Step 2: Create EditPub component with form fields**

- [ ] **Step 3: Add history toggle checkboxes to the form**

- [ ] **Step 4: Commit**

```bash
git add web-app/src/components/edit_pub.rs web-app/src/server.rs
git commit -m "feat: add pub editing form with history management"
```

---

### Task 7: Admin Dashboard & Wiring

**Files:**
- Create: `web-app/src/components/admin.rs`
- Modify: `web-app/src/components/pub_detail.rs`

- [ ] **Step 1: Create AdminDashboard with audit log view**

- [ ] **Step 2: Add "Edit" button to PubDetail (conditional on auth)**

- [ ] **Step 3: Final verification of the end-to-end flow**

- [ ] **Step 4: Commit**

```bash
git add web-app/src/components/admin.rs web-app/src/components/pub_detail.rs
git commit -m "feat: complete admin dashboard and wire up edit buttons"
```
