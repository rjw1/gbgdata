# GitHub Action CI Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Automate code quality, security, and functional testing for every pull request via GitHub Actions.

**Architecture:** A monolithic job running on `ubuntu-latest` that executes our `Makefile` targets. It manages a Docker Postgres instance for integration tests.

**Tech Stack:** GitHub Actions, Rust, Docker, Docker Compose.

---

### Task 1: Create CI Workflow

**Files:**
- Create: `.github/workflows/ci.yml`

- [ ] **Step 1: Create the workflow directory**
```bash
mkdir -p .github/workflows
```

- [ ] **Step 2: Implement the CI workflow**
```yaml
name: CI

on:
  pull_request:
    branches: [ main, master ]

concurrency:
  group: ${{ github.workflow }}-${{ github.ref }}
  cancel-in-progress: true

jobs:
  test:
    name: Lint, Security, and Test
    runs-on: ubuntu-latest
    
    steps:
      - name: Checkout code
        uses: actions/checkout@v4

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt

      - name: Rust Cache
        uses: Swatinem/rust-cache@v2

      - name: Setup Docker
        uses: docker/setup-buildx-action@v3

      - name: Install dependencies
        run: |
          sudo apt-get update
          sudo apt-get install -y postgresql-client

      - name: Run Lint
        run: make lint

      - name: Run Security Placeholder
        run: make security

      - name: Run Tests
        run: make test
        env:
          DATABASE_URL: postgres://test_user:test_password@localhost:5433/gbgdata_test
```

- [ ] **Step 3: Verify yaml syntax**
Run: `yamllint .github/workflows/ci.yml` (if available) or check via online validator.
Expected: Valid YAML.

- [ ] **Step 4: Commit**
```bash
git add .github/workflows/ci.yml
git commit -m "ci: add github action workflow for automated testing"
```
