use leptos::prelude::*;
use crate::server::{Logout, get_current_user, get_audit_logs};

#[component]
pub fn AdminDashboard() -> impl IntoView {
    let logout_action = ServerAction::<Logout>::new();
    let user = Resource::new(|| (), |_| get_current_user());
    let audit_logs = Resource::new(|| (), |_| get_audit_logs());

    view! {
        <div class="admin-dashboard">
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                {move || match user.get() {
                    Some(Ok(Some(u))) => view! {
                        <div class="admin-header">
                            <h2>"Admin Dashboard"</h2>
                            <p>"Logged in as: " <strong>{u.username}</strong></p>
                            <ActionForm action=logout_action>
                                <button type="submit">"Logout"</button>
                            </ActionForm>
                        </div>
                        <div class="admin-content">
                            <h3>"Recent Activity"</h3>
                            <Transition fallback=move || view! { <p>"Loading logs..."</p> }>
                                {move || match audit_logs.get() {
                                    Some(Ok(logs)) => view! {
                                        <table class="audit-log-table">
                                            <thead>
                                                <tr>
                                                    <th>"Time"</th>
                                                    <th>"User"</th>
                                                    <th>"Action"</th>
                                                    <th>"Entity"</th>
                                                </tr>
                                            </thead>
                                            <tbody>
                                                {logs.into_iter().map(|log| view! {
                                                    <tr>
                                                        <td>{log.timestamp.to_rfc3339()}</td>
                                                        <td>{log.username}</td>
                                                        <td>{log.action}</td>
                                                        <td>{format!("{}: {}", log.entity_type, log.entity_id)}</td>
                                                    </tr>
                                                }).collect_view()}
                                            </tbody>
                                        </table>
                                    }.into_any(),
                                    _ => view! { <p>"Error loading audit logs."</p> }.into_any(),
                                }}
                            </Transition>
                        </div>
                    }.into_any(),
                    Some(Ok(None)) => view! {
                        <div class="error-container">
                            <p>"Access Denied. Please login."</p>
                            <a href="/login">"Login"</a>
                        </div>
                    }.into_any(),
                    _ => view! { <p>"Error loading user data."</p> }.into_any(),
                }}
            </Transition>
        </div>
    }
}
