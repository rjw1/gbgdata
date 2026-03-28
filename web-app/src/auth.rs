use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub role: String,
    pub totp_setup_completed: bool,
}

#[cfg(feature = "ssr")]
pub fn verify_password(password: &str, hash: &str) -> bool {
    use argon2::{
        password_hash::{PasswordHash, PasswordVerifier},
        Argon2,
    };
    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return false,
    };
    Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
}

#[cfg(feature = "ssr")]
pub fn verify_totp(username: &str, secret_bytes: &[u8], code: &str) -> bool {
    use totp_rs::{Algorithm, TOTP};
    let totp = match TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret_bytes.to_vec(),
        Some("GBGData".to_string()),
        username.to_string(),
    ) {
        Ok(t) => t,
        Err(_) => return false,
    };
    totp.check_current(code).unwrap_or(false)
}

#[cfg(feature = "ssr")]
pub fn verify_recovery_code(code: &str, hashed_codes: &[String]) -> bool {
    for hash in hashed_codes {
        if verify_password(code, hash) {
            return true;
        }
    }
    false
}

#[cfg(feature = "ssr")]
pub mod session {
    use crate::auth::User;
    use tower_sessions::Session;

    pub async fn login(
        session: &Session,
        user: &User,
    ) -> Result<(), tower_sessions::session::Error> {
        session.insert("user", user).await?;
        Ok(())
    }

    pub async fn logout(session: &Session) -> Result<(), tower_sessions::session::Error> {
        session.delete().await?;
        Ok(())
    }

    pub async fn get_user(session: &Session) -> Option<User> {
        session.get("user").await.ok().flatten()
    }
}

pub mod client {
    use serde_json::Value;

    pub async fn authenticate(_challenge: &Value) -> Result<Value, String> {
        // In a real app, this would use web-sys to call navigator.credentials.get()
        // and return the credential response as JSON.
        // For now, we'll return an error to indicate it's not fully implemented.
        Err("Passkey authentication not yet implemented in browser".to_string())
    }

    pub async fn register(_challenge: &Value) -> Result<Value, String> {
        // Similar to authenticate, but for navigator.credentials.create()
        Err("Passkey registration not yet implemented in browser".to_string())
    }
}
