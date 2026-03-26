# Design Spec: Filtered Data Exports (CSV, JSON, Parquet)

## 1. Executive Summary
The **Filtered Data Exports** feature provides users with the ability to download Good Beer Guide pub data in three major formats: **CSV, JSON, and Parquet**. These exports support granular filtering by County, Town, and Postal Outcode, enabling both full guide backups and localized data extracts.

## 2. Goals & Success Criteria
- **Multi-Format Support:** Provide identical data across CSV, JSON, and Parquet formats.
- **Contextual Filtering:** Enable users to export subsets of data based on their current geographic view.
- **High Performance:** Use streaming serialization to handle the full 32,000+ record dataset efficiently.
- **Ease of Use:** Integrate export buttons directly into the hierarchy navigation pages.

## 3. Architecture & Endpoints
We will add three new GET endpoints to the Axum server:

| Format | URL | Content-Type |
| :--- | :--- | :--- |
| **JSON** | `/export/json` | `application/json` |
| **CSV** | `/export/csv` | `text/csv` |
| **Parquet** | `/export/parquet` | `application/vnd.apache.parquet` |

### 3.1 Query Parameters
All endpoints support the following optional parameters:
- `county`: String (e.g., `Surrey`)
- `town`: String (e.g., `Godalming`)
- `outcode`: String (e.g., `GU7`)

## 4. Data Schema
The exported data will be a flattened representation of the `PubDetail` model:
- **Core Info:** `id`, `name`, `address`, `town`, `county`, `postcode`, `closed`.
- **Spatial:** `lat`, `lon`.
- **Statistics:** `current_streak`, `last_5_years`, `last_10_years`, `total_years`.
- **History:** `years` (Array in JSON, comma-separated string in CSV).

## 5. Implementation Details
### 5.1 Streaming Fetch
- Use `sqlx` to stream rows from the database.
- Serialize each row immediately to the response body to minimize memory footprint.

### 5.2 Formats
- **JSON:** `serde_json` for serialization.
- **CSV:** `csv` crate for robust header and quoting management.
- **Parquet:** `arrow` and `parquet` crates to convert rows to record batches.

## 6. UI Integration
Contextual export buttons will be added to:
- **Explorer Home:** Full dataset exports.
- **County Dashboard:** County-filtered exports.
- **Location List:** Town/Outcode-filtered exports.

## 7. Testing Strategy
- **Filtering Logic:** Verify that `?county=X` returns only pubs from that county.
- **Format Validation:** Ensure the exported files are valid and readable by standard tools (Excel, Pandas, etc.).
- **Volume Test:** Verify that exporting the full 32,633 records completes without server timeout.
