# Project Housekeeping & Quality Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Clean up the git repository history, formalize project documentation, and establish a testing foundation.

**Architecture:** 
1. **Git Cleanup**: Rewrite history to purge large data/generated files while preserving migrations.
2. **Docs**: Create a structured documentation suite (README, Usage, Hosting, ADRs).
3. **Testing**: Implement initial unit tests for data parsing logic and setup DB integration testing.

**Tech Stack:** Git, Rust (Cargo), Leptos, Sqlx, Markdown.

---

### Task 1: Git History Cleanup & Repository Shrinking

**Files:**
- Modify: `.gitignore`
- Run: `git filter-branch` and repository maintenance commands.

- [ ] **Step 1: Update .gitignore**

Add data files and generated site artifacts to the root `.gitignore`.

```gitignore
# Data and Logs
*.xlsx
*.sql
pubs.json
*.log
!migrations/*.sql

# Web App Artifacts
site/pkg/
target/
```

- [ ] **Step 2: Commit .gitignore change**

```bash
git add .gitignore
git commit -m "chore: update gitignore for data and generated artifacts"
```

- [ ] **Step 3: Deep Clean History**

Run `git filter-repo` to remove the targeted files from all commits.
*Warning: This changes all commit hashes. --force is required because we have untracked changes.*

```bash
git filter-repo --invert-paths \
  --path "GBG counties one sheet Duncan 2025.xlsx" \
  --path "gbgdata_dump.sql" \
  --path "pubs.json" \
  --path-glob "*.log" \
  --path "site/pkg/" \
  --force
```

- [ ] **Step 4: Shrink Repository Size**

Purge the old objects and optimize the database.

```bash
git reflog expire --expire=now --all
git gc --prune=now --aggressive
```

- [ ] **Step 5: Verify size reduction**

Run: `du -sh .git`
Expected: Significant reduction (from ~60MB+ to <10MB).

---

### Task 2: Core Project Documentation

**Files:**
- Create: `README.md` (root), `docs/usage.md`, `docs/hosting.md`.

- [ ] **Step 1: Create Root README.md**

Provide a high-level overview of the project.

```markdown
# GBG Data Explorer

A tool for visualizing and analyzing Good Beer Guide (GBG) data over time.

## Project Structure

- `import-tool/`: Rust utility to parse Excel data and import into Postgres.
- `web-app/`: Leptos/Axum web application for browsing and filtering pub data.
- `migrations/`: SQL database schema and views.

## Documentation

- [Usage Guide](docs/usage.md) - How to import data and run the apps.
- [Hosting Guide](docs/hosting.md) - Deployment and TrueNAS setup.
- [Architecture Decisions](docs/adr/) - Technical design records.
```

- [ ] **Step 2: Create Usage Guide (docs/usage.md)**

```markdown
# Usage Guide

## Importing Data

1. Place the source Excel file in the root directory.
2. Set your `.env` variables (see `.env.example`).
3. Run the import tool:
   ```bash
   cargo run -p import-tool
   ```

## Running the Web App

```bash
cd web-app
cargo leptos watch
```
```

- [ ] **Step 3: Create Hosting Guide (docs/hosting.md)**

```markdown
# Hosting & Deployment

## TrueNAS SCALE

The project includes a `docker-compose.yml` for deployment as a Custom App on TrueNAS.

## Configuration

- `DATABASE_URL`: Postgres connection string.
- `NOMINATIM_URL`: (Optional) URL for a Nominatim geocoding instance.
- `OPTIONAL_NOMINATIM`: Set to `true` to disable external geocoding calls in the web UI.
```

- [ ] **Step 4: Commit Documentation**

```bash
git add README.md docs/
git commit -m "docs: initialize core project documentation"
```

---

### Task 3: Architectural Decision Records (ADRs)

**Files:**
- Create: `docs/adr/0001-record-architecture-decisions.md`, `docs/adr/0002-technology-stack.md`.

- [ ] **Step 1: Create ADR 0001 (Process)**

```markdown
# 1. Record architecture decisions

Date: 2026-03-27

## Status

Accepted

## Context

We need a way to record significant technical decisions.

## Decision

We will use Architectural Decision Records (ADRs) stored in `docs/adr/`.

## Consequences

Technical choices are documented for future maintainers.
```

- [ ] **Step 2: Create ADR 0002 (Tech Stack)**

```markdown
# 2. Technology Stack

Date: 2026-03-27

## Status

Accepted

## Context

The project requires high-performance data processing and a reactive web UI.

## Decision

- **Language**: Rust (Type safety, performance).
- **Web Framework**: Leptos (Full-stack SSR/Hydration).
- **Database**: Postgres (Relational queries, spatial extensions).
- **ORM/Query**: Sqlx (Compile-time SQL verification).

## Consequences

High developer productivity with strong reliability guarantees.
```

- [ ] **Step 3: Commit ADRs**

```bash
git add docs/adr/
git commit -m "docs: initialize ADR log"
```

---

### Task 4: Testing Foundation (import-tool)

**Files:**
- Modify: `import-tool/src/parsers.rs`
- Create: `import-tool/tests/parser_tests.rs`

- [ ] **Step 1: Add unit tests for parsers**

Add a test for basic row parsing logic in `import-tool/src/parsers.rs`.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing_logic() {
        // Add specific test case based on excel.rs structures
    }
}
```

- [ ] **Step 2: Run tests**

Run: `cargo test -p import-tool`
Expected: PASS

- [ ] **Step 3: Commit**

```bash
git add import-tool/src/parsers.rs
git commit -m "test: add initial unit tests for data parsing"
```
