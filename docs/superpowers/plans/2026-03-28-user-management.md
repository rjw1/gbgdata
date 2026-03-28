# User Management & Invitations Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement administrative tools for managing users, resetting 2FA, and inviting new users via registration links.

**Architecture:** Extend the `AdminDashboard` with a "Users" tab and create a new `/register` route. Utilize existing user and invite schemas.

**Tech Stack:** Rust (Leptos 0.8.0, Axum, SQLx).

---

### Task 1: User Management Server Functions

**Files:**
- Modify: `web-app/src/server.rs`
- Modify: `web-app/src/models.rs`

- [ ] **Step 1: Define `UserManagementEntry` in models**

```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
pub struct UserManagementEntry {
    pub id: Uuid,
    pub username: String,
    pub role: String,
    pub totp_setup_completed: bool,
    pub last_login: Option<chrono::DateTime<chrono::Utc>>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}
```

- [ ] **Step 2: Implement `GetUsers` server function**

```rust
#[server(GetUsers, "/api")]
pub async fn get_users(search: String, role_filter: String) -> Result<Vec<UserManagementEntry>, ServerFnError> {
    // 1. Session check (admin only)
    // 2. Build dynamic query
    // 3. Return user list
}
```

- [ ] **Step 3: Implement `UpdateUserRole` and `ResetUser2FA` functions**

- [ ] **Step 4: Commit**

```bash
git add web-app/src/server.rs web-app/src/models.rs
git commit -m "feat: add user management server functions"
```

---

### Task 2: Users Tab in Admin Dashboard

**Files:**
- Modify: `web-app/src/components/admin.rs`

- [ ] **Step 1: Add "Users" Tab UI**

Implement search input and role filter.

- [ ] **Step 2: Implement User Table with Actions**

Render the table and wire up "Change Role" and "Reset 2FA" buttons.

- [ ] **Step 3: Commit**

```bash
git add web-app/src/components/admin.rs
git commit -m "feat: implement User Management tab in AdminDashboard"
```

---

### Task 3: Invitation System

**Files:**
- Modify: `web-app/src/server.rs`
- Modify: `web-app/src/components/admin.rs`

- [ ] **Step 1: Implement `CreateInvite` and `GetPendingInvites` server functions**

- [ ] **Step 2: Add "Invites" Section to Users Tab**

Implement "Generate Link" UI and the list of active invites.

- [ ] **Step 3: Commit**

```bash
git add web-app/src/server.rs web-app/src/components/admin.rs
git commit -m "feat: implement invite generation and listing"
```

---

### Task 4: Registration Page

**Files:**
- Create: `web-app/src/components/register.rs`
- Modify: `web-app/src/components/mod.rs`
- Modify: `web-app/src/app.rs`

- [ ] **Step 1: Implement `/register` route and component**

Check the `invite` query parameter. If valid, show the registration form.

- [ ] **Step 2: Implement `RegisterUser` server function**

- [ ] **Step 3: Commit**

```bash
git add web-app/src/components/register.rs web-app/src/app.rs web-app/src/components/mod.rs web-app/src/server.rs
git commit -m "feat: implement invite-only registration page"
```
