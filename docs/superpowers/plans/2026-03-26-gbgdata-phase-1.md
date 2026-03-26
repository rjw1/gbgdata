# gbgdata Phase 1: Tooling & Data Import Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create a Rust CLI tool to parse the Excel spreadsheet and seed a PostgreSQL database with pub data and historical inclusion records.

**Architecture:** A standalone Rust binary within a workspace that uses `calamine` for Excel parsing and `sqlx` for database interaction.

**Tech Stack:** Rust, PostgreSQL, PostGIS, `calamine`, `sqlx`, `tokio`, `uuid`.

---

## File Structure
- `Cargo.toml`: Workspace configuration.
- `import-tool/Cargo.toml`: Dependencies for the import tool.
- `import-tool/src/main.rs`: Entry point for the CLI.
- `import-tool/src/excel.rs`: Excel parsing logic.
- `import-tool/src/db.rs`: Database schema and insertion logic.
- `migrations/`: SQL migration files for Postgres/PostGIS.

## Tasks

### Task 1: Project Scaffolding
- [x] **Step 1: Create workspace Cargo.toml**
```toml
[workspace]
members = ["import-tool"]
resolver = "2"
```
- [x] **Step 2: Initialize import-tool**
Run: `cargo new import-tool`
- [x] **Step 3: Add dependencies to import-tool/Cargo.toml**
```toml
[package]
name = "import-tool"
version = "0.1.0"
edition = "2021"

[dependencies]
calamine = "0.24"
sqlx = { version = "0.7", features = ["runtime-tokio", "tls-rustls", "postgres", "uuid", "chrono"] }
tokio = { version = "1", features = ["full"] }
uuid = { version = "1", features = ["v4"] }
chrono = "0.4"
dotenvy = "0.15"
anyhow = "1.0"
```
- [x] **Step 4: Commit**
```bash
git add Cargo.toml import-tool/
git commit -m "chore: scaffold rust workspace and import-tool"
```

### Task 2: Database Migrations
- [ ] **Step 1: Create migrations directory**
Run: `mkdir migrations`
- [ ] **Step 2: Create initial migration for PostGIS and tables**
File: `migrations/20260326000000_initial_schema.sql`
```sql
CREATE EXTENSION IF NOT EXISTS postgis;

CREATE TABLE pubs (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(255) NOT NULL,
    address TEXT,
    town VARCHAR(100),
    county VARCHAR(100),
    postcode VARCHAR(20),
    closed BOOLEAN DEFAULT FALSE,
    location GEOGRAPHY(POINT, 4326),
    untappd_id VARCHAR(100),
    google_maps_id VARCHAR(255),
    whatpub_id VARCHAR(255),
    rgl_id VARCHAR(255),
    untappd_verified BOOLEAN DEFAULT FALSE,
    last_seen DATE DEFAULT CURRENT_DATE
);

CREATE TABLE gbg_history (
    id SERIAL PRIMARY KEY,
    pub_id UUID REFERENCES pubs(id) ON DELETE CASCADE,
    year INTEGER NOT NULL,
    UNIQUE(pub_id, year)
);
```
- [ ] **Step 3: Commit**
```bash
git add migrations/
git commit -m "feat: add initial database schema with postgis"
```

### Task 3: Excel Parsing Logic
- [ ] **Step 1: Define Pub struct and parsing function**
File: `import-tool/src/excel.rs`
```rust
use calamine::{Reader, Xlsx, open_workbook, DataType};
use anyhow::Result;

#[derive(Debug)]
pub struct RawPub {
    pub name: String,
    pub address: String,
    pub town: String,
    pub county: String,
    pub postcode: String,
    pub closed: bool,
    pub years: Vec<i32>,
}

pub fn parse_excel(path: &str) -> Result<Vec<RawPub>> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let mut pubs = Vec::new();
    // Logic to parse "GBG counties one sheet Duncan 2025.xlsx"
    // Assuming columns: County, Town, Name, Address, Postcode, Closed, [Years...]
    if let Some(Ok(range)) = workbook.worksheet_range("Sheet1") {
        for row in range.rows().skip(1) {
             // Mapping logic based on column indices
        }
    }
    Ok(pubs)
}
```
- [ ] **Step 2: Write a test for parsing**
File: `import-tool/src/excel.rs` (append)
```rust
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_placeholder() {
        // Test with a small sample if possible
    }
}
```
- [ ] **Step 3: Commit**
```bash
git add import-tool/src/excel.rs
git commit -m "feat: implement excel parsing logic"
```

### Task 4: Database Insertion Logic
- [x] **Step 1: Implement DB insertion**
File: `import-tool/src/db.rs`
```rust
use anyhow::Result;
use crate::excel::RawPub;
use sqlx::{PgPool, Postgres, Transaction};

/// Inserts a pub and its historical inclusion records into the database.
///
/// This operation is performed within a transaction to ensure that either both the pub
/// and its history are recorded, or nothing is recorded in case of an error.
pub async fn insert_pub(pool: &PgPool, raw_pub: &RawPub) -> Result<()> {
    let mut tx: Transaction<'_, Postgres> = pool.begin().await?;

    // Insert the pub and get the generated UUID
    let pub_id: uuid::Uuid = sqlx::query_scalar(
        "INSERT INTO pubs (name, address, town, county, postcode, closed) 
         VALUES ($1, $2, $3, $4, $5, $6) 
         RETURNING id",
    )
    .bind(&raw_pub.name)
    .bind(&raw_pub.address)
    .bind(&raw_pub.town)
    .bind(&raw_pub.county)
    .bind(&raw_pub.postcode)
    .bind(raw_pub.closed)
    .fetch_one(&mut *tx)
    .await?;

    // Insert each year the pub was in the Good Beer Guide
    for year in &raw_pub.years {
        sqlx::query(
            "INSERT INTO gbg_history (pub_id, year) 
             VALUES ($1, $2) 
             ON CONFLICT (pub_id, year) DO NOTHING",
        )
        .bind(pub_id)
        .bind(year)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}
```
- [x] **Step 2: Commit**
```bash
git add import-tool/src/db.rs
git commit -m "feat: improve code quality of database insertion logic"
```

### Task 5: Main CLI Execution
- [ ] **Step 1: Implement main loop**
File: `import-tool/src/main.rs`
```rust
mod excel;
mod db;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")?;
    let pool = PgPoolOptions::new().max_connections(5).connect(&database_url).await?;

    let pubs = excel::parse_excel("GBG counties one sheet Duncan 2025.xlsx")?;
    for p in pubs {
        db::insert_pub(&pool, &p).await?;
    }
    println!("Import complete!");
    Ok(())
}
```
- [ ] **Step 2: Commit**
```bash
git add import-tool/src/main.rs
git commit -m "feat: finalize import-tool main execution"
```
