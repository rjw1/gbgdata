# Consistent Semantic Button System Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Standardize all buttons across the application using a semantic global class system (`.btn`, `.btn-primary`, etc.) to ensure visual consistency between public and administrative features.

**Architecture:** We will implement a tiered CSS system where a base `.btn` class handles layout and behavior, while semantic variant classes handle coloring. Existing context-specific button styles will be removed in favor of these global utilities.

**Tech Stack:** Rust (Leptos), SCSS.

---

### Task 1: Global Button Styles in SCSS

**Files:**
- Modify: `web-app/style/main.scss`

- [ ] **Step 1: Define the semantic button system**
Add the new `.btn` classes to the global scope in `main.scss`.

```scss
/* Semantic Button System */
.btn {
  padding: 0.6rem 1.25rem;
  font-size: 0.95rem;
  font-weight: bold;
  border: none;
  border-radius: 8px;
  cursor: pointer;
  transition: background-color 0.2s, opacity 0.2s, transform 0.1s;
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 0.5rem;
  text-decoration: none;
  font-family: inherit;
  line-height: 1.2;

  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  &:active:not(:disabled) {
    transform: translateY(1px);
  }
}

.btn-primary {
  background-color: var(--forest-green);
  color: var(--light-text);
  &:hover:not(:disabled) { background-color: color-mix(in srgb, var(--forest-green), black 20%); }
}

.btn-secondary {
  background-color: var(--amber);
  color: #1a3c34;
  &:hover:not(:disabled) { background-color: color-mix(in srgb, var(--amber), black 10%); }
}

.btn-danger {
  background-color: var(--error);
  color: white;
  &:hover:not(:disabled) { background-color: color-mix(in srgb, var(--error), black 20%); }
}

.btn-ghost {
  background-color: transparent;
  color: var(--forest-green);
  border: 1px solid var(--forest-green);
  &:hover:not(:disabled) { background-color: var(--track); }
}

.btn-sm {
  padding: 0.3rem 0.6rem;
  font-size: 0.8rem;
  border-radius: 4px;
}

.btn-block {
  width: 100%;
}
```

- [ ] **Step 2: Remove redundant button styles**
Remove `.search-btn`, `.gps-btn`, `.location-btn`, `.back-btn` (from login-form), and any other context-specific button styles that are now covered by the semantic system.

- [ ] **Step 3: Commit**
```bash
git add web-app/style/main.scss
git commit -m "style: define semantic global button system"
```

---

### Task 2: Update Search & Navigation Components

**Files:**
- Modify: `web-app/src/components/near_me.rs`
- Modify: `web-app/src/components/explorer.rs`
- Modify: `web-app/src/components/pub_list.rs`

- [ ] **Step 1: Update Near Me buttons**
Change `.location-btn` and `.gps-btn` to use semantic classes.

- [ ] **Step 2: Update Explorer buttons**
Update pagination or action buttons in `explorer.rs`.

- [ ] **Step 3: Update Pub List buttons**
Update `.bulk-toggle-btn` and bulk action buttons.

- [ ] **Step 4: Commit**
```bash
git add web-app/src/components/near_me.rs web-app/src/components/explorer.rs web-app/src/components/pub_list.rs
git commit -m "style: update search and list buttons to semantic system"
```

---

### Task 3: Update Detail & Action Components

**Files:**
- Modify: `web-app/src/components/pub_detail.rs`
- Modify: `web-app/src/components/log_visit.rs`
- Modify: `web-app/src/components/suggest_update.rs`
- Modify: `web-app/src/components/edit_pub.rs`

- [ ] **Step 1: Update Pub Detail buttons**
Update "Suggest Update", "Edit", and "Log Visit" buttons.

- [ ] **Step 2: Update Log Visit modal buttons**
Update "Save" and "Cancel" buttons.

- [ ] **Step 3: Update Suggest Update modal buttons**
Update "Submit" and "Back" buttons.

- [ ] **Step 4: Update Edit Pub modal buttons**
Update "Save", "Cancel", and "Helper" buttons.

- [ ] **Step 5: Commit**
```bash
git add web-app/src/components/pub_detail.rs web-app/src/components/log_visit.rs web-app/src/components/suggest_update.rs web-app/src/components/edit_pub.rs
git commit -m "style: update detail and action buttons to semantic system"
```

---

### Task 4: Update Auth & Profile Components

**Files:**
- Modify: `web-app/src/components/login.rs`
- Modify: `web-app/src/components/register.rs`
- Modify: `web-app/src/components/profile.rs`
- Modify: `web-app/src/components/setup_2fa.rs`

- [ ] **Step 1: Update Login buttons**
Update login, passkey, and back buttons. Use `.btn-block` where appropriate.

- [ ] **Step 2: Update Register/Profile/2FA buttons**
Standardize all buttons in auth-related screens.

- [ ] **Step 3: Commit**
```bash
git add web-app/src/components/login.rs web-app/src/components/register.rs web-app/src/components/profile.rs web-app/src/components/setup_2fa.rs
git commit -m "style: update auth and profile buttons to semantic system"
```

---

### Task 5: Update Admin & Misc Components

**Files:**
- Modify: `web-app/src/components/admin.rs`
- Modify: `web-app/src/components/my_visits.rs`

- [ ] **Step 1: Update Admin Dashboard buttons**
Standardize tab buttons (using `.btn-sm`), logout, and row actions (edit/delete).

- [ ] **Step 2: Update My Visits buttons**
Update export buttons.

- [ ] **Step 3: Commit**
```bash
git add web-app/src/components/admin.rs web-app/src/components/my_visits.rs
git commit -m "style: update admin and misc buttons to semantic system"
```

---

### Task 6: Final Verification

- [ ] **Step 1: Build check**
Run: `cd web-app && cargo check --features ssr && cargo check --features hydrate`
Expected: No errors.

- [ ] **Step 2: Final Commit**
```bash
git commit --allow-empty -m "style: completed consistent button styling refactor"
```
