# User Engagement and Advanced Admin Features Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Enhance the GBG Data Explorer with user visit tracking, community suggestions, advanced admin reporting, bulk editing, and passkey authentication.

**Architecture:** Extend the existing Postgres schema and Leptos/Axum backend to support role-based access control, a multi-stage registration/MFA flow, and dedicated tables for user engagement and administrative auditing.

**Tech Stack:** Rust (Leptos 0.8.0, Axum, SQLx), Postgres + PostGIS, Leaflet, `webauthn-rs`, `parquet`.

---

### Phase 1: Authentication & Roles

#### Task 1: Database Migration for Roles and Invites

**Files:**
- Create: `migrations/20260328000001_user_engagement_schema.sql`

- [ ] **Step 1: Write the migration**

```sql
-- migrations/20260328000001_user_engagement_schema.sql
ALTER TABLE users ADD COLUMN role VARCHAR(10) NOT NULL DEFAULT 'user';
ALTER TABLE users ADD COLUMN totp_setup_completed BOOLEAN NOT NULL DEFAULT FALSE;

CREATE TABLE user_invites (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    role VARCHAR(10) NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    created_by UUID REFERENCES users(id),
    used_at TIMESTAMPTZ
);

CREATE TABLE user_visits (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    pub_id UUID NOT NULL REFERENCES pubs(id),
    visit_date DATE NOT NULL,
    notes TEXT,
    UNIQUE (user_id, pub_id, visit_date)
);

CREATE TABLE suggested_updates (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pub_id UUID NOT NULL REFERENCES pubs(id),
    user_id UUID NOT NULL REFERENCES users(id),
    status VARCHAR(10) NOT NULL DEFAULT 'pending',
    suggested_data JSONB NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    processed_at TIMESTAMPTZ,
    processed_by UUID REFERENCES users(id)
);

CREATE TABLE pub_photos (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    pub_id UUID NOT NULL REFERENCES pubs(id),
    user_id UUID REFERENCES users(id),
    flickr_id TEXT,
    image_url TEXT NOT NULL,
    owner_name TEXT NOT NULL,
    license_type TEXT NOT NULL,
    license_url TEXT NOT NULL,
    original_url TEXT NOT NULL,
    is_cc_licensed BOOLEAN DEFAULT TRUE,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
```

- [ ] **Step 2: Apply the migration**

Run: `sqlx migrate run`

- [ ] **Step 3: Commit**

```bash
git add migrations/20260328000001_user_engagement_schema.sql
git commit -m "db: add user engagement and roles schema"
```

#### Task 2: Update Auth Models and Session Logic

**Files:**
- Modify: `web-app/src/auth.rs`
- Modify: `web-app/src/models.rs`

- [ ] **Step 1: Update `User` struct in `auth.rs`**

```rust
// web-app/src/auth.rs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub role: String,
    pub totp_setup_completed: bool,
}
```

- [ ] **Step 2: Update `User` model in `models.rs`**

```rust
// web-app/src/models.rs
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub role: String,
    pub totp_setup_completed: bool,
}
```

- [ ] **Step 3: Update `login` server function in `server.rs` to fetch full user info**

```rust
// web-app/src/server.rs:450 (approx)
// Inside your login logic:
let user = sqlx::query_as!(User, "SELECT id, username, role, totp_setup_completed FROM users WHERE id = $1", db_user.id)
    .fetch_one(&pool).await?;
session::login(&session, &user).await?;
```

- [ ] **Step 4: Commit**

```bash
git add web-app/src/auth.rs web-app/src/models.rs web-app/src/server.rs
git commit -m "feat: update user model with role and totp status"
```

### Phase 2: Visit Tracking

#### Task 3: Visit Tracking Server Functions

**Files:**
- Modify: `web-app/src/server.rs`

- [ ] **Step 1: Add `LogVisit`, `GetUserVisits`, and `GetPubVisitStatus`**

```rust
#[server(LogVisit, "/api")]
pub async fn log_visit(pub_id: Uuid, visit_date: String, notes: Option<String>) -> Result<(), ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use crate::auth::session;

    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;
    let session = extract::<Session>().await?;
    let user = session::get_user(&session).await.ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    let date = chrono::NaiveDate::parse_from_str(&visit_date, "%Y-%m-%d")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    sqlx::query!(
        "INSERT INTO user_visits (user_id, pub_id, visit_date, notes) VALUES ($1, $2, $3, $4)
         ON CONFLICT (user_id, pub_id, visit_date) DO NOTHING",
        user.id, pub_id, date, notes
    )
    .execute(&pool).await?;

    Ok(())
}

#[server(GetUserVisits, "/api")]
pub async fn get_user_visits() -> Result<Vec<crate::models::VisitRecord>, ServerFnError> {
    // ... Implementation fetching from user_visits JOIN pubs ...
    Ok(vec![])
}
```

- [ ] **Step 2: Commit**

```bash
git add web-app/src/server.rs
git commit -m "feat: add visit tracking server functions"
```

### Phase 3: Advanced Admin Tools

#### Task 4: Flickr API Integration

**Files:**
- Modify: `web-app/Cargo.toml`
- Modify: `web-app/src/server.rs`

- [ ] **Step 1: Add `reqwest` and `serde_json` for server-side API calls if not present**

- [ ] **Step 2: Implement `FetchFlickrPhoto` server function**

```rust
#[server(FetchFlickrPhoto, "/api")]
pub async fn fetch_flickr_photo(url_or_id: String) -> Result<crate::models::FlickrPhotoInfo, ServerFnError> {
    let api_key = std::env::var("FLICKR_API_KEY").map_err(|_| ServerFnError::new("Flickr API Key missing"))?;
    // 1. Extract Photo ID
    // 2. Call flickr.photos.getInfo and flickr.photos.getSizes
    // 3. Verify license (CC licenses are 1, 2, 3, 4, 5, 6, 9)
    // 4. Return FlickrPhotoInfo
    Ok(crate::models::FlickrPhotoInfo { .. })
}
```

- [ ] **Step 3: Commit**

```bash
git add web-app/src/server.rs web-app/Cargo.toml
git commit -m "feat: add flickr api integration"
```

### Phase 4: Passkeys (WebAuthn)

#### Task 5: WebAuthn Schema and Registration

**Files:**
- Create: `migrations/20260328000002_passkeys_schema.sql`
- Modify: `web-app/src/server.rs`

- [ ] **Step 1: Create Passkeys table**

```sql
CREATE TABLE user_credentials (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    credential_id BYTEA NOT NULL,
    public_key BYTEA NOT NULL,
    sign_count BIGINT NOT NULL DEFAULT 0,
    transports TEXT[],
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
```

- [ ] **Step 2: Implement registration challenge and verification server functions**

- [ ] **Step 3: Commit**

```bash
git add migrations/20260328000002_passkeys_schema.sql web-app/src/server.rs
git commit -m "feat: add webauthn passkey support"
```

---

*This plan is a high-level roadmap. Each task involves TDD: write the model/test first, then the migration/server-function, then the UI.*
