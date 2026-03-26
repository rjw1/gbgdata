# Design Spec: Navigation Hierarchy (County/Town/Postcode)

## 1. Executive Summary
The **Navigation Hierarchy** feature adds a structured way for users to discover pubs by drilling down through geographic and postal categories. It replaces the simple list with a multi-level exploration experience: **County -> Town/Outcode -> Pub List**.

## 2. Goals & Success Criteria
- **Structured Discovery:** Enable users to browse the full guide by geographic hierarchy.
- **Hybrid Drill-down:** Allow users to choose between drilling by Town or Postal Outcode from the County page.
- **Clean URLs:** Use nested, human-readable URLs for all levels of the hierarchy.
- **SEO & Shareability:** Ensure every category page has a unique, bookmarkable URL.

## 3. Architecture & Routing
We will use **Nested Routing** in Leptos Router to manage the states:

| Level | URL Pattern | View Component |
| :--- | :--- | :--- |
| **All Counties** | `/explore` | `ExplorerHome` |
| **County Dashboard** | `/explore/:county` | `CountyDashboard` |
| **Town Pubs** | `/explore/:county/town/:town` | `LocationPubList` |
| **Outcode Pubs** | `/explore/:county/outcode/:outcode` | `LocationPubList` |

## 4. Components & Logic
### 4.1 `ExplorerHome`
- Fetches and displays a list of all unique counties.
- Shows the total pub count per county.

### 4.2 `CountyDashboard`
- Displays summary statistics for the selected county.
- **Towns Section:** Lists all towns in the county with pub counts.
- **Outcodes Section:** Lists all postal outcodes (first half of postcode) in the county.
- Links to the final `LocationPubList` for each selection.

### 4.3 `LocationPubList`
- A reusable list view that filters pubs based on either the `town` or `outcode` route parameters.
- Inherits the `county` from the parent route for context.

### 4.4 Outcode Calculation
- In SQL: `SPLIT_PART(postcode, ' ', 1)` will be used to group and filter by outcode.

## 5. UI & Styling
- **Grid Layout:** Use alphabetical grids for counties and towns.
- **Breadcrumbs:** Implement a breadcrumb component for easy upward navigation.
- **Search Integration:** Maintain the global search as a secondary way to find locations quickly.

## 6. Testing Strategy
- **Routing Verification:** Ensure all nested paths resolve to the correct components.
- **Data Accuracy:** Verify that pub counts on category pages match the actual lists.
- **Edge Cases:** Handle counties or towns with special characters or spaces in their names (URL encoding).
