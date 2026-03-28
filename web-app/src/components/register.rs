use crate::server::RegisterUser;
use leptos::prelude::*;
use uuid::Uuid;

#[component]
pub fn RegisterPage() -> impl IntoView {
    let query_params = leptos_router::hooks::use_params_map();
    let invite_id_str = move || {
        query_params
            .get()
            .get("invite")
            .map(|s| s.to_string())
            .unwrap_or_default()
    };
    let invite_id = move || Uuid::parse_str(&invite_id_str()).ok();

    let invite_status = Resource::new(invite_id, |id| async move {
        match id {
            Some(uuid) => crate::server::validate_invite(uuid).await,
            None => Ok(None),
        }
    });

    let register_action = ServerAction::<RegisterUser>::new();
    let (username, set_username) = signal(String::new());
    let (password, set_password) = signal(String::new());
    let (confirm, set_confirm) = signal(String::new());

    let navigate = leptos_router::hooks::use_navigate();

    Effect::new(move |_| {
        if let Some(Ok(true)) = register_action.value().get() {
            navigate("/setup-2fa", Default::default());
        }
    });

    view! {
        <div class="login-container">
            <h2>"Create Account"</h2>
            <Suspense fallback=|| view! { <p>"Verifying invitation..."</p> }>
                {move || invite_status.get().map(|res| {
                    match res {
                        Ok(Some(role)) => view! {
                            <p>"Invitation valid for role: " <strong>{role}</strong></p>
                            <form on:submit=move |ev| {
                                ev.prevent_default();
                                if password.get() == confirm.get() {
                                    register_action.dispatch(RegisterUser {
                                        invite_id: invite_id().unwrap(),
                                        username: username.get(),
                                        password: password.get(),
                                    });
                                }
                            } class="login-form">
                                <div class="form-group">
                                    <label>"Username"</label>
                                    <input type="text" on:input=move |ev| set_username.set(event_target_value(&ev)) required />
                                </div>
                                <div class="form-group">
                                    <label>"Password"</label>
                                    <input type="password" on:input=move |ev| set_password.set(event_target_value(&ev)) required />
                                </div>
                                <div class="form-group">
                                    <label>"Confirm Password"</label>
                                    <input type="password" on:input=move |ev| set_confirm.set(event_target_value(&ev)) required />
                                </div>
                                {move || if !confirm.get().is_empty() && password.get() != confirm.get() {
                                    view! { <p class="error">"Passwords do not match"</p> }.into_any()
                                } else {
                                    ().into_any()
                                }}
                                <button type="submit" class="btn btn-primary btn-block" disabled=register_action.pending()>
                                    {move || if register_action.pending().get() { "Creating Account..." } else { "Create Account" }}
                                </button>
                                {move || register_action.value().get().map(|v| {
                                    if let Err(e) = v {
                                        view! { <p class="error">{e.to_string()}</p> }.into_any()
                                    } else {
                                        ().into_any()
                                    }
                                })}
                            </form>
                        }.into_any(),
                        _ => view! {
                            <div class="error-container">
                                <p class="error">"Invalid or expired invitation link."</p>
                                <p>"Please contact an administrator for a new invite."</p>
                                <a href="/">"Return Home"</a>
                            </div>
                        }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}
