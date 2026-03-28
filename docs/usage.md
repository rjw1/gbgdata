# Usage Guide

## Importing Data

1. Place the source Excel file in the root directory.
2. Set your `.env` variables (see `.env.example`).
3. Run the import tool:
   ```bash
   cargo run -p import-tool
   ```

## Running the Web App

```bash
cd web-app
cargo leptos watch
```

## User Management & Invitations

### Inviting New Users

Administrators can invite others to join the platform:
1. Navigate to the **Admin Dashboard** > **Users** tab.
2. Select the desired role (Admin or User) and click **Generate Invite Link**.
3. Copy the unique link and send it to the new contributor.
4. Links are one-time use and expire after 7 days.

### Roles & Permissions

- **Owner**: Full system control. Cannot be deleted or demoted. Only an Owner can promote an Admin to Owner (transferring ownership).
- **Admin**: Can edit all pub data, process community suggestions, manage non-owner users, and view audit logs.
- **User**: Can browse data, track their own visits, and submit update suggestions.

### Registration

New users can only register using a valid invitation link. Upon clicking the link, they will choose a username and password before being guided through the mandatory 2FA setup.

## Authentication & Security

### Mandatory 2FA (TOTP)

Multi-factor authentication is mandatory for ALL logged-in users.
- **First Login**: You will be redirected to a setup page showing a QR code and secret.
- **Subsequent Logins**: After entering your password, you must provide a 6-digit code from your authenticator app.

### Passkeys (WebAuthn)

Once logged in, users can register one or more **Passkeys** (FaceID, TouchID, or security keys) via their **Profile** page.
- **Faster Login**: If a Passkey is registered, you can log in with a single click after entering your username, bypassing the password and TOTP challenge.

## User Engagement

### Tracking Visits

Logged-in users can keep a personal log of the pubs they have visited:
1. Navigate to a **Pub Detail** page.
2. Click **Log Visit** in the "My Activity" section.
3. Enter the date and optional notes.
4. View your full history and a personal map at the **My Visits** page.

### Data Exports

You can download your personal visit history in several formats from the **My Visits** page:
- **CSV**: Standard spreadsheet format.
- **JSON**: Machine-readable format.
- **Parquet**: Highly efficient columnar format for large datasets.

### Community Suggestions

Users can suggest corrections to pub data:
1. Click **Suggest Update** on any pub page.
2. Choose a category (e.g., "Report Closed" or "Fix Links").
3. Submit your changes for administrator review.

## Administrator Management

### Audit Logging

All administrative actions (edits, user deletions, invite generation, suggestion approvals) are recorded in the **Recent Activity** log. This log is searchable and filterable by administrators.

### Missing Data Reports

Admins can use the dashboard tabs to quickly identify data gaps:
- **Missing Coords**: Pubs that don't have a location on the map.
- **Missing IDs**: Pubs needing WhatPub, Google, or Untappd links.
- **Inconsistencies**: Pubs marked as closed but with recent GBG activity.

## External Data IDs

### Finding Google Maps Place IDs

To link a pub to Google Maps, you must provide its unique **Place ID**.

1.  Visit the official [Google Maps Place ID Finder](https://developers.google.com/maps/documentation/javascript/examples/places-placeid-finder).
2.  Search for the pub by name and town in the search box.
3.  Click on the correct result on the map.
4.  Copy the long alphanumeric string displayed (e.g., `ChIJN1t_tDeuEmsRUsoyG83frY4`).
5.  Paste this into the **Google Maps ID** field on the **Edit Pub** screen.
