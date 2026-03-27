use leptos::prelude::*;
use leptos_router::components::A;
use crate::server::get_pubs;
use crate::models::SortMode;
use crate::components::sort::SortSelector;

#[component]
pub fn PubList() -> impl IntoView {
    let (query, set_query) = signal(String::new());
    let (sort, set_sort) = signal(SortMode::default());
    let (open_only, set_open_only) = signal(false);

    let pubs = Resource::new(
        move || (query.get(), sort.get(), open_only.get()),
        |(q, s, open)| async move { get_pubs(q, Some(s), Some(open)).await }
    );

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
            </div>

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
                            
                            view! {
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
                            }
                        }).collect_view().into_any(),
                        Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
                    })}
                </Suspense>
            </div>
        </div>
    }
}
