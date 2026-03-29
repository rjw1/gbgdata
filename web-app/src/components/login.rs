use crate::server::{
    CheckUserAuthType, FinishPasskeyAuthentication, Login, StartPasskeyAuthentication, Verify2FA,
};
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn LoginForm() -> impl IntoView {
    let check_auth_action = ServerAction::<CheckUserAuthType>::new();
    let login_action = ServerAction::<Login>::new();
    let start_passkey_auth = ServerAction::<StartPasskeyAuthentication>::new();
    let finish_passkey_auth = ServerAction::<FinishPasskeyAuthentication>::new();

    let (username, set_username) = signal(String::new());
    let (stage, set_stage) = signal(1); // 1: Username, 2: Auth Choice / Password

    let auth_status = move || check_auth_action.value().get().and_then(|v| v.ok());
    let user_id = move || {
        login_action
            .value()
            .get()
            .and_then(|v: Result<Option<Uuid>, ServerFnError>| v.ok().flatten())
    };

    let navigate = leptos_router::hooks::use_navigate();

    // Handle stage transition after checking auth type
    Effect::new(move |_| {
        if let Some(Ok(_)) = check_auth_action.value().get() {
            set_stage.set(2);
        }
    });

    // Handle passkey authentication start
    let on_passkey_login = move |_| {
        start_passkey_auth.dispatch(StartPasskeyAuthentication {
            username: username.get(),
        });
    };

    // Handle passkey challenge and finish
    Effect::new(move |_| {
        if let Some(Ok(_challenge)) = start_passkey_auth.value().get() {
            #[cfg(feature = "hydrate")]
            {
                use wasm_bindgen_futures::spawn_local;
                let challenge_cloned = _challenge.clone();
                let finish_auth = finish_passkey_auth;

                spawn_local(async move {
                    let result = crate::auth::client::authenticate(&challenge_cloned).await;

                    if let Ok(resp) = result {
                        finish_auth.dispatch(FinishPasskeyAuthentication {
                            auth_response: resp,
                        });
                    }
                });
            }
        }
    });

    // Handle passkey success
    Effect::new(move |_| {
        if let Some(Ok(true)) = finish_passkey_auth.value().get() {
            navigate("/admin", Default::default());
        }
    });

    view! {
        <div class="login-container">
            <h2>"Admin Login"</h2>

            <Show when=move || user_id().is_none() fallback=move || view! { <TotpChallenge user_id=user_id().unwrap() /> }>
                <div class="login-form">
                    <Show when=move || stage.get() == 1>
                        <ActionForm action=check_auth_action>
                            <div class="form-group">
                                <label for="username">"Username"</label>
                                <input type="text" name="username" id="username" required
                                    on:input=move |ev| set_username.set(event_target_value(&ev)) />
                            </div>
                            <button type="submit" class="btn btn-primary btn-block" disabled=check_auth_action.pending()>
                                {move || if check_auth_action.pending().get() { "Checking..." } else { "Next" }}
                            </button>
                        </ActionForm>
                    </Show>

                    <Show when=move || stage.get() == 2>
                        <p>"Logging in as: " <strong>{move || username.get()}</strong></p>

                        {move || match auth_status() {
                            Some(status) if status.has_passkeys => view! {
                                <div class="passkey-login">
                                    <button type="button" class="btn btn-secondary btn-block" on:click=on_passkey_login disabled=start_passkey_auth.pending()>
                                        {move || if start_passkey_auth.pending().get() { "Starting..." } else { "Login with Passkey" }}
                                    </button>
                                    <div class="divider">"OR"</div>
                                </div>
                            }.into_any(),
                            _ => ().into_any()
                        }}

                        <ActionForm action=login_action>
                            <input type="hidden" name="username" value=move || username.get() />
                            <div class="form-group">
                                <label for="password">"Password"</label>
                                <input type="password" name="password" id="password" required autofocus />
                            </div>
                            <button type="submit" class="btn btn-primary btn-block" disabled=login_action.pending()>
                                {move || if login_action.pending().get() { "Logging in..." } else { "Login" }}
                            </button>
                            <button type="button" class="btn btn-ghost btn-block" style="margin-top: 1rem;" on:click=move |_| set_stage.set(1)>"Back"</button>

                            {move || login_action.value().get().map(|v: Result<Option<Uuid>, ServerFnError>| {
                                if v.is_ok() && v.as_ref().unwrap().is_none() {
                                    view! { <p class="error">"Invalid username or password"</p> }.into_any()
                                } else {
                                    ().into_any()
                                }
                            })}
                        </ActionForm>
                    </Show>
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
                    <button type="submit" class="btn btn-primary btn-block" disabled=verify_action.pending()>
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
