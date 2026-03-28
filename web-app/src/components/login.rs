use leptos::prelude::*;
use uuid::Uuid;

#[server(Login, "/api")]
pub async fn login(username: String, password: String) -> Result<Option<Uuid>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use crate::auth::verify_password;
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let user = sqlx::query!(
        "SELECT id, password_hash FROM users WHERE username = $1",
        username
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    if let Some(user) = user {
        if verify_password(&password, &user.password_hash) {
            return Ok(Some(user.id));
        }
    }

    Ok(None)
}

#[server(Verify2FA, "/api")]
pub async fn verify_2fa(user_id: Uuid, code: String) -> Result<bool, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use crate::auth::{verify_totp, verify_recovery_code, User, session};
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let session = extract::<Session>().await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let user_data = sqlx::query!(
        "SELECT id, username, role, totp_setup_completed, totp_secret_enc, recovery_codes_hash FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let success = if code.len() == 6 && code.chars().all(|c| c.is_digit(10)) {
        verify_totp(&user_data.username, &user_data.totp_secret_enc, &code)
    } else {
        // Check recovery codes
        if verify_recovery_code(&code, &user_data.recovery_codes_hash) {
            // Remove used recovery code
            let new_codes: Vec<String> = user_data.recovery_codes_hash
                .into_iter()
                .filter(|h| !crate::auth::verify_password(&code, h))
                .collect();
            
            sqlx::query!(
                "UPDATE users SET recovery_codes_hash = $1 WHERE id = $2",
                &new_codes,
                user_id
            )
            .execute(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
            
            true
        } else {
            false
        }
    };

    if success {
        session::login(&session, &User {
            id: user_data.id,
            username: user_data.username,
            role: user_data.role,
            totp_setup_completed: user_data.totp_setup_completed,
        }).await.map_err(|e| ServerFnError::new(e.to_string()))?;
    }

    Ok(success)
}

#[component]
pub fn LoginForm() -> impl IntoView {
    let login_action = ServerAction::<Login>::new();
    let user_id = move || login_action.value().get().and_then(|v| v.ok().flatten());
    
    view! {
        <div class="login-container">
            <h2>"Admin Login"</h2>
            <Show
                when=move || user_id().is_none()
                fallback=move || view! { <TotpChallenge user_id=user_id().unwrap() /> }
            >
                <div class="login-form">
                    <ActionForm action=login_action>
                        <div class="form-group">
                            <label for="username">"Username"</label>
                            <input type="text" name="username" id="username" required />
                        </div>
                        <div class="form-group">
                            <label for="password">"Password"</label>
                            <input type="password" name="password" id="password" required />
                        </div>
                        <button type="submit" disabled=login_action.pending()>
                            {move || if login_action.pending().get() { "Logging in..." } else { "Login" }}
                        </button>
                        {move || login_action.value().get().map(|v| {
                            if v.is_ok() && v.unwrap().is_none() {
                                view! { <p class="error">"Invalid username or password"</p> }.into_any()
                            } else {
                                ().into_any()
                            }
                        })}
                    </ActionForm>
                </div>
            </Show>
        </div>
    }
}

#[component]
pub fn TotpChallenge(user_id: Uuid) -> impl IntoView {
    let verify_action = ServerAction::<Verify2FA>::new();
    let navigate = leptos_router::hooks::use_navigate();

    Effect::new(move |_| {
        if let Some(Ok(true)) = verify_action.value().get() {
            navigate("/admin", Default::default());
        }
    });

    view! {
        <div class="totp-challenge">
            <h3>"Two-Factor Authentication"</h3>
            <p>"Enter the code from your authenticator app or a recovery code."</p>
            <div class="login-form">
                <ActionForm action=verify_action>
                    <input type="hidden" name="user_id" value=user_id.to_string() />
                    <div class="form-group">
                        <label for="code">"Code"</label>
                        <input type="text" name="code" id="code" required autocomplete="one-time-code" autofocus />
                    </div>
                    <button type="submit" disabled=verify_action.pending()>
                        {move || if verify_action.pending().get() { "Verifying..." } else { "Verify" }}
                    </button>
                    {move || verify_action.value().get().map(|v| {
                        if let Ok(false) = v {
                            view! { <p class="error">"Invalid code"</p> }.into_any()
                        } else {
                            ().into_any()
                        }
                    })}
                </ActionForm>
            </div>
        </div>
    }
}
