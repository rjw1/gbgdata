use leptos::prelude::*;
use leptos_router::components::A;
use crate::server::{get_pubs, BulkUpdatePubsList, get_current_user};
use crate::models::SortMode;
use crate::components::sort::SortSelector;
use std::collections::HashSet;
use uuid::Uuid;

#[component]
pub fn PubList() -> impl IntoView {
    let user = Resource::new(|| (), |_| get_current_user());
    let (query, set_query) = signal(String::new());
    let (sort, set_sort) = signal(SortMode::default());
    let (open_only, set_open_only) = signal(false);

    // Bulk edit state
    let (bulk_mode, set_bulk_mode) = signal(false);
    let (selected_ids, set_selected_ids) = signal(HashSet::<Uuid>::new());
    let (bulk_action, set_bulk_action) = signal(String::from("mark_closed"));
    let (bulk_value, set_bulk_value) = signal(String::from("true"));

    let bulk_update_action = ServerAction::<BulkUpdatePubsList>::new();

    let pubs = Resource::new(
        move || (query.get(), sort.get(), open_only.get(), bulk_update_action.value().get()),
        |(q, s, open, _)| async move { get_pubs(q, Some(s), Some(open)).await }
    );

    let on_bulk_apply = move |_| {
        let ids: Vec<Uuid> = selected_ids.get().into_iter().collect();
        if !ids.is_empty() {
            bulk_update_action.dispatch(BulkUpdatePubsList {
                ids,
                action: bulk_action.get(),
                value: bulk_value.get(),
            });
            set_selected_ids.set(HashSet::new());
        }
    };

    let toggle_selection = move |id: Uuid| {
        set_selected_ids.update(|set| {
            if !set.insert(id) {
                set.remove(&id);
            }
        });
    };

    view! {
        <div class="pub-list-container">
            <div class="list-controls">
                <input
                    type="text"
                    placeholder="Search pubs, towns, or regions..."
                    class="search-input"
                    on:input=move |ev| {
                        set_query.set(event_target_value(&ev));
                    }
                    prop:value=query
                />
                <label class="open-only-toggle">
                    <input 
                        type="checkbox" 
                        on:change=move |ev| set_open_only.set(event_target_checked(&ev))
                        prop:checked=open_only
                    />
                    " Open only"
                </label>
                <SortSelector 
                    sort=Signal::from(sort) 
                    on_change=Callback::new(move |mode| set_sort.set(mode)) 
                />
                <Show when=move || matches!(user.get(), Some(Ok(Some(ref u))) if u.role == "admin" || u.role == "owner")>
                    <button class="bulk-toggle-btn" on:click=move |_| set_bulk_mode.update(|b| *b = !*b)>
                        {move || if bulk_mode.get() { "Cancel Bulk Edit" } else { "Bulk Edit" }}
                    </button>
                </Show>
            </div>

            <Show when=move || bulk_mode.get()>
                <div class="bulk-action-bar">
                    <span>{move || selected_ids.get().len()} " selected"</span>
                    <select on:change=move |ev| set_bulk_action.set(event_target_value(&ev))>
                        <option value="mark_closed">"Change Closed Status"</option>
                        <option value="add_year">"Add GBG Year"</option>
                        <option value="remove_year">"Remove GBG Year"</option>
                    </select>
                    <input type="text" value=bulk_value on:input=move |ev| set_bulk_value.set(event_target_value(&ev)) />
                    <button on:click=on_bulk_apply disabled=move || selected_ids.get().is_empty() || bulk_update_action.pending().get()>
                        "Apply to Selected"
                    </button>
                </div>
            </Show>

            <div class="pub-grid">
                <Suspense fallback=|| view! { <p>"Loading pubs..."</p> }>
                    {move || pubs.get().map(|res| match res {
                        Ok(list) => list.into_iter().map(|p| {
                            let id = p.id;
                            let name = p.name.clone();
                            let town = p.town.clone();
                            let region = p.region.clone();
                            let closed = p.closed;
                            let total = p.total_years_rank.unwrap_or(0);
                            let streak = p.current_streak.unwrap_or(0);
                            let year_text = p.latest_year.map(|y| format!("In GBG {}", y)).unwrap_or_else(|| "In GBG".to_string());
                            
                            let is_selected = move || selected_ids.get().contains(&id);

                            view! {
                                <div class="pub-card-wrapper">
                                    <Show when=move || bulk_mode.get()>
                                        <input type="checkbox" checked=is_selected on:change=move |_| toggle_selection(id) />
                                    </Show>
                                    <A href=format!("/pub/{}", id) attr:class="pub-card">
                                        <h3>{name}</h3>
                                        <p>{format!("{}, {}", town, region)}</p>
                                        
                                        <div class="card-stats">
                                            <div class=format!("stat-badge {}", if sort.get() == SortMode::TotalEntries { "highlight" } else { "" })>
                                                <span class="count">{total}</span>
                                                <span class="label">" entries"</span>
                                            </div>
                                            <div class=format!("stat-badge {}", if sort.get() == SortMode::Streak { "highlight" } else { "" })>
                                                <span class="count">{streak}</span>
                                                <span class="label">" streak"</span>
                                            </div>
                                        </div>

                                        {if closed {
                                            view! { <span class="badge closed">"Closed"</span> }.into_any()
                                        } else {
                                            view! { <span class="badge open">{year_text}</span> }.into_any()
                                        }}
                                    </A>
                                </div>
                            }
                        }).collect_view().into_any(),
                        Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
                    })}
                </Suspense>
            </div>
        </div>
    }
}
