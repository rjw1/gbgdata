# Design Spec: Improved Stats Display

## 1. Executive Summary
The **Improved Stats Display** for the gbgdata site aims to transform the current text-based statistics into a highly visual, "Classic Pub" themed dashboard. It focuses on hero-ifying the **Current Streak** and using **SVG Percentage Rings** for historical consistency (5-year and 10-year counts).

## 2. Goals & Success Criteria
- **Hero-ify Streak:** Display the current streak as the most prominent number on the pub detail page.
- **Visual Consistency:** Implement two side-by-side circular progress rings for 5-year and 10-year guide inclusion counts.
- **Responsiveness:** Ensure the dashboard layout scales gracefully from desktop (horizontal) to mobile (stacked).
- **Thematic Alignment:** Maintain the "Classic Pub" aesthetic using Amber and Forest Green colors.

## 3. Architecture & Components
### 3.1 `StatRing` Component
A reusable Leptos component designed to render a circular progress ring.
- **Props:**
    - `value`: `i64` (Current count)
    - `max`: `i64` (Total years in window, e.g., 5 or 10)
    - `label`: `String` (Label text, e.g., "Last 10 Years")
- **Logic:** Calculates `stroke-dashoffset` for an SVG circle based on the `value / max` ratio.

### 3.2 `PubDetail` Integration
The existing `PubDetail` component will be updated to:
- Wrap the statistics in a new `.stats-dashboard` container.
- Display the `current_streak` as a large centered number.
- Render two `StatRing` instances for 5-year and 10-year stats.

## 4. UI & Styling
- **Dashboard Layout:** A flex or grid container that centers the hero stat and flanks it with the rings.
- **Colors:**
    - `Amber` (#ffbf00): Progress color for rings and hero number.
    - `Forest Green` (#1a3c34): Labels and borders.
    - `Track Grey` (#e9ecef): Background color for the inactive part of the rings.
- **Typography:** Bold, large font size for the hero streak.

## 5. Testing Strategy
- **Visual Verification:** Check rendering for various edge cases (0/10, 10/10).
- **Responsive Audit:** Verify stacking behavior on mobile screen widths.
- **Data Integrity:** Ensure the visual numbers match the underlying database records.
