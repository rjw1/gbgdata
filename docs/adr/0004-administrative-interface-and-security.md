# 4. Administrative Interface and Security Model

Date: 2026-03-28

## Status

Accepted

## Context

The Good Beer Guide data requires periodic corrections and updates that cannot be handled via automated imports alone. We need a secure way for authorized administrators to edit pub details, historical records, and manage other users directly through the web application.

## Decision

- **Authentication**: Local user accounts with password hashing via Argon2id.
- **Multi-Factor Authentication (MFA)**: Mandatory TOTP (Time-based One-Time Password) for all administrative sessions.
- **Session Management**: Server-side sessions stored in Postgres via `tower-sessions`.
- **Authorization**: Role-based access control (initially all users are admins) enforced via server-side session checks.
- **Audit Logging**: Every administrative mutation (Create, Update, Delete) is logged to an `audit_log` table, capturing the user, timestamp, and a JSON diff of the change.
- **User Bootstrapping**: A CLI command in the `import-tool` is used to create the initial administrator and generate recovery codes.

## Consequences

- Increased maintenance capability for data integrity.
- Higher security posture through mandatory MFA.
- Complete traceability of all data changes.
- Additional infrastructure complexity (session tables, encryption dependencies).
