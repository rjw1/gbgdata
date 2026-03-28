# Fix Hydration Mismatch Warnings Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Resolve hydration mismatch warnings in Leptos 0.8 by ensuring all `user.get()` calls within `view!` macros are wrapped in `<Suspense/>` boundaries.

**Architecture:** Wrap conditional UI elements or entire form sections that depend on the `user` resource in `<Suspense fallback=|| ()>` to coordinate client-side hydration with server-rendered HTML.

**Tech Stack:** Rust (Leptos 0.8).

---

### Task 1: Fix `pub_list.rs`

**Files:**
- Modify: `web-app/src/components/pub_list.rs`

- [ ] **Step 1: Wrap Bulk Edit toggle in Suspense**
Wrap the `<Show>` component that checks user roles in a `<Suspense>`.

```rust
// Around line 73
<Suspense fallback=|| ()>
    <Show when=move || matches!(user.get(), Some(Ok(Some(ref u))) if u.role == "admin" || u.role == "owner")>
        <button class="btn btn-secondary" on:click=move |_| set_bulk_mode.update(|b| *b = !*b)>
            {move || if bulk_mode.get() { "Cancel Bulk Edit" } else { "Bulk Edit" }}
        </button>
    </Show>
</Suspense>
```

- [ ] **Step 2: Commit**
```bash
git add web-app/src/components/pub_list.rs
git commit -m "fix: wrap user resource read in pub_list.rs with Suspense"
```

---

### Task 2: Fix `pub_detail.rs`

**Files:**
- Modify: `web-app/src/components/pub_detail.rs`

- [ ] **Step 1: Wrap Admin Suggestion Banner in Suspense**
Ensure the admin banner check is wrapped (it currently has a Suspense for `pending_suggestions_res`, but the `user.get()` check is outside it).

```rust
// Around line 114
<Suspense fallback=|| ()>
    <Show when=move || matches!(user.get(), Some(Ok(Some(ref u))) if u.role == "admin" || u.role == "owner")>
        <Suspense fallback=|| ()>
            // ... suggestions view ...
        </Suspense>
    </Show>
</Suspense>
```

- [ ] **Step 2: Wrap Header Actions in Suspense**
Wrap "Suggest Update" and "Edit" buttons.

```rust
// Around line 147
<Suspense fallback=|| ()>
    <Show when=move || matches!(user.get(), Some(Ok(Some(_))))>
        <button class="btn btn-secondary" on:click=move |_| set_show_suggest.set(true)>"Suggest Update"</button>
    </Show>
    <Show when=move || matches!(user.get(), Some(Ok(Some(ref u))) if u.role == "admin" || u.role == "owner")>
        <button class="btn btn-secondary" on:click=move |_| set_show_edit.set(true)>"Edit"</button>
    </Show>
</Suspense>
```

- [ ] **Step 3: Wrap Log Visit action in Suspense**
Wrap the "My Activity" section.

```rust
// Around line 223
<Suspense fallback=|| ()>
    <Show when=move || matches!(user.get(), Some(Ok(Some(_))))>
        // ... activity card ...
    </Show>
</Suspense>
```

- [ ] **Step 4: Commit**
```bash
git add web-app/src/components/pub_detail.rs
git commit -m "fix: wrap user resource reads in pub_detail.rs with Suspense"
```

---

### Task 3: Fix `setup_2fa.rs`

**Files:**
- Modify: `web-app/src/components/setup_2fa.rs`

- [ ] **Step 1: Wrap 2FA verify section in Suspense**
Wrap the entire form or the section reading `user.get()`.

```rust
// Around line 62
<Suspense fallback=|| ()>
    <div class="verify-section">
        <h3>"Verify Setup"</h3>
        // ... form ...
    </div>
</Suspense>
```

- [ ] **Step 2: Commit**
```bash
git add web-app/src/components/setup_2fa.rs
git commit -m "fix: wrap user resource read in setup_2fa.rs with Suspense"
```

---

### Task 4: Verify `admin.rs` and `profile.rs`

**Files:**
- Modify: `web-app/src/components/admin.rs`
- Modify: `web-app/src/components/profile.rs`

- [ ] **Step 1: Check `admin.rs`**
Verify the main `<Transition>` correctly encapsulates the resource read. If needed, wrap internal matches.

- [ ] **Step 2: Check `profile.rs`**
Verify the existing `<Suspense>` encapsulates all resource reads.

- [ ] **Step 3: Commit (if changes made)**
```bash
git add web-app/src/components/admin.rs web-app/src/components/profile.rs
git commit -m "fix: ensure user resource reads in admin and profile are wrapped"
```

---

### Task 5: Final Build Check

- [ ] **Step 1: Build check**
Run: `cd web-app && cargo check --features ssr && cargo check --features hydrate`
Expected: No errors.

- [ ] **Step 2: Final Commit**
```bash
git commit --allow-empty -m "fix: completed hydration mismatch fixes"
```
