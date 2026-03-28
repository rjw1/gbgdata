# User Engagement Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement the UI for user visit tracking, a personal history dashboard with map/list views, and visit data exports.

**Architecture:** Extend the Leptos web app with new components for visit logging and management, utilizing existing server functions for data persistence.

**Tech Stack:** Rust (Leptos 0.8.0, Axum, SQLx), Leaflet for mapping.

---

### Task 1: "My Activity" Section in PubDetail

**Files:**
- Modify: `web-app/src/components/pub_detail.rs`

- [ ] **Step 1: Update `PubDetail` to fetch visit status**

Add a resource to fetch the user's visit status for the current pub.

```rust
let visit_status = Resource::new(
    move || (id(), show_edit.get()), // Refresh on load or after edit/log
    move |(id, _)| async move {
        match id {
            Some(uuid) => crate::server::get_pub_visit_status(uuid).await,
            None => Ok(false), // Default to false if no ID
        }
    }
);
```

- [ ] **Step 2: Add the "My Activity" View**

Insert the section before `external-links`.

```rust
<Show when=move || matches!(user.get(), Some(Ok(Some(_))))>
    <div class="my-activity-card">
        <h3>"My Activity"</h3>
        <Suspense fallback=|| view! { <p>"Loading activity..."</p> }>
            {move || {
                visit_status.get().map(|res| {
                    let has_visited = res.unwrap_or(false);
                    view! {
                        <p>
                            {if has_visited { "You have visited this pub." } else { "You haven't logged a visit here yet." }}
                        </p>
                        <button class="log-visit-btn" on:click=move |_| set_show_log_visit.set(true)>
                            "Log Visit"
                        </button>
                    }
                })
            }}
        </Suspense>
    </div>
</Show>
```

- [ ] **Step 3: Define `show_log_visit` signal**

```rust
let (show_log_visit, set_show_log_visit) = signal(false);
```

- [ ] **Step 4: Commit**

```bash
git add web-app/src/components/pub_detail.rs
git commit -m "feat: add My Activity section to PubDetail"
```

---

### Task 2: Log Visit Modal

**Files:**
- Create: `web-app/src/components/log_visit.rs`
- Modify: `web-app/src/components/mod.rs`
- Modify: `web-app/src/components/pub_detail.rs`

- [ ] **Step 1: Implement `LogVisit` Component**

Create a modal form for logging a visit.

```rust
// web-app/src/components/log_visit.rs
use leptos::prelude::*;
use uuid::Uuid;
use crate::server::LogVisit;

#[component]
pub fn LogVisitModal(pub_id: Uuid, on_close: Callback<()>) -> impl IntoView {
    let log_action = ServerAction::<LogVisit>::new();
    let today = chrono::Local::now().format("%Y-%m-%d").to_string();
    let (date, set_date) = signal(today);
    let (notes, set_notes) = signal(String::new());

    let on_submit = move |ev: leptos::web_sys::SubmitEvent| {
        ev.prevent_default();
        log_action.dispatch(LogVisit {
            pub_id,
            visit_date: date.get(),
            notes: Some(notes.get()).filter(|s| !s.is_empty()),
        });
    };

    Effect::new(move |_| {
        if let Some(Ok(())) = log_action.value().get() {
            on_close.run(());
        }
    });

    view! {
        <div class="modal-overlay">
            <div class="modal-content">
                <h3>"Log Visit"</h3>
                <form on:submit=on_submit>
                    <div class="form-group">
                        <label>"Date"</label>
                        <input type="date" value=date on:input=move |ev| set_date.set(event_target_value(&ev)) required />
                    </div>
                    <div class="form-group">
                        <label>"Notes (Optional)"</label>
                        <textarea on:input=move |ev| set_notes.set(event_target_value(&ev))></textarea>
                    </div>
                    <div class="form-actions">
                        <button type="submit" disabled=log_action.pending()>"Save"</button>
                        <button type="button" on:click=move |_| on_close.run(())>"Cancel"</button>
                    </div>
                </form>
            </div>
        </div>
    }
}
```

- [ ] **Step 2: Register component and add to PubDetail**

```rust
// web-app/src/components/mod.rs
pub mod log_visit;
```

```rust
// web-app/src/components/pub_detail.rs
<Show when=move || show_log_visit.get()>
    <LogVisitModal pub_id=id().unwrap() on_close=Callback::new(move |_| set_show_log_visit.set(false)) />
</Show>
```

- [ ] **Step 3: Commit**

```bash
git add web-app/src/components/log_visit.rs web-app/src/components/mod.rs web-app/src/components/pub_detail.rs
git commit -m "feat: implement Log Visit modal"
```

---

### Task 3: "My Visits" Page - Basic Structure & Dashboard

**Files:**
- Create: `web-app/src/components/my_visits.rs`
- Modify: `web-app/src/components/mod.rs`
- Modify: `web-app/src/app.rs`

- [ ] **Step 1: Create `MyVisits` component and route**

```rust
// web-app/src/components/my_visits.rs
use leptos::prelude::*;
use crate::server::{get_user_visits, export_user_visits};

#[component]
pub fn MyVisits() -> impl IntoView {
    let visits = Resource::new(|| (), |_| get_user_visits());
    
    view! {
        <div class="my-visits-container">
            <h1>"My Visits"</h1>
            <Suspense fallback=|| view! { <p>"Loading dashboard..."</p> }>
                {move || visits.get().map(|res| {
                    match res {
                        Ok(v) => view! {
                            <div class="stats-grid">
                                <div class="stat-card">
                                    <h3>"Total Visits"</h3>
                                    <p>{v.len()}</p>
                                </div>
                                // ... Add more cards later ...
                            </div>
                        }.into_any(),
                        Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}
```

- [ ] **Step 2: Add route and Navigation link**

```rust
// web-app/src/app.rs
<A href="/my-visits">"My Visits"</A>
// ...
<Route path=path!("/my-visits") view=MyVisits/>
```

- [ ] **Step 3: Commit**

```bash
git add web-app/src/components/my_visits.rs web-app/src/app.rs web-app/src/components/mod.rs
git commit -m "feat: add basic My Visits page and navigation"
```

---

### Task 4: My Visits List & Map View Toggle

**Files:**
- Modify: `web-app/src/components/my_visits.rs`

- [ ] **Step 1: Implement View Toggle**

Add a signal for `view_mode` ("list" | "map").

- [ ] **Step 2: Implement List View table**

Render the `VisitRecord` list in a table.

- [ ] **Step 3: Implement Map View (Leaflet)**

Reuse Leaflet logic to plot visited pubs.

- [ ] **Step 4: Commit**

```bash
git add web-app/src/components/my_visits.rs
git commit -m "feat: implement List/Map toggle for My Visits"
```

---

### Task 5: Visit Exports

**Files:**
- Modify: `web-app/src/components/my_visits.rs`

- [ ] **Step 1: Add Export buttons**

Link the "Export" buttons to the `export_user_visits` server function (which returns a Parquet file as a base64 or similar, depending on implementation).

- [ ] **Step 2: Commit**

```bash
git add web-app/src/components/my_visits.rs
git commit -m "feat: add export functionality to My Visits"
```
