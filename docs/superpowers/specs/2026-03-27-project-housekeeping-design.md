# Design Spec: Project Housekeeping & Quality

Establish a solid foundation for the GBG Data project by cleaning the Git repository, formalizing documentation, and defining a testing strategy.

## 1. Git Repository Cleanup & Integrity

The repository currently contains or has historically tracked large data files and generated build artifacts that bloat the history and create noise in diffs.

### Strategy: Deep History Scrub

- **Target Files/Patterns**:
  - `*.xlsx` (Excel source data)
  - `*.sql` (Database dumps, **excluding** the `migrations/` directory)
  - `pubs.json` (Intermediate data exports, ~14MB)
  - `*.log` (Geocoding and process logs)
  - `site/pkg/` (Generated artifacts including large `web-app.wasm` files, ~10-13MB each)
- **Action**: Use `git filter-repo` (preferred) or `git filter-branch` to rewrite the history and permanently remove these patterns from all commits.
- **Gitignore Update**: Formalize these exclusions in the root `.gitignore` to prevent re-introduction.

## 2. Documentation Architecture

Transition from a template-based setup to a project-specific documentation suite.

### Root `README.md`

- High-level project purpose: Visualizing and analyzing Good Beer Guide (GBG) data.
- System overview: `import-tool` (Rust/Postgres) and `web-app` (Leptos/Axum).
- Quick-start pointers to specialized docs.

### Usage Guide (`docs/usage.md`)

- **Importing Data**: Steps to run the `import-tool`, required environment variables (DB connection, Nominatim URL), and source file expectations.
- **Web App**: Development workflow (`cargo leptos watch`) and production builds.

### Hosting & Deployment (`docs/hosting.md`)

- **TrueNAS/Docker**: Configuration for running as a containerized app on TrueNAS SCALE (using the provided `docker-compose.yml` as a base).
- **Optional Services**: Logic for making Nominatim (geocoding) optional in hosted environments to reduce resource usage.

### Architectural Decision Records (`docs/adr/`)

Initialize the ADR log to track significant technical choices:

- `0001-record-architecture-decisions.md`: Defining the ADR process.
- `0002-technology-stack.md`: Justifying Rust (Leptos/Axum/Sqlx) and Postgres.
- `0003-data-import-strategy.md`: How we handle Excel to Postgres mapping.

## 3. Testing & Validation Strategy

Establish a "Secure by Design" and "High Quality" workflow as per dxw standards.

### `import-tool` Tests

- **Unit Tests**: Focus on `parsers.rs` (Excel row to internal models) and `geocoder.rs` logic.
- **Integration Tests**: Use `sqlx`'s `#[sqlx::test]` to verify database migrations and CRUD operations against a real Postgres instance.

### `web-app` Tests

- **Component Tests**: Unit tests for Leptos components in `src/components/`.
- **End-to-End**: Expand the existing Playwright suite in `end2end/` to cover critical user paths (e.g., searching for a pub, viewing a region).

## 4. Success Criteria

- [ ] Git repository size significantly reduced.
- [ ] No generated or sensitive data in Git history.
- [ ] Complete documentation for a new developer to set up the project from scratch.
- [ ] Passing test suite with >70% coverage for core data logic.
