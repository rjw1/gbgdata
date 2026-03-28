# Community Suggestion Form Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a user-friendly, category-based suggestion form for logged-in users to propose updates to pub data.

**Architecture:** Create a new `SuggestUpdateModal` component with multiple view states. Integrate into `PubDetail`.

**Tech Stack:** Rust (Leptos 0.8.0, Axum, SQLx).

---

### Task 1: Basic Suggestion Button in PubDetail

**Files:**
- Modify: `web-app/src/components/pub_detail.rs`

- [ ] **Step 1: Add `show_suggest` signal and button**

```rust
let (show_suggest, set_show_suggest) = signal(false);

// In the view, near the Edit button
<Show when=move || matches!(user.get(), Some(Ok(Some(_))))>
    <button class="suggest-btn" on:click=move |_| set_show_suggest.set(true)>"Suggest Update"</button>
</Show>
```

- [ ] **Step 2: Commit**

```bash
git add web-app/src/components/pub_detail.rs
git commit -m "feat: add Suggest Update entry point to PubDetail"
```

---

### Task 2: Suggestion Modal UI (States & Menu)

**Files:**
- Create: `web-app/src/components/suggest_update.rs`
- Modify: `web-app/src/components/mod.rs`

- [ ] **Step 1: Implement basic Modal structure and Action Menu**

Define an enum for `SuggestionCategory` and a signal to track the active category.

- [ ] **Step 2: Register component in `mod.rs`**

- [ ] **Step 3: Commit**

```bash
git add web-app/src/components/suggest_update.rs web-app/src/components/mod.rs
git commit -m "feat: implement SuggestUpdateModal skeleton and menu"
```

---

### Task 3: Category Form Views

**Files:**
- Modify: `web-app/src/components/suggest_update.rs`

- [ ] **Step 1: Implement "Report Closed" view**
- [ ] **Step 2: Implement "Add/Fix Links" view**
- [ ] **Step 3: Implement "Location" and "History" views**
- [ ] **Step 4: Commit**

```bash
git add web-app/src/components/suggest_update.rs
git commit -m "feat: implement category-specific suggestion forms"
```

---

### Task 4: Submission Logic & Feedback

**Files:**
- Modify: `web-app/src/components/suggest_update.rs`
- Modify: `web-app/src/components/pub_detail.rs`

- [ ] **Step 1: Connect Submit button to `SuggestUpdate` server function**

Initialize local state from `PubDetail` data and apply changes before dispatching.

- [ ] **Step 2: Add success feedback and auto-close**

- [ ] **Step 3: Commit**

```bash
git add web-app/src/components/suggest_update.rs web-app/src/components/pub_detail.rs
git commit -m "feat: finalize suggestion submission and feedback"
```
