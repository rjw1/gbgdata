# Comprehensive Testing and Security Enhancements Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Establish a clean, isolated test environment, refactor export logic for unit testing, harden the application with security headers, and verify core features with E2E tests.

**Architecture:** We use `docker-compose.test.yml` for an isolated Postgres instance and `#[sqlx::test]` for automated test database management. Export logic is decoupled from Axum handlers into pure functions. A `tower-http` layer is added for security headers.

**Tech Stack:** Rust, Leptos, Axum, SQLx, Playwright, Docker, Tower-HTTP.

---

### Task 1: Test Infrastructure Setup

**Files:**
- Create: `docker-compose.test.yml`
- Create: `.env.test`
- Modify: `Makefile`

- [ ] **Step 1: Create `docker-compose.test.yml`**
```yaml
services:
  test-db:
    image: postgis/postgis:15-3.3
    environment:
      POSTGRES_USER: test_user
      POSTGRES_PASSWORD: test_password
      POSTGRES_DB: gbgdata_test
    ports:
      - "5433:5432"
    tmpfs:
      - /var/lib/postgresql/data
```

- [ ] **Step 2: Create `.env.test`**
```bash
DATABASE_URL=postgres://test_user:test_password@localhost:5433/gbgdata_test
LEPTOS_ENV=test
```

- [ ] **Step 3: Update `Makefile` to use test environment**
```makefile
test:
	docker-compose -f docker-compose.test.yml up -d
	export DATABASE_URL=postgres://test_user:test_password@localhost:5433/gbgdata_test && \
	cd web-app && cargo test --features ssr && \
	cd ../import-tool && cargo test
	docker-compose -f docker-compose.test.yml down
```

- [ ] **Step 4: Verify Docker container starts**
Run: `docker-compose -f docker-compose.test.yml up -d`
Expected: Container `test-db` is running on port 5433.

- [ ] **Step 5: Commit**
```bash
git add docker-compose.test.yml .env.test Makefile
git commit -m "test: set up isolated docker test environment"
```

---

### Task 2: Refactor Export Logic for Unit Testing

**Files:**
- Modify: `web-app/src/export.rs`
- Create: `web-app/src/tests/export_tests.rs`

- [ ] **Step 1: Refactor `export.rs` to extract formatting logic**
```rust
// web-app/src/export.rs
pub fn pub_list_to_csv(data: Vec<PubDetail>) -> Result<Vec<u8>, csv::Error> {
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(true)
        .from_writer(Vec::new());

    for p in data {
        let years_str = p.years.iter().map(|y| y.to_string()).collect::<Vec<_>>().join(";");
        wtr.write_record([
            p.id.to_string(), p.name, p.address, p.town, p.region, p.postcode, 
            p.closed.to_string(), p.untappd_id.unwrap_or_default(), 
            p.google_maps_id.unwrap_or_default(), p.whatpub_id.unwrap_or_default(), 
            p.rgl_id.unwrap_or_default(), p.lat.map(|v| v.to_string()).unwrap_or_default(), 
            p.lon.map(|v| v.to_string()).unwrap_or_default(), p.current_streak.to_string(), 
            p.last_5_years.to_string(), p.last_10_years.to_string(), 
            p.total_years.to_string(), p.first_year.map(|v| v.to_string()).unwrap_or_default(), 
            p.latest_year.map(|v| v.to_string()).unwrap_or_default(), years_str,
        ])?;
    }
    wtr.into_inner()
}
```

- [ ] **Step 2: Create unit tests for CSV export**
```rust
// web-app/src/tests/export_tests.rs
#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::PubDetail;
    use uuid::Uuid;

    #[test]
    fn test_csv_formatting_with_special_characters() {
        let pub_data = vec![PubDetail {
            id: Uuid::new_v4(),
            name: "The Dog & Duck, London".to_string(),
            address: "123 Main \"Street\"".to_string(),
            // ... other fields ...
            ..Default::default()
        }];
        let csv = pub_list_to_csv(pub_data).unwrap();
        let csv_str = String::from_utf8(csv).unwrap();
        assert!(csv_str.contains("\"The Dog & Duck, London\""));
        assert!(csv_str.contains("\"123 Main \"\"Street\"\"\""));
    }
}
```

- [ ] **Step 3: Run unit tests**
Run: `cargo test -p web-app --features ssr tests::export_tests`
Expected: PASS

- [ ] **Step 4: Commit**
```bash
git add web-app/src/export.rs web-app/src/tests/export_tests.rs
git commit -m "test: refactor export logic and add unit tests"
```

---

### Task 3: Security Headers Middleware

**Files:**
- Modify: `web-app/src/main.rs`
- Modify: `web-app/Cargo.toml`

- [ ] **Step 1: Add `tower-http` with `set-header` feature**
Run: `cd web-app && cargo add tower-http --features set-header`

- [ ] **Step 2: Implement security headers layer in `main.rs`**
```rust
// web-app/src/main.rs
use tower_http::set_header::SetResponseHeaderLayer;
use ax_http::header;

let mut app = Router::new()
    // ... existing routes ...
    .layer(SetResponseHeaderLayer::if_not_present(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    ))
    .layer(SetResponseHeaderLayer::if_not_present(
        header::X_FRAME_OPTIONS,
        HeaderValue::from_static("DENY"),
    ))
    .layer(SetResponseHeaderLayer::if_not_present(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static("default-src 'self'; script-src 'self' unpkg.com; style-src 'self' 'unsafe-inline' unpkg.com; img-src 'self' data: *.tile.openstreetmap.org unpkg.com;"),
    ));

if std::env::var("LEPTOS_ENV").unwrap_or_default() == "production" {
    app = app.layer(SetResponseHeaderLayer::if_not_present(
        header::STRICT_TRANSPORT_SECURITY,
        HeaderValue::from_static("max-age=31536000; includeSubDomains"),
    ));
}
```

- [ ] **Step 3: Verify headers with curl**
Run: `curl -I http://localhost:3000` (after starting server)
Expected: `X-Content-Type-Options`, `X-Frame-Options`, and `Content-Security-Policy` headers are present. `Strict-Transport-Security` is absent.

- [ ] **Step 4: Commit**
```bash
git add web-app/Cargo.toml web-app/src/main.rs
git commit -m "security: add global security headers middleware"
```

---

### Task 4: Admin and Search E2E Tests

**Files:**
- Modify: `web-app/end2end/tests/example.spec.ts` (renaming to `gbg.spec.ts`)

- [ ] **Step 1: Implement Admin Access Control tests**
```typescript
test("unauthenticated user cannot access admin", async ({ page }) => {
  await page.goto("/admin");
  await expect(page).toHaveURL(/\/login/);
});

test("regular user cannot access admin", async ({ page }) => {
  // 1. Login as regular user
  // 2. Navigate to /admin
  // 3. Expect redirect or 403
});
```

- [ ] **Step 2: Implement Search and Filter tests**
```typescript
test("filter by region 'Kent' returns only Kent pubs", async ({ page }) => {
  await page.goto("/explore");
  await page.click('text="Kent"');
  // Verify all items in list have "Kent" as region
});

test("open only filter works", async ({ page }) => {
  await page.goto("/");
  await page.check('input[type="checkbox"]'); // Open Only
  // Verify no pubs with "Closed" label are visible
});
```

- [ ] **Step 3: Run E2E tests**
Run: `cd web-app/end2end && npx playwright test`
Expected: All tests pass (requires local DB with seed data).

- [ ] **Step 4: Commit**
```bash
git add web-app/end2end/tests/gbg.spec.ts
git commit -m "test: add admin access and search/filter E2E tests"
```

---

### Task 5: Search Filter Integration Tests (SQLx)

**Files:**
- Create: `web-app/src/tests/search_integration.rs`

- [ ] **Step 1: Implement SQLx integration test for region search**
```rust
#[sqlx::test(migrations = "../migrations")]
async fn test_get_pubs_by_location_filters_by_region(pool: PgPool) {
    // 1. Seed test data
    // 2. Call server function
    let results = crate::server::get_pubs_by_location(
        "Kent".to_string(), None, None, None, None, None
    ).await.unwrap();
    // 3. Assert all results are in Kent
    assert!(results.iter().all(|p| p.region == "Kent"));
}
```

- [ ] **Step 2: Run integration tests**
Run: `export DATABASE_URL=... && cargo test -p web-app --features ssr tests::search_integration`
Expected: PASS

- [ ] **Step 3: Commit**
```bash
git add web-app/src/tests/search_integration.rs
git commit -m "test: add SQLx integration tests for search filters"
```
