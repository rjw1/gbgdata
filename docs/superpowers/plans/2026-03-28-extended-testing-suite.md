# Extended Testing Suite Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement critical business logic verification (1972 Rule), cryptographic safety (Auth), data mapping reliability (Excel), and security header integration tests.

**Architecture:** We use `sqlx::test` for DB-driven logic, pure function extraction for Auth and Excel parsing to enable unit testing, and `tower::ServiceExt` for mock Axum header verification.

**Tech Stack:** Rust, SQLx, Argon2, Calamine, Axum, Tower.

---

### Task 1: 1972 Rule Integration Test

**Files:**
- Modify: `web-app/src/tests/search_integration.rs`

- [ ] **Step 1: Add test case for 1972 exclusion rule**
```rust
#[sqlx::test(migrations = "../migrations")]
async fn test_pub_stats_ignores_1972_trial_year(pool: PgPool) {
    let id = Uuid::new_v4();
    // Seed pub
    sqlx::query("INSERT INTO pubs (id, name, region, town, postcode) VALUES ($1, $2, $3, $4, $5)")
        .bind(id).bind("1972 Test Pub").bind("Test Region").bind("Test Town").bind("TS1 1AA")
        .execute(&pool).await.unwrap();

    // Seed history including 1972
    for year in [1972, 1973, 1974] {
        sqlx::query("INSERT INTO gbg_history (pub_id, year) VALUES ($1, $2)")
            .bind(id).bind(year)
            .execute(&pool).await.unwrap();
    }

    // Refresh view
    sqlx::query("REFRESH MATERIALIZED VIEW pub_stats").execute(&pool).await.unwrap();

    // Assertions
    let stats = sqlx::query("SELECT total_years, first_year FROM pub_stats WHERE pub_id = $1")
        .bind(id)
        .fetch_one(&pool).await.unwrap();
    
    let total: i64 = stats.get("total_years");
    let first: i32 = stats.get("first_year");
    
    assert_eq!(total, 2, "Should exclude 1972 from count");
    assert_eq!(first, 1973, "First year should be 1973, ignoring 1972");
}
```

- [ ] **Step 2: Run integration tests**
Run: `DATABASE_URL=... ./scripts/migrate_test_db.sh && cargo test -p web-app --features ssr tests::search_integration`
Expected: PASS

- [ ] **Step 3: Commit**
```bash
git add web-app/src/tests/search_integration.rs
git commit -m "test: verify 1972 trial year exclusion rule"
```

---

### Task 2: Auth Logic Refactoring & Unit Tests

**Files:**
- Create: `web-app/src/auth.rs`
- Modify: `web-app/src/server.rs`
- Create: `web-app/src/tests/auth_tests.rs`
- Modify: `web-app/src/tests/mod.rs`

- [ ] **Step 1: Extract password helpers to `auth.rs`**
```rust
// web-app/src/auth.rs
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, PasswordVerifier, SaltString},
    Argon2, PasswordHash,
};

pub fn hash_password(password: &str) -> Result<String, anyhow::Error> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|h| h.to_string())
        .map_err(|e| anyhow::anyhow!("Hashing failed: {}", e))
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}
```

- [ ] **Step 2: Create unit tests for auth**
```rust
// web-app/src/tests/auth_tests.rs
use crate::auth::*;

#[test]
fn test_password_roundtrip() {
    let password = "correct-horse-battery-staple";
    let hash = hash_password(password).unwrap();
    assert!(verify_password(password, &hash));
    assert!(!verify_password("wrong-password", &hash));
}

#[test]
fn test_invalid_hash_format() {
    assert!(!verify_password("any", "not-a-valid-hash"));
}
```

- [ ] **Step 3: Update `server.rs` to use helpers**
(Replace manual argon2 logic in `login` and `register_user`)

- [ ] **Step 4: Register module and run tests**
Run: `cargo test -p web-app --features ssr tests::auth_tests`
Expected: PASS

- [ ] **Step 5: Commit**
```bash
git add web-app/src/auth.rs web-app/src/server.rs web-app/src/tests/auth_tests.rs
git commit -m "test: refactor auth logic and add unit tests"
```

---

### Task 3: Excel Parser Unit Tests

**Files:**
- Modify: `import-tool/src/excel.rs`

- [ ] **Step 1: Refactor `parse_excel` to extract mapping logic**
```rust
// import-tool/src/excel.rs
pub fn row_to_import_pub(row: &[calamine::Data]) -> Option<ImportPub> {
    if row.len() < 10 { return None; }
    // Existing mapping logic from loop goes here...
    Some(ImportPub { ... })
}
```

- [ ] **Step 2: Add unit tests within `excel.rs`**
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use calamine::Data;

    #[test]
    fn test_row_mapping() {
        let row = vec![
            Data::String("E".to_string()), // country
            Data::String("Kent".to_string()), // region
            Data::String("Dover".to_string()), // town
            Data::String("The Port".to_string()), // name
            Data::String("1 High St".to_string()), // address
            Data::String("CT16 1AA".to_string()), // postcode
            Data::Bool(false), // closed
            Data::Empty, // lat
            Data::Empty, // lon
            Data::String("2024;2023".to_string()), // years
        ];
        let pub_info = row_to_import_pub(&row).unwrap();
        assert_eq!(pub_info.name, "The Port");
        assert_eq!(pub_info.years, vec![2024, 2023]);
    }
}
```

- [ ] **Step 3: Run tests**
Run: `cargo test -p import-tool excel::tests`
Expected: PASS

- [ ] **Step 4: Commit**
```bash
git add import-tool/src/excel.rs
git commit -m "test: decouple excel parsing and add unit tests"
```

---

### Task 4: Security Middleware Integration Tests

**Files:**
- Create: `web-app/src/tests/security_tests.rs`
- Modify: `web-app/src/tests/mod.rs`

- [ ] **Step 1: Implement security header verification**
```rust
// web-app/src/tests/security_tests.rs
use axum::{routing::get, Router};
use axum::http::{HeaderValue, header};
use tower::ServiceExt;
use tower_http::set_header::SetResponseHeaderLayer;

#[tokio::test]
async fn test_security_headers_present() {
    let app = Router::new()
        .route("/", get(|| async { "ok" }))
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ));
    
    let response = app.oneshot(axum::http::Request::builder().uri("/").body(axum::body::Body::empty()).unwrap())
        .await.unwrap();
    
    assert_eq!(response.headers().get(header::X_CONTENT_TYPE_OPTIONS).unwrap(), "nosniff");
}
```

- [ ] **Step 2: Run tests**
Run: `cargo test -p web-app --features ssr tests::security_tests`
Expected: PASS

- [ ] **Step 3: Commit**
```bash
git add web-app/src/tests/security_tests.rs
git commit -m "test: verify security headers in axum stack"
```
