# Design Spec: Authentication & Security (Passkeys & TOTP)

**Date:** 2026-03-28
**Status:** Draft
**Topic:** Username-first login, Passkey (WebAuthn) support, and mandatory TOTP setup.

---

## 1. Objective
Strengthen the application's security by enforcing Two-Factor Authentication (TOTP) for all users and providing a more modern, passwordless login experience via Passkeys.

## 2. User Interface Designs

### 2.1 Username-First Login Flow (`/login`)
- **Stage 1**: A single input field for the username.
- **Stage 2**:
    - If user has no Passkeys: Show password field + TOTP challenge (existing).
    - If user has Passkeys: Show "Login with Passkey" button. Include a "Login with Password" link for fallback.
- **Stage 3**:
    - Passkey success: Direct login to target page.
    - Password success: Redirect to TOTP challenge or TOTP setup.

### 2.2 Mandatory TOTP Setup Wizard (`/setup-2fa`)
A dedicated full-page wizard for users whose `totp_setup_completed` flag is false.
- **Step 1: Setup**:
    - Display QR Code (SVG/PNG).
    - Display plain-text Secret.
    - **New**: Display the raw OTP URL (`otpauth://...`) for advanced users/manual apps.
- **Step 2: Verification**:
    - Input field for the 6-digit code.
    - "Verify and Enable" button.
- **Step 3: Post-Setup**:
    - Confirmation message.
    - Optional: "Add a Passkey for easier login" button.

### 2.3 User Profile & Security (`/profile`)
A new route for authenticated users to manage their security credentials.
- **2FA Section**: Status (Enabled/Disabled) and link to view/regenerate recovery codes.
- **Passkeys Section**:
    - List of registered passkeys.
    - "Add new Passkey" button.
    - "Remove" action for each passkey.

## 3. Data Flow & Integration

### 3.1 Server Functions
- **`CheckUserAuthType(username)`**: **To be implemented.** Returns if the user has Passkeys and/or requires TOTP.
- **`StartPasskeyRegistration` / `FinishPasskeyRegistration`**: Already exist in `server.rs`.
- **`StartPasskeyAuthentication` / `FinishPasskeyAuthentication`**: Already exist in `server.rs`.
- **`VerifyAndCompleteTotpSetup(code)`**: **To be implemented.** Verifies the first code and sets `totp_setup_completed = true`.
- **`GetMyPasskeys()`**: **To be implemented.** Returns a list of the current user's passkeys.
- **`DeletePasskey(id)`**: **To be implemented.** Removes a specific passkey credential.

### 3.2 Security Rules
- **Access Guard**: Any access to `/admin`, `/profile`, or `/my-visits` must check `totp_setup_completed`. If false, redirect to `/setup-2fa`.
- **Passkey Bypass**: Successful Passkey authentication bypasses the TOTP challenge, as it is considered a strong multi-factor credential by itself.

## 4. Implementation Phases
1. **Phase 1**: Update `LoginForm` to a two-stage (username-first) flow.
2. **Phase 2**: Integrate `StartPasskeyAuthentication` and `FinishPasskeyAuthentication` into the login flow.
3. **Phase 3**: Implement the `/setup-2fa` wizard and the `VerifyAndCompleteTotpSetup` server function.
4. **Phase 4**: Create the `/profile` page with Passkey and Recovery Code management.
