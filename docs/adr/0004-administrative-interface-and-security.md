# 4. Administrative Interface and Security Model

Date: 2026-03-28

## Status

Accepted

## Context

The Good Beer Guide data requires periodic corrections and updates that cannot be handled via automated imports alone. We need a secure way for authorized administrators to edit pub details, historical records, and manage other users directly through the web application.

## Decision

- **Authentication**: Local user accounts with password hashing via Argon2id.
- **Passkeys (WebAuthn)**: Support for passwordless, multi-factor login using biometric or hardware keys.
- **Multi-Factor Authentication (MFA)**: Mandatory TOTP (Time-based One-Time Password) for ALL logged-in users. Successful Passkey login satisfies the MFA requirement.
- **Session Management**: Server-side sessions stored in Postgres via `tower-sessions`.
- **Authorization**: Role-based access control (Owner, Admin, User) enforced via server-side session checks and fresh database role verification.
    - **Owner**: Immune to deletion/demotion. Can transfer ownership.
    - **Admin**: Full data management rights.
    - **User**: Read-only access with personal visit tracking and suggestion capabilities.
- **User Management**: An invite-only system where admins generate unique, one-time-use registration links.
- **Audit Logging**: Every administrative mutation (edits, user management, invitations, suggestions) is logged to an `audit_log` table. The log is searchable and configurable within the Admin dashboard.
- **User Engagement**: Logged-in users can track visits, export personal data (CSV, JSON, Parquet), and suggest updates to pub records for admin triage.
- **User Bootstrapping**: A CLI command in the `import-tool` is used to create the first owner account.

## Consequences

- Highly secure infrastructure with multiple layers of authentication.
- Decentralized community-led data enrichment via suggestions.
- Strict data integrity through ownership and administrative oversight.
- Full traceability and accountability for all system changes.
