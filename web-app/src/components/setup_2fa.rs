use leptos::prelude::*;
use crate::server::{VerifyAndCompleteTotpSetup, get_current_user};

#[component]
pub fn Setup2FA() -> impl IntoView {
    let setup_info = Resource::new(|| (), |_| get_totp_setup_info_wrapper());
    let verify_action = ServerAction::<VerifyAndCompleteTotpSetup>::new();
    let user = Resource::new(|| (), |_| get_current_user());
    
    let (_code, set_code) = signal(String::new());
    let navigate = leptos_router::hooks::use_navigate();

    Effect::new(move |_| {
        if let Some(Ok(true)) = verify_action.value().get() {
            navigate("/admin", Default::default());
        }
    });

    view! {
        <div class="setup-2fa-container">
            <h1>"Two-Factor Authentication Setup"</h1>
            <p>"To keep your account secure, you must set up 2FA before continuing."</p>

            <Suspense fallback=|| view! { <p>"Loading setup info..."</p> }>
                {move || setup_info.get().map(|res| {
                    match res {
                        Ok(info) => {
                            let qr_code = info["qr_code"].as_str().unwrap_or_default().to_string();
                            let secret = info["secret"].as_str().unwrap_or_default().to_string();
                            let url = info["url"].as_str().unwrap_or_default().to_string();

                            view! {
                                <div class="setup-grid">
                                    <div class="qr-section">
                                        <div class="qr-code" inner_html=qr_code></div>
                                        <p class="secret-text">"Secret: " <code>{secret}</code></p>
                                        <p class="url-text">"URL: " <small>{url}</small></p>
                                    </div>
                                    <div class="verify-section">
                                        <h3>"Verify Setup"</h3>
                                        <p>"Enter the 6-digit code from your authenticator app."</p>
                                        <ActionForm action=verify_action>
                                            <input type="hidden" name="user_id" value=move || {
                                                user.get().and_then(|u| u.ok().flatten()).map(|u| u.id.to_string()).unwrap_or_default()
                                            } />
                                            <div class="form-group">
                                                <input type="text" name="code" placeholder="000000" 
                                                    on:input=move |ev| set_code.set(event_target_value(&ev)) required />
                                            </div>
                                            <button type="submit" disabled=verify_action.pending()>
                                                {move || if verify_action.pending().get() { "Verifying..." } else { "Enable 2FA" }}
                                            </button>
                                            {move || verify_action.value().get().map(|v| {
                                                if let Ok(false) = v {
                                                    view! { <p class="error">"Invalid code. Please try again."</p> }.into_any()
                                                } else {
                                                    ().into_any()
                                                }
                                            })}
                                        </ActionForm>
                                    </div>
                                </div>
                            }.into_any()
                        },
                        Err(e) => view! { <p class="error">"Error loading setup: " {e.to_string()}</p> }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}

async fn get_totp_setup_info_wrapper() -> Result<serde_json::Value, ServerFnError> {
    crate::server::get_totp_setup_info().await
}
