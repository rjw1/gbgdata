# Spec: Extended Testing Suite and Security Verification

**Date**: 2026-03-28
**Status**: Draft
**Topic**: Comprehensive Logic Verification and Security Integration Testing

## 1. Overview
This specification extends the existing testing suite to cover critical business logic ("1972 Rule"), cryptographic operations (Password/Auth), complex data parsing (Excel), and live security header verification.

## 2. Goals
- Verify the mandatory exclusion of the 1972 trial year from all statistical views.
- Ensure authentication helpers are robust and independently tested.
- Decouple Excel parsing from file I/O to allow unit testing of mapping logic.
- Programmatically verify the application of security headers in the Axum response stack.

## 3. Architecture & Design

### 3.1 1972 Rule Integration (Section 1)
- **Implementation**: Add test cases to `web-app/src/tests/search_integration.rs`.
- **Logic**: Use `sqlx` to seed data across 1972-1974 and assert that the `pub_stats` materialized view correctly filters out 1972.

### 3.2 Auth Logic Refactoring (Section 2)
- **Refactoring**: Create pure functions in `web-app/src/auth.rs` for `hash_password` and `verify_password`.
- **Unit Tests**: Add `web-app/src/tests/auth_tests.rs` to verify success/failure paths for password hashing.
- **Server Integration**: Update `login` and `register_user` in `server.rs` to use these helpers.

### 3.3 Excel Parser Unit Tests (Section 3)
- **Refactoring**: Modify `import-tool/src/excel.rs` to extract `parse_row` and `parse_range` functions that operate on `calamine::Data` and `calamine::Range`.
- **Unit Tests**: Add tests within `excel.rs` to verify column-to-struct mapping, header skipping, and multi-year string parsing.

### 3.4 Security Middleware Verification (Section 4)
- **Implementation**: Create `web-app/src/tests/security_tests.rs`.
- **Logic**: Construct a mock Axum router with the security layers and use `tower::ServiceExt` to inspect response headers.
- **Environment Tests**: Verify `Strict-Transport-Security` is only present when `LEPTOS_ENV=production`.

## 4. Success Criteria
- [ ] `pub_stats` integration test accurately reflects the 1972 exclusion rule.
- [ ] Authentication unit tests cover 100% of password hashing/verification logic.
- [ ] Excel parsing logic is verified without requiring physical `.xlsx` files.
- [ ] Security headers are confirmed present in a mock Axum server response.

## 5. Components & Data Flow
1. **Developer** runs `make test`.
2. **Rust Test Runner** executes:
    - **Unit Tests**: Auth, Excel mapping.
    - **Integration Tests**: 1972 Rule (via SQLx), Security Headers (via mock Axum).
3. **Assertions** provide definitive proof of logic correctness.
