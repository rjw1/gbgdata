# Design Spec: Data Model & Stats Enhancements

Update the core data model to support Country data, rename "County" to "Region", and refine the statistics logic to handle historical data nuances (1972 trial year).

## 1. Geographic Model Update

### 1.1 Region vs. County
To better align with the Good Beer Guide's organizational structure, all references to "County" will be renamed to "Region". This affects:
- **Database Schema**: `pubs.county` -> `pubs.region`.
- **UI Labels**: Breadcrumbs, headers, and filters will now display "Region".
- **URL Structure**: `/explore/:county` will be maintained for backward compatibility (via aliases) but the primary label will be "Region".

### 1.2 Country Support
A new `country_code` field will be added to the `pubs` table to capture the country (England, Scotland, Wales, etc.) from the source Excel data.
- **Source**: Column 0 of the Excel sheet contains the country code.
- **Data Type**: `VARCHAR(10)` (e.g., 'E', 'S', 'W', 'NIR', 'IOM', 'CI').

## 2. Statistics Logic Refinement (1972 Data)

### 2.1 The "Trial Year" Problem
The 1972 Good Beer Guide was a trial run. While the data is valuable for research, it should not be included in "Appearance" or "Streak" calculations to maintain consistency with official GBG records.

### 2.2 Exclusion Strategy
- **Aggregations**: All server-side functions and SQL views (e.g., `pub_stats_view`) that calculate `total_appearances` or `current_streak` must filter out the year 1972.
- **Display**: The 1972 entry will still appear in the "History" list on the pub detail page but will be clearly marked as a "Trial Year" and excluded from the total count.

## 3. Implementation Details

### 3.1 Database Migrations
- Rename `county` to `region`.
- Add `country_code` to the `pubs` table.
- Update `pub_stats_view` to include the `year != 1972` filter.

### 3.2 Import Tool
- Update `excel.rs` and `db.rs` to map the first column (Country) and the renamed Region column correctly.

## 4. Testing Strategy
- **Data Validation**: Verify that 1972 entries do not increment the appearance count.
- **Rename Audit**: Ensure no "County" strings remain in the user-facing UI.
- **Import Verification**: Check that country codes are correctly captured for a sample of pubs.
