# Spec: Comprehensive Testing and Security Enhancements

**Date**: 2026-03-28
**Status**: Draft
**Topic**: Testing Infrastructure, Unit/Integration/E2E Testing, and Security Hardening

## 1. Overview
This specification defines the infrastructure and implementation for a robust testing suite and a security-hardened environment for the GBG Data Explorer. It covers isolated database testing, unit tests for data exports, security middleware for HTTP headers, and end-to-end (E2E) verification of administrative and search features.

## 2. Goals
- Establish a clean, isolated test environment using Docker and SQLx.
- Refactor and unit test data export logic (JSON, CSV, Parquet).
- Harden the web application with mandatory security headers (CSP, HSTS, etc.).
- Verify administrative access control and critical UI flows via Playwright.

## 3. Architecture & Design

### 3.1 Test Infrastructure (Section 1)
- **Docker Integration**: A `docker-compose.test.yml` file will manage a fresh Postgres + PostGIS instance for tests.
- **SQLx Macros**: Tests will use `#[sqlx::test]` to automatically manage database creation, migrations, and transactional rollbacks.
- **Environment**: A `.env.test` file will define the `DATABASE_URL` for the test container.

### 3.2 Data Export Logic (Section 2)
- **Refactoring**: Move core formatting logic in `web-app/src/export.rs` into pure functions (e.g., `pub_list_to_csv(Vec<PubDetail>) -> String`).
- **Unit Tests**:
    - Verify header and row formatting for JSON, CSV, and Parquet.
    - Test escaping of special characters in pub names and addresses.
    - Confirm correct mapping of all 20+ fields in the export records.

### 3.3 Security Hardening (Section 3)
- **Middleware**: Implement a `tower-http` layer in `web-app/src/main.rs`.
- **Headers**:
    - `Content-Security-Policy`: Restrict to self and `unpkg.com` (Leaflet).
    - `Strict-Transport-Security`: Conditional on `LEPTOS_ENV=production`.
    - `X-Content-Type-Options: nosniff`.
    - `X-Frame-Options: DENY`.
    - `Referrer-Policy: strict-origin-when-cross-origin`.
- **CSRF**: Require a custom `X-GBG-Request` header for all state-changing API calls (POST, PUT, DELETE).

### 3.4 E2E Verification (Section 4)
- **Tooling**: Playwright (`web-app/end2end/tests/`).
- **Access Control Tests**:
    - Verify Guest/Regular User redirection from `/admin`.
    - Confirm Admin access to `/admin`.
- **Search & Filter Tests**:
    - Verify "Region", "Town", and "Outcode" filter accuracy.
    - Test "Open Only" toggle behavior.
- **Security Flow Tests**:
    - End-to-end TOTP setup and verification flow.
    - Verification of 2FA requirement during login.

## 4. Success Criteria
- [ ] `cargo test` passes for both `web-app` and `import-tool`.
- [ ] `cargo clippy` and `cargo fmt` are clean.
- [ ] Security headers are present in all HTTP responses (verified via `curl` or browser).
- [ ] All Playwright tests pass against a local development server.
- [ ] HSTS is NOT active in non-production environments.

## 5. Components & Data Flow
1. **Developer/CI** runs `make test`.
2. **Makefile** triggers `docker-compose -f docker-compose.test.yml up -d`.
3. **SQLx** migrates the test DB and executes Rust tests.
4. **Playwright** launches browser instances to verify the UI and security redirects.
