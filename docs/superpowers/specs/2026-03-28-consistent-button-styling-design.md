# Design: Consistent Semantic Button System

This document outlines the strategy for standardizing button styles across the GBG Data Explorer application using a semantic class system.

## 1. Goal
Ensure visual consistency for all interactive elements (buttons and button-like links) by replacing fragmented, context-specific styles with a global semantic system.

## 2. Core Button System

### 2.1 Base Styles (`.btn`)
All buttons will inherit from a base `.btn` class that defines:
- **Spacing**: Consistent padding and height.
- **Typography**: Font size, weight, and family.
- **Interactions**: Transitions for hover/active states, and disabled styling.
- **Layout**: `inline-flex` with center alignment and gap for icons.

### 2.2 Semantic Variants

| Class | Intent | Visuals |
| :--- | :--- | :--- |
| `.btn-primary` | Main actions (Save, Search, Login) | Forest Green background, White text |
| `.btn-secondary` | Accent actions (GPS, Suggest, Export) | Amber background, Dark Green text |
| `.btn-danger` | Destructive actions (Delete, Remove) | Error Red background, White text |
| `.btn-ghost` | Subtle actions (Cancel, Back, Close) | Bordered or transparent, Forest Green text |

### 2.3 Utility Variants

| Class | Purpose |
| :--- | :--- |
| `.btn-sm` | Smaller buttons for tables or dense UI |
| `.btn-block` | Full-width buttons for forms or mobile |

## 3. Migration Map

| Current Class/Usage | New Classes |
| :--- | :--- |
| `.search-btn` | `.btn .btn-primary` |
| `.gps-btn`, `.location-btn` | `.btn .btn-secondary` |
| `.save-btn` | `.btn .btn-primary` |
| `.cancel-btn`, `.back-btn` | `.btn .btn-ghost` |
| `.delete-btn` | `.btn .btn-danger` |
| `.suggest-btn`, `.edit-btn` | `.btn .btn-secondary` |
| `.add-btn` | `.btn .btn-secondary` |
| `.export-btn`, `.export-link` | `.btn .btn-secondary` |
| `.bulk-toggle-btn` | `.btn .btn-secondary` |
| `.close-btn` (modals) | `.btn .btn-ghost` |
| Login/Form buttons | `.btn .btn-primary .btn-block` |

## 4. Implementation Steps

1.  **Update `main.scss`**: 
    - Define the new `.btn` system.
    - Remove redundant specific button classes.
    - Consolidate color logic into the semantic variants.
2.  **Update Components**:
    - Iterate through all components and update `class="..."` to use the new semantic classes.
    - Ensure `<a>` tags used as buttons also get the `.btn` classes.
3.  **Verification**:
    - Run `cargo check --features ssr,hydrate` to ensure no build regressions.
    - Manually verify the visual consistency of buttons in major views.
