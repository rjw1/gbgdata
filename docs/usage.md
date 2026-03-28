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

## Administrator Management

### Creating the First Administrator

To bootstrap the system with an administrator account, use the `create-admin` command in the import tool:

```bash
cargo run -p import-tool --bin import-tool -- create-admin --username <name> --password <password>
```

The tool will output:
1. **TOTP Secret (Base32)**: For manual entry into your authenticator app.
2. **TOTP Setup URI**: Can be used to generate a QR code or pasted into compatible apps.
3. **Recovery Codes**: Five one-time use codes to regain access if you lose your 2FA device. **Store these securely.**

### Mandatory 2FA

Multi-factor authentication is mandatory for all administrative actions. After logging in with your username and password, you will be prompted for a TOTP code from your authenticator app.
