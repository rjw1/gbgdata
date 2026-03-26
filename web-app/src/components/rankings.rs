use leptos::prelude::*;
use leptos_router::components::A;
use crate::server::get_ranked_pubs;
use crate::models::SortMode;
use crate::components::sort::SortSelector;

#[component]
pub fn Rankings() -> impl IntoView {
    let (sort, set_sort) = signal(SortMode::TotalEntries); // Default to Total for Rankings

    let pubs = Resource::new(
        move || sort.get(), 
        |s| async move { get_ranked_pubs(Some(s)).await }
    );

    view! {
        <div class="explorer-container">
            <div class="explorer-header">
                <h1>"All-Time GBG Rankings"</h1>
                <SortSelector 
                    sort=Signal::from(sort) 
                    on_change=Callback::new(move |mode| set_sort.set(mode)) 
                />
            </div>
            <p class="years-range">"The most frequently featured pubs in Good Beer Guide history (Top 100)"</p>

            <div class="pub-grid">
                <Suspense fallback=|| view! { <p>"Loading rankings..."</p> }>
                    {move || pubs.get().map(|res| match res {
                        Ok(list) => list.into_iter().enumerate().map(|(idx, p)| {
                            let id = p.id;
                            let name = p.name.clone();
                            let town = p.town.clone();
                            let county = p.county.clone();
                            let total = p.total_years_rank.unwrap_or(0);
                            let streak = p.current_streak.unwrap_or(0);
                            let closed = p.closed;
                            
                            view! {
                                <A href=format!("/pub/{}", id) attr:class="pub-card ranking-card">
                                    <div class="ranking-number">{format!("#{}", idx + 1)}</div>
                                    <div class="ranking-content">
                                        <h3>{name}</h3>
                                        <p>{format!("{}, {}", town, county)}</p>
                                        
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
                                            view! { <span class="badge closed small">"Closed"</span> }.into_any()
                                        } else {
                                            ().into_any()
                                        }}
                                    </div>
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
