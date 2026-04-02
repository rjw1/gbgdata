# Spec: GitHub Action for Continuous Integration

**Date**: 2026-03-28
**Status**: Draft
**Topic**: CI/CD Automation via GitHub Actions

## 1. Overview
This specification defines a GitHub Action workflow to automate the verification of code quality, security, and functionality for every pull request. It leverages the existing `Makefile` and `docker-compose.test.yml` to ensure consistency between local development and CI environments.

## 2. Goals
- Automatically run linting (`clippy`, `fmt`) on every PR.
- Execute security placeholders and dependency scans.
- Run the full suite of Rust unit and integration tests in an isolated Docker environment.
- Provide fast feedback to developers via GitHub PR checks.

## 3. Architecture & Design

### 3.1 Workflow Configuration
- **File Location**: `.github/workflows/ci.yml`
- **Trigger**: `pull_request` on all branches.
- **Environment**: `ubuntu-latest`.
- **Concurrency**: Cancel in-progress runs for the same PR.

### 3.2 Job Steps
1. **Checkout**: Retrieve the repository source.
2. **Rust Toolchain**: Install stable Rust with `clippy` and `rustfmt`.
3. **Caching**: Cache `~/.cargo` and `target/` to minimize build times.
4. **Services**:
    - The workflow will use Docker Compose to spin up the Postgres + PostGIS container as defined in `docker-compose.test.yml`.
5. **Execution**:
    - `make lint`: Verify formatting and run clippy.
    - `make security`: Execute security checks.
    - `make test`: Run migrations and execute tests against the Docker database.

## 4. Success Criteria
- [ ] PRs cannot be merged unless the `CI` workflow passes.
- [ ] Build times are optimized via effective caching (target < 10 mins).
- [ ] The workflow correctly manages the Docker lifecycle (up/down).
- [ ] Failures in any stage (lint, test) are clearly reported in the GitHub UI.

## 5. Components & Data Flow
1. Developer pushes to a PR branch.
2. GitHub triggers `ci.yml`.
3. Runner sets up the environment and restores cache.
4. `make test` starts the DB, runs `migrate_test_db.sh`, and executes `cargo test`.
5. Results are reported back to the PR.
