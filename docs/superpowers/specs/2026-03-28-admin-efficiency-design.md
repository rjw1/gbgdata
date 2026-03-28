# Design Spec: Admin Efficiency (Reporting, Bulk Editing, Suggestions)

**Date:** 2026-03-28
**Status:** Draft
**Topic:** Tools for identifying data gaps, batch updating, and community suggestion triage.

---

## 1. Objective
Streamline administrative tasks by providing tools to quickly identify missing data, manage community-submitted updates, and perform bulk changes across multiple pubs.

## 2. User Interface Designs

### 2.1 Missing Data Dashboard (in `/admin`)
A tabbed interface within the Admin Dashboard to identify pubs needing attention.
- **Tab 1: Coordinates**: Pubs where `lat` or `lon` is null.
- **Tab 2: External IDs**: Pubs missing at least one of (WhatPub ID, Google Maps ID, Untappd ID).
- **Tab 3: Inconsistencies**: Pubs marked `closed = true` but with GBG inclusions in the last 2 guide years.
- **Actions**: Each row includes an "Edit" button to open the `EditPub` modal.

### 2.2 Community Suggestion Triage
- **Global Queue (`/admin`)**: A table of all pending suggestions showing Pub Name, Submitting User, and Date.
- **Pub Detail Banner**: If a pub has a pending suggestion, show a prominent notice to admins on the `PubDetail` page.
- **Review Modal**:
    - Show side-by-side comparison of current data vs. suggested data.
    - Buttons: "Approve" (Apply changes), "Reject", "Modify & Approve".

### 2.3 Bulk Editing Tool
Available on Region, Town, and Outcode pub lists for authenticated admins.
- **Activation**: "Bulk Edit Mode" toggle button.
- **Selection**: Checkboxes appear next to each pub row.
- **Action Bar**: Fixed header with:
    - "{Count} Selected"
    - "Mark as Closed/Open" dropdown.
    - "Add/Remove GBG Year" dropdown.
    - "Change Region/Town" dropdown.
    - "Apply to All" button with confirmation.

## 3. Data Flow & Integration

### 3.1 Server Functions
- **`GetMissingDataReports(report_type)`**: **To be implemented.** Returns `Vec<PubSummary>`.
- **`GetPendingSuggestions()`**: **To be implemented.** Returns `Vec<SuggestedUpdate>`.
- **`ProcessSuggestedUpdate(id, action, final_data)`**: **To be implemented.** Updates pub and suggestion status.
- **`BulkUpdatePubs(ids, changes)`**: **To be implemented.** Performs batch updates and logs audits for each.

### 3.2 Audit & Logging
Every modification (whether from a suggestion or bulk edit) MUST create an `audit_log` entry for each pub changed, identifying the admin and the specific fields modified.

## 4. Implementation Phases
1. **Phase 1**: Implement Missing Data Dashboard tabs in `/admin`.
2. **Phase 2**: Build the Community Suggestion triage queue and review modal.
3. **Phase 3**: Add the "Pending Suggestion" banner to `PubDetail`.
4. **Phase 4**: Implement the Bulk Editing UI and server-side batch logic.
