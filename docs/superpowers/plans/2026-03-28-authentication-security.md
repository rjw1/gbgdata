# Authentication & Security Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement a username-first login flow with Passkey support, a mandatory TOTP setup wizard, and a security profile management page.

**Architecture:** Extend the existing auth logic with a two-stage login UI and a new `/setup-2fa` gate. Use `webauthn-rs` for Passkey management.

**Tech Stack:** Rust (Leptos 0.8.0, Axum, SQLx), `webauthn-rs`.

---

### Task 1: New Auth Server Functions

**Files:**
- Modify: `web-app/src/server.rs`

- [ ] **Step 1: Implement `CheckUserAuthType`**

This function checks if a username exists and returns their available auth methods.

```rust
#[server(CheckUserAuthType, "/api")]
pub async fn check_user_auth_type(username: String) -> Result<crate::models::UserAuthStatus, ServerFnError> {
    use sqlx::PgPool;
    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;

    let user = sqlx::query!(
        "SELECT id, totp_setup_completed FROM users WHERE username = $1",
        username
    ).fetch_optional(&pool).await?;

    match user {
        Some(u) => {
            let passkeys_count = sqlx::query_scalar!(
                "SELECT COUNT(*) FROM user_credentials WHERE user_id = $1",
                u.id
            ).fetch_one(&pool).await?.unwrap_or(0);

            Ok(crate::models::UserAuthStatus {
                user_id: Some(u.id),
                has_passkeys: passkeys_count > 0,
                totp_required: !u.totp_setup_completed,
            })
        }
        None => Ok(crate::models::UserAuthStatus {
            user_id: None,
            has_passkeys: false,
            totp_required: false,
        })
    }
}
```

- [ ] **Step 2: Implement `VerifyAndCompleteTotpSetup`**

```rust
#[server(VerifyAndCompleteTotpSetup, "/api")]
pub async fn verify_and_complete_totp_setup(user_id: Uuid, code: String) -> Result<bool, ServerFnError> {
    // 1. Verify TOTP code (reuse logic from verify_2fa)
    // 2. If valid, UPDATE users SET totp_setup_completed = true WHERE id = $1
    // 3. Return success
    Ok(true)
}
```

- [ ] **Step 3: Commit**

```bash
git add web-app/src/server.rs
git commit -m "feat: add auth status and totp setup server functions"
```

---

### Task 2: Two-Stage LoginForm UI

**Files:**
- Modify: `web-app/src/components/login.rs`

- [ ] **Step 1: Implement Stage 1 (Username Entry)**

Change the `LoginForm` to show only the username field first.

- [ ] **Step 2: Implement Stage 2 (Auth Choice)**

After username submission, call `CheckUserAuthType`. 
- If `has_passkeys`, show "Login with Passkey" and "Login with Password" link.
- Otherwise, show the Password field.

- [ ] **Step 3: Integrate Passkey Authentication**

Wire up the "Login with Passkey" button to `StartPasskeyAuthentication` and `FinishPasskeyAuthentication`.

- [ ] **Step 4: Commit**

```bash
git add web-app/src/components/login.rs
git commit -m "feat: implement two-stage login with passkey support"
```

---

### Task 3: Mandatory TOTP Setup Wizard

**Files:**
- Create: `web-app/src/components/setup_2fa.rs`
- Modify: `web-app/src/components/mod.rs`
- Modify: `web-app/src/app.rs`

- [ ] **Step 1: Implement `/setup-2fa` page**

Display the QR code, secret, and OTP URL. Add a verification field.

- [ ] **Step 2: Add Access Guard to App**

Ensure that if a user is logged in but `totp_setup_completed` is false, they are redirected to `/setup-2fa` when accessing protected routes.

- [ ] **Step 3: Commit**

```bash
git add web-app/src/components/setup_2fa.rs web-app/src/app.rs web-app/src/components/mod.rs
git commit -m "feat: implement mandatory TOTP setup wizard"
```

---

### Task 4: User Profile & Security Management

**Files:**
- Create: `web-app/src/components/profile.rs`
- Modify: `web-app/src/components/mod.rs`
- Modify: `web-app/src/app.rs`

- [ ] **Step 1: Implement `/profile` page**

Show current TOTP status, recovery codes, and a list of Passkeys.

- [ ] **Step 2: Implement Passkey Registration in Profile**

Add "Add New Passkey" button using the post-TOTP setup logic or a dedicated profile action.

- [ ] **Step 3: Commit**

```bash
git add web-app/src/components/profile.rs web-app/src/app.rs web-app/src/components/mod.rs
git commit -m "feat: implement user security profile page"
```
