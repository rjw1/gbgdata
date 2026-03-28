# Design Spec: User Management & Invitations

**Date:** 2026-03-28
**Status:** Draft
**Topic:** Administrative tools for managing user accounts and inviting new contributors.

---

## 1. Objective
Provide a secure and efficient way for administrators to manage existing user accounts, adjust permissions, and invite new users to the platform via unique registration links.

## 2. User Interface Designs

### 2.1 User Management Dashboard (in `/admin`)
A new "Users" tab added to the existing Admin Dashboard.
- **Search & Filter**:
    - Text input for username search.
    - Dropdown for Role filter (All, Admin, User).
- **User List Table**:
    - **Columns**: `Username`, `Role`, `2FA Status`, `Last Login`, `Actions`.
    - **Actions**:
        - **Change Role**: Dropdown to toggle between 'admin' and 'user'.
        - **Reset 2FA**: Clear TOTP secret and set `totp_setup_completed = false` (useful for lost devices).
        - **Delete**: Remove the user account (with confirmation).

### 2.2 Invitation Management
Located below the User List in the "Users" tab.
- **Action**: "Generate Invite Link" button with a role selection dropdown.
- **Pending Invites Table**:
    - **Columns**: `Invite Link`, `Role`, `Expiration`, `Actions`.
    - **Link Format**: `https://{domain}/register?invite={uuid}`.
    - **Actions**: "Revoke" (Delete the invite record).

### 2.3 Registration Page (`/register`)
A new route specifically for invited users.
- **Validation**: Check `invite` UUID parameter against `user_invites` table.
- **Form**:
    - Username field.
    - Password field (with confirmation).
- **Behavior**: Upon success, log the user in and redirect to `/setup-2fa`.

## 3. Data Flow & Integration

### 3.1 Server Functions
- **`GetUsers(search, role)`**: Returns `Vec<UserDetail>`.
- **`UpdateUserRole(user_id, role)`**: Updates `users.role`.
- **`ResetUser2FA(user_id)`**: Clears TOTP fields for a user.
- **`CreateInvite(role)`**: Generates a new `user_invites` record (default expiry: 7 days).
- **`GetPendingInvites()`**: Lists unused, non-expired invites.
- **`ValidateInvite(invite_id)`**: Checks if an invite is still valid.
- **`RegisterUser(invite_id, username, password)`**: Creates the user and marks invite as used.

### 3.2 Security Rules
- Only `admin` users can access any of these functions.
- Every action (role change, invite creation, 2FA reset) must be logged in `audit_log`.
- Registration is ONLY possible with a valid invite ID.

## 4. Implementation Phases
1. **Phase 1**: Add the "Users" tab and `GetUsers` server function to the Admin Dashboard.
2. **Phase 2**: Implement Role Change and 2FA Reset functionality.
3. **Phase 3**: Build the Invitation generation and listing UI.
4. **Phase 4**: Implement the `/register` route and registration logic.
