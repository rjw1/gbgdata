# Hosting & Optional Nominatim Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make the Nominatim geocoder optional and optimize the app for hosting on TrueNAS SCALE.

**Architecture:**
1. **Conditional Geocoding**: Update `import-tool` and `web-app` to check for an `OPTIONAL_NOMINATIM` environment variable or missing URL.
2. **TrueNAS Config**: Finalize the `docker-compose.yml` and provide a `.env.example`.

**Tech Stack:** Rust, Docker, TrueNAS SCALE.

---

### Task 1: Optional Nominatim in import-tool

**Files:**
- Modify: `import-tool/src/geocoder.rs`, `import-tool/src/main.rs`

- [ ] **Step 1: Update Geocoder logic**

Modify `import-tool/src/geocoder.rs` to handle an empty or missing `NOMINATIM_URL` gracefully. If the URL is missing, return `Ok(None)` instead of an error.

```rust
pub async fn geocode(&self, ...) -> Result<Option<(f64, f64)>> {
    if self.url.is_empty() {
        return Ok(None);
    }
    // ... normal logic
}
```

- [ ] **Step 2: Update import loop**

In `import-tool/src/main.rs`, log a warning if geocoding is skipped because Nominatim is disabled.

- [ ] **Step 3: Commit import-tool changes**

---

### Task 2: Optional Nominatim in web-app (Server-side)

**Files:**
- Modify: `web-app/src/server.rs`

- [ ] **Step 1: Check environment variable**

In the server-side geocoding logic (if any), check `std::env::var("OPTIONAL_NOMINATIM")`.

- [ ] **Step 2: Gracefully handle disabled state**

Ensure the UI doesn't crash if geocoding fails or is disabled.

- [ ] **Step 3: Commit web-app changes**

---

### Task 3: TrueNAS SCALE Configuration

**Files:**
- Modify: `docker-compose.yml`
- Create: `.env.example`

- [ ] **Step 1: Finalize docker-compose.yml**

Ensure the `docker-compose.yml` is ready for TrueNAS SCALE with appropriate environment variables and volume mounts.

- [ ] **Step 2: Create .env.example**

Include all necessary configuration options:
- `DATABASE_URL`
- `NOMINATIM_URL` (Optional)
- `OPTIONAL_NOMINATIM`

- [ ] **Step 3: Commit Hosting Config**
