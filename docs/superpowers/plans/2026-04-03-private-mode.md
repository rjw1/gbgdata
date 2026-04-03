# Private Mode Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Restrict site access to logged-in users only, with an environment variable override and a database-backed toggle for the owner.

**Architecture:** Hybrid configuration where `PRIVATE_MODE=true` in `.env` hard-locks the site to private, otherwise it falls back to a `site_settings` table. Access is enforced via Axum middleware and Leptos router redirects.

**Tech Stack:** Rust (Leptos, Axum, SQLx), Postgres, CSS.

---

### Task 1: Database Migration

**Files:**
- Create: `migrations/20260403000000_site_settings.sql`

- [ ] **Step 1: Create the migration file**

```sql
-- migrations/20260403000000_site_settings.sql
CREATE TABLE site_settings (
    id SERIAL PRIMARY KEY,
    private_mode BOOLEAN NOT NULL DEFAULT FALSE,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_by UUID REFERENCES users(id)
);

-- Initialize with default public state
INSERT INTO site_settings (private_mode) VALUES (FALSE);
```

- [ ] **Step 2: Run migrations**

Run: `cd web-app && sqlx migrate run` (or use the existing project-specific migration command if different).

- [ ] **Step 3: Commit**

```bash
git add migrations/20260403000000_site_settings.sql
git commit -m "db: add site_settings table for private mode"
```

---

### Task 2: Models & Server Functions (Data Access)

**Files:**
- Modify: `web-app/src/models.rs`
- Modify: `web-app/src/server.rs`

- [ ] **Step 1: Add SiteSettings model**

Add to `web-app/src/models.rs`:
```rust
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SiteSettings {
    pub private_mode: bool,
    pub is_hard_locked: bool, // Reflects the ENV override
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UpdateSiteSettingsRequest {
    pub private_mode: bool,
}
```

- [ ] **Step 2: Implement is_private_mode_active utility**

Add to `web-app/src/server.rs` (SSR-only section):
```rust
#[cfg(feature = "ssr")]
pub async fn is_private_mode_active() -> bool {
    if std::env::var("PRIVATE_MODE").unwrap_or_default() == "true" {
        return true;
    }
    
    use leptos::context::use_context;
    use sqlx::PgPool;
    
    let pool = match use_context::<PgPool>() {
        Some(p) => p,
        None => return false, // Default to public if pool is missing
    };
    
    sqlx::query_scalar!("SELECT private_mode FROM site_settings LIMIT 1")
        .fetch_one(&pool)
        .await
        .unwrap_or(false)
}
```

- [ ] **Step 3: Implement get_site_settings server function**

Add to `web-app/src/server.rs`:
```rust
#[server(GetSiteSettings, "/api")]
pub async fn get_site_settings() -> Result<crate::models::SiteSettings, ServerFnError> {
    use leptos::context::use_context;
    use sqlx::PgPool;

    let is_hard_locked = std::env::var("PRIVATE_MODE").unwrap_or_default() == "true";
    
    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;
    
    let private_mode = sqlx::query_scalar!("SELECT private_mode FROM site_settings LIMIT 1")
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
        
    Ok(crate::models::SiteSettings {
        private_mode: is_hard_locked || private_mode,
        is_hard_locked,
    })
}
```

- [ ] **Step 4: Implement update_site_settings server function**

Add to `web-app/src/server.rs`:
```rust
#[server(UpdateSiteSettings, "/api")]
pub async fn update_site_settings(req: crate::models::UpdateSiteSettingsRequest) -> Result<(), ServerFnError> {
    use crate::auth::session;
    use leptos::context::use_context;
    use leptos_axum::extract;
    use sqlx::PgPool;
    use tower_sessions::Session;

    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;
    let session = extract::<Session>().await?;
    let user = session::get_user(&session)
        .await
        .ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    if user.role != "admin" && user.role != "owner" {
        return Err(ServerFnError::new("Unauthorized"));
    }

    sqlx::query!(
        "UPDATE site_settings SET private_mode = $1, updated_at = CURRENT_TIMESTAMP, updated_by = $2",
        req.private_mode, user.id
    )
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}
```

- [ ] **Step 5: Commit**

```bash
git add web-app/src/models.rs web-app/src/server.rs
git commit -m "feat: add site settings models and server functions"
```

---

### Task 3: Access Control Middleware

**Files:**
- Modify: `web-app/src/server.rs`
- Modify: `web-app/src/main.rs`

- [ ] **Step 1: Refactor admin_auth_middleware to site_auth_middleware**

Replace `admin_auth_middleware` in `web-app/src/server.rs` with:
```rust
#[cfg(feature = "ssr")]
pub async fn site_auth_middleware(
    session: tower_sessions::Session,
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> impl axum::response::IntoResponse {
    use crate::auth::User;
    use axum::response::IntoResponse;

    let path = request.uri().path();
    
    // 1. Allow static assets and essential paths
    let allowed_prefixes = ["/pkg", "/assets", "/login", "/register", "/about", "/robots.txt", "/favicon.ico", "/api"];
    if allowed_prefixes.iter().any(|p| path.starts_with(p)) || path == "/" {
        // We'll handle the root path ("/") redirect in the Leptos router for better UX
        return next.run(request).await;
    }

    // 2. Check if private mode is active
    if is_private_mode_active().await {
        let user: Option<User> = session.get("user").await.ok().flatten();
        if user.is_none() {
            return axum::response::Redirect::temporary("/login").into_response();
        }
    }

    // 3. Admin enforcement
    if path.starts_with("/admin") {
        let user: Option<User> = session.get("user").await.ok().flatten();
        match user {
            Some(u) if u.role == "admin" || u.role == "owner" => next.run(request).await,
            _ => axum::response::Redirect::temporary("/login").into_response(),
        }
    } else {
        next.run(request).await
    }
}
```

- [ ] **Step 2: Update middleware usage in main.rs**

In `web-app/src/main.rs`:
```rust
// Replace admin_auth_middleware with site_auth_middleware
.layer(axum::middleware::from_fn(
    web_app::server::site_auth_middleware,
))
```

- [ ] **Step 3: Commit**

```bash
git add web-app/src/server.rs web-app/src/main.rs
git commit -m "feat: implement site-wide authentication middleware"
```

---

### Task 4: UI Implementation (Admin & Nav)

**Files:**
- Modify: `web-app/src/app.rs`
- Modify: `web-app/src/components/admin.rs`

- [ ] **Step 1: Add private mode badge to navigation**

In `web-app/src/app.rs`, fetch site settings and show the badge:
```rust
#[component]
fn RouterContent() -> impl IntoView {
    let user = Resource::new(|| (), |_| crate::server::get_current_user());
    let site_settings = Resource::new(|| (), |_| crate::server::get_site_settings());
    // ...
    
    view! {
        <main>
            <nav>
                <div class="nav-links">
                    // ... existing links ...
                    <Suspense fallback=|| ()>
                        <Show when=move || matches!(site_settings.get(), Some(Ok(s)) if s.private_mode)>
                            " | "
                            <span class="badge-private">"PRIVATE"</span>
                        </Show>
                    </Suspense>
                    // ... admin and login/logout ...
                </div>
            </nav>
            // ...
        </main>
    }
}
```

- [ ] **Step 2: Add CSS for the private badge**

Add to `web-app/style/main.scss`:
```scss
.badge-private {
    background-color: #ff4444;
    color: white;
    padding: 2px 8px;
    border-radius: 12px;
    font-size: 0.7rem;
    font-weight: bold;
    vertical-align: middle;
}
```

- [ ] **Step 3: Add Settings tab to AdminDashboard**

In `web-app/src/components/admin.rs`, add the "Settings" tab and its view logic:
```rust
// 1. Add update_settings_action
let update_settings_action = ServerAction::<crate::server::UpdateSiteSettings>::new();
let site_settings = Resource::new(
    move || update_settings_action.version().get(),
    |_| async move { crate::server::get_site_settings().await }
);

// 2. Add "Settings" tab button
<button class=move || format!("btn btn-sm {}", if active_tab.get() == "settings" { "btn-primary active" } else { "btn-ghost" })
    on:click=move |_| set_active_tab.set("settings".to_string())>
    "Settings"
</button>

// 3. Add "Settings" tab content
<Show when=move || active_tab.get() == "settings">
    <h3>"Site Settings"</h3>
    <Transition fallback=|| view! { <p>"Loading settings..."</p> }>
        {move || match site_settings.get() {
            Some(Ok(s)) => view! {
                <div class="settings-card">
                    <div class="settings-row">
                        <div class="settings-info">
                            <strong>"Private Mode"</strong>
                            <p>"Require login to view pub data and statistics."</p>
                        </div>
                        <div class="settings-control">
                            <button 
                                class=move || format!("btn {}", if s.private_mode { "btn-danger" } else { "btn-primary" })
                                on:click=move |_| {
                                    update_settings_action.dispatch(crate::server::UpdateSiteSettings {
                                        req: crate::models::UpdateSiteSettingsRequest { private_mode: !s.private_mode }
                                    });
                                }
                                disabled=s.is_hard_locked
                            >
                                {if s.private_mode { "Disable" } else { "Enable" }}
                            </button>
                        </div>
                    </div>
                    {if s.is_hard_locked {
                        view! { <p class="help-text">"Hard-locked via environment variable."</p> }.into_any()
                    } else {
                        view! {}.into_any()
                    }}
                </div>
            }.into_any(),
            _ => view! { <p>"Error loading settings."</p> }.into_any(),
        }}
    </Transition>
</Show>
```

- [ ] **Step 4: Commit**

```bash
git add web-app/src/app.rs web-app/src/components/admin.rs web-app/style/main.scss
git commit -m "feat: add site settings UI and private mode badge"
```

---

### Task 5: Router-level Redirects (UX)

**Files:**
- Modify: `web-app/src/app.rs`

- [ ] **Step 1: Add redirect logic to RouterContent**

In `web-app/src/app.rs`, ensure that if the site is private and the user is not logged in, they are redirected from `/` and other paths:
```rust
Effect::new(move |_| {
    if let (Some(Ok(s)), Some(Ok(user_opt))) = (site_settings.get(), user.get()) {
        if s.private_mode && user_opt.is_none() {
            let path = location.pathname.get();
            let allowed_prefixes = ["/login", "/register", "/about"];
            if !allowed_prefixes.iter().any(|p| path.starts_with(p)) {
                navigate("/login", Default::default());
            }
        }
    }
});
```

- [ ] **Step 2: Commit**

```bash
git add web-app/src/app.rs
git commit -m "feat: add client-side redirects for private mode"
```
