use leptos::prelude::*;
use crate::server::{Logout, get_current_user, get_audit_logs, get_pub_detail, ProcessSuggestedUpdate};
use crate::components::edit_pub::EditPub;

#[component]
pub fn AdminDashboard() -> impl IntoView {
    let logout_action = ServerAction::<Logout>::new();
    let user = Resource::new(|| (), |_| get_current_user());
    let audit_logs = Resource::new(|| (), |_| get_audit_logs());
    
    let (active_tab, set_active_tab) = signal(String::from("activity"));
    let (editing_pub_id, set_editing_pub_id) = signal(None::<uuid::Uuid>);
    let (user_search, set_user_search) = signal(String::new());
    let (role_filter, set_role_filter) = signal(String::from("all"));

    let users_list = Resource::new(
        move || (active_tab.get(), user_search.get(), role_filter.get()),
        |(tab, search, role)| async move {
            if tab == "users" {
                crate::server::get_users(search, role).await
            } else {
                Ok(vec![])
            }
        }
    );

    let update_role_action = ServerAction::<crate::server::UpdateUserRole>::new();
    let reset_2fa_action = ServerAction::<crate::server::ResetUser2FA>::new();

    Effect::new(move |_| {
        if update_role_action.value().get().is_some() || reset_2fa_action.value().get().is_some() {
            users_list.refetch();
        }
    });

    let suggestions = Resource::new(
        move || active_tab.get(),
        |tab| async move {
            if tab == "suggestions" {
                crate::server::get_suggested_updates(Some("pending".to_string())).await
            } else {
                Ok(vec![])
            }
        }
    );

    let process_suggestion = ServerAction::<ProcessSuggestedUpdate>::new();

    Effect::new(move |_| {
        if process_suggestion.value().get().is_some() {
            suggestions.refetch();
        }
    });

    let report_data = Resource::new(
        move || active_tab.get(),
        |tab| async move {
            match tab.as_str() {
                "coords" | "ids" | "closed" => crate::server::get_missing_data_reports(tab).await,
                _ => Ok(vec![]),
            }
        }
    );

    let pub_to_edit = Resource::new(
        move || editing_pub_id.get(),
        |id| async move {
            match id {
                Some(uuid) => get_pub_detail(uuid).await,
                None => Err(ServerFnError::new("No pub selected")),
            }
        }
    );

    view! {
        <div class="admin-dashboard">
            <Transition fallback=move || view! { <p>"Loading..."</p> }>
                {move || match user.get() {
                    Some(Ok(Some(u))) if u.role == "admin" => view! {
                        <div class="admin-header">
                            <h2>"Admin Dashboard"</h2>
                            <p>"Logged in as: " <strong>{u.username}</strong></p>
                            <ActionForm action=logout_action>
                                <button type="submit">"Logout"</button>
                            </ActionForm>
                        </div>
                        <div class="admin-content">
                            <div class="admin-tabs">
                                <button class=move || if active_tab.get() == "activity" { "active" } else { "" }
                                    on:click=move |_| set_active_tab.set("activity".to_string())>
                                    "Recent Activity"
                                </button>
                                <button class=move || if active_tab.get() == "coords" { "active" } else { "" }
                                    on:click=move |_| set_active_tab.set("coords".to_string())>
                                    "Missing Coords"
                                </button>
                                <button class=move || if active_tab.get() == "ids" { "active" } else { "" }
                                    on:click=move |_| set_active_tab.set("ids".to_string())>
                                    "Missing IDs"
                                </button>
                                <button class=move || if active_tab.get() == "closed" { "active" } else { "" }
                                    on:click=move |_| set_active_tab.set("closed".to_string())>
                                    "Closed (In GBG)"
                                </button>
                                <button class=move || if active_tab.get() == "suggestions" { "active" } else { "" }
                                    on:click=move |_| set_active_tab.set("suggestions".to_string())>
                                    "Suggestions"
                                </button>
                                <button class=move || if active_tab.get() == "users" { "active" } else { "" }
                                    on:click=move |_| set_active_tab.set("users".to_string())>
                                    "Users"
                                </button>
                            </div>

                            <Show when=move || active_tab.get() == "activity">
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
                                                            <td>{log.timestamp.map(|t| t.to_rfc3339()).unwrap_or_else(|| "N/A".to_string())}</td>
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
                            </Show>

                            <Show when=move || active_tab.get() == "suggestions">
                                <h3>"Community Suggestions"</h3>
                                <Transition fallback=move || view! { <p>"Loading suggestions..."</p> }>
                                    {move || match suggestions.get() {
                                        Some(Ok(list)) if !list.is_empty() => view! {
                                            <table class="audit-log-table">
                                                <thead>
                                                    <tr>
                                                        <th>"Pub"</th>
                                                        <th>"User"</th>
                                                        <th>"Date"</th>
                                                        <th>"Actions"</th>
                                                    </tr>
                                                </thead>
                                                <tbody>
                                                    {list.into_iter().map(|s| {
                                                        let id = s.id;
                                                        view! {
                                                            <tr>
                                                                <td><a href=format!("/pub/{}", s.pub_id)>{s.pub_name}</a></td>
                                                                <td>{s.username}</td>
                                                                <td>{s.created_at.map(|t| t.format("%Y-%m-%d").to_string()).unwrap_or_default()}</td>
                                                                <td>
                                                                    <button on:click=move |_| {
                                                                        process_suggestion.dispatch(ProcessSuggestedUpdate {
                                                                            suggestion_id: id,
                                                                            approve: true,
                                                                        });
                                                                    }>"Approve"</button>
                                                                    <button class="delete-btn" on:click=move |_| {
                                                                        process_suggestion.dispatch(ProcessSuggestedUpdate {
                                                                            suggestion_id: id,
                                                                            approve: false,
                                                                        });
                                                                    }>"Reject"</button>
                                                                </td>
                                                            </tr>
                                                        }
                                                    }).collect_view()}
                                                </tbody>
                                            </table>
                                        }.into_any(),
                                        Some(Ok(_)) => view! { <p>"No pending suggestions."</p> }.into_any(),
                                        _ => view! { <p>"Error loading suggestions."</p> }.into_any(),
                                    }}
                                </Transition>
                            </Show>

                            <Show when=move || active_tab.get() == "users">
                                <h3>"User Management"</h3>
                                <div class="list-controls">
                                    <input type="text" placeholder="Search users..." 
                                        on:input=move |ev| set_user_search.set(event_target_value(&ev)) />
                                    <select on:change=move |ev| set_role_filter.set(event_target_value(&ev))>
                                        <option value="all">"All Roles"</option>
                                        <option value="admin">"Admin"</option>
                                        <option value="user">"User"</option>
                                    </select>
                                </div>
                                <Transition fallback=move || view! { <p>"Loading users..."</p> }>
                                    {move || match users_list.get() {
                                        Some(Ok(list)) => view! {
                                            <table class="audit-log-table">
                                                <thead>
                                                    <tr>
                                                        <th>"Username"</th>
                                                        <th>"Role"</th>
                                                        <th>"2FA"</th>
                                                        <th>"Last Login"</th>
                                                        <th>"Actions"</th>
                                                    </tr>
                                                </thead>
                                                <tbody>
                                                    {list.into_iter().map(|u| {
                                                        let id = u.id;
                                                        let current_role = u.role.clone();
                                                        view! {
                                                            <tr>
                                                                <td>{u.username}</td>
                                                                <td>
                                                                    <select on:change=move |ev| {
                                                                        update_role_action.dispatch(crate::server::UpdateUserRole {
                                                                            user_id: id,
                                                                            new_role: event_target_value(&ev),
                                                                        });
                                                                    }>
                                                                        <option value="admin" selected=current_role == "admin">"Admin"</option>
                                                                        <option value="user" selected=current_role == "user">"User"</option>
                                                                    </select>
                                                                </td>
                                                                <td>{if u.totp_setup_completed { "✅" } else { "❌" }}</td>
                                                                <td>{u.last_login.map(|t| t.format("%Y-%m-%d").to_string()).unwrap_or_else(|| "Never".to_string())}</td>
                                                                <td>
                                                                    <button on:click=move |_| {
                                                                        reset_2fa_action.dispatch(crate::server::ResetUser2FA { user_id: id });
                                                                    } disabled=reset_2fa_action.pending()>"Reset 2FA"</button>
                                                                </td>
                                                            </tr>
                                                        }
                                                    }).collect_view()}
                                                </tbody>
                                            </table>
                                        }.into_any(),
                                        _ => view! { <p>"Error loading users."</p> }.into_any(),
                                    }}
                                </Transition>
                            </Show>

                            <Show when=move || active_tab.get() != "activity" && active_tab.get() != "suggestions" && active_tab.get() != "users">
                                <h3>{move || match active_tab.get().as_str() {
                                    "coords" => "Pubs with Missing Coordinates",
                                    "ids" => "Pubs with Missing External IDs",
                                    "closed" => "Pubs marked Closed but in recent GBG",
                                    _ => ""
                                }}</h3>
                                <Transition fallback=move || view! { <p>"Loading report..."</p> }>
                                    {move || match report_data.get() {
                                        Some(Ok(pubs)) => view! {
                                            <table class="audit-log-table">
                                                <thead>
                                                    <tr>
                                                        <th>"Name"</th>
                                                        <th>"Location"</th>
                                                        <th>"Missing"</th>
                                                        <th>"Action"</th>
                                                    </tr>
                                                </thead>
                                                <tbody>
                                                    {pubs.into_iter().map(|p| {
                                                        let id = p.id;
                                                        view! {
                                                            <tr>
                                                                <td><a href=format!("/pub/{}", p.id)>{p.name}</a></td>
                                                                <td>{format!("{}, {}", p.town, p.postcode)}</td>
                                                                <td>
                                                                    {match active_tab.get().as_str() {
                                                                        "coords" => "Lat/Lon".to_string(),
                                                                        "ids" => {
                                                                            let mut missing = vec![];
                                                                            if p.whatpub_id.is_none() { missing.push("WhatPub"); }
                                                                            if p.google_maps_id.is_none() { missing.push("Google"); }
                                                                            if p.untappd_id.is_none() { missing.push("Untappd"); }
                                                                            missing.join(", ")
                                                                        },
                                                                        "closed" => "Closed status".to_string(),
                                                                        _ => "".to_string()
                                                                    }}
                                                                </td>
                                                                <td>
                                                                    <button on:click=move |_| set_editing_pub_id.set(Some(id))>"Edit"</button>
                                                                </td>
                                                            </tr>
                                                        }
                                                    }).collect_view()}
                                                </tbody>
                                            </table>
                                        }.into_any(),
                                        _ => view! { <p>"Error loading report data."</p> }.into_any(),
                                    }}
                                </Transition>
                            </Show>

                            <Show when=move || editing_pub_id.get().is_some()>
                                <Suspense fallback=|| view! { <p>"Loading editor..."</p> }>
                                    {move || pub_to_edit.get().map(|res| {
                                        match res {
                                            Ok(p) => view! {
                                                <EditPub pub_data=p on_close=Callback::new(move |_| {
                                                    set_editing_pub_id.set(None);
                                                    report_data.refetch();
                                                }) />
                                            }.into_any(),
                                            Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
                                        }
                                    })}
                                </Suspense>
                            </Show>
                        </div>
                    }.into_any(),
                    Some(Ok(Some(_))) => view! {
                        <div class="error-container">
                            <p>"Access Denied. Administrative privileges required."</p>
                            <a href="/">"Return Home"</a>
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
