use crate::server::{
    get_current_user, get_my_passkeys, FinishPasskeyRegistration, StartPasskeyRegistration,
};
use leptos::prelude::*;

#[component]
pub fn Profile() -> impl IntoView {
    let user = Resource::new(|| (), |_| get_current_user());
    let passkeys = Resource::new(|| (), |_| get_my_passkeys());

    let delete_action = ServerAction::<crate::server::DeletePasskey>::new();
    let start_reg_action = ServerAction::<StartPasskeyRegistration>::new();
    let finish_reg_action = ServerAction::<FinishPasskeyRegistration>::new();

    // Handle passkey registration challenge
    Effect::new(move |_| {
        if let Some(Ok(_challenge)) = start_reg_action.value().get() {
            #[cfg(feature = "hydrate")]
            {
                use wasm_bindgen_futures::spawn_local;
                let challenge_cloned = _challenge.clone();
                let finish_reg = finish_reg_action;

                spawn_local(async move {
                    let result = crate::auth::client::register(&challenge_cloned).await;
                    if let Ok(resp) = result {
                        finish_reg.dispatch(FinishPasskeyRegistration { reg_response: resp });
                    }
                });
            }
        }
    });

    // Refresh passkeys after actions
    Effect::new(move |_| {
        if delete_action.value().get().is_some() || finish_reg_action.value().get().is_some() {
            passkeys.refetch();
        }
    });

    view! {
        <div class="profile-container">
            <h1>"User Profile"</h1>

            <Suspense fallback=|| view! { <p>"Loading user data..."</p> }>
                {move || user.get().map(|res| {
                    match res {
                        Ok(Some(u)) => view! {
                            <div class="profile-section">
                                <h2>"Security Settings"</h2>
                                <div class="stats-card">
                                    <p><strong>"Username: "</strong> {u.username}</p>
                                    <p><strong>"Role: "</strong> {u.role}</p>
                                    <p><strong>"2FA Status: "</strong>
                                        {if u.totp_setup_completed { "Enabled" } else { "Not Set Up" }}
                                    </p>
                                </div>

                                <h3>"Passkeys (WebAuthn)"</h3>
                                <div class="passkeys-list">
                                    <Suspense fallback=|| view! { <p>"Loading passkeys..."</p> }>
                                        {move || passkeys.get().map(|res| {
                                            match res {
                                                Ok(list) if !list.is_empty() => view! {
                                                    <table class="audit-log-table">
                                                        <thead>
                                                            <tr>
                                                                <th>"Credential ID"</th>
                                                                <th>"Actions"</th>
                                                            </tr>
                                                        </thead>
                                                        <tbody>
                                                            {list.into_iter().map(|pk| {
                                                                let cred_id = pk.credential_id.clone();
                                                                view! {
                                                                    <tr>
                                                                        <td>{hex::encode(&pk.credential_id[..8])}"..."</td>
                                                                        <td>
                                                                            <button class="btn btn-danger btn-sm"
                                                                                on:click=move |_| { delete_action.dispatch(crate::server::DeletePasskey { credential_id: cred_id.clone() }); }
                                                                                disabled=delete_action.pending()>
                                                                                "Remove"
                                                                            </button>
                                                                        </td>
                                                                    </tr>
                                                                }
                                                            }).collect_view()}
                                                        </tbody>
                                                    </table>
                                                }.into_any(),
                                                Ok(_) => view! { <p>"No passkeys registered yet."</p> }.into_any(),
                                                Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
                                            }
                                        })}
                                    </Suspense>
                                    <button class="btn btn-secondary" on:click=move |_| { start_reg_action.dispatch(StartPasskeyRegistration {}); } disabled=start_reg_action.pending()>
                                        {move || if start_reg_action.pending().get() { "Starting..." } else { "Add New Passkey" }}
                                    </button>
                                </div>
                            </div>
                        }.into_any(),
                        Ok(None) => view! { <p>"Please login to view your profile."</p> }.into_any(),
                        Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}
