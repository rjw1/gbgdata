use leptos::prelude::*;
use leptos_router::components::A;
use crate::server::get_pubs;
use crate::models::SortMode;
use crate::components::sort::SortSelector;

#[component]
pub fn PubList() -> impl IntoView {
    let (query, set_query) = signal(String::new());
    let (sort, set_sort) = signal(SortMode::default());

    let pubs = Resource::new(
        move || (query.get(), sort.get()),
        |(q, s)| async move { get_pubs(q, Some(s)).await }
    );

    view! {
        <div class="pub-list-container">
            <div class="list-controls">
                <input
                    type="text"
                    placeholder="Search pubs, towns, or counties..."
                    class="search-input"
                    on:input=move |ev| {
                        set_query.set(event_target_value(&ev));
                    }
                    prop:value=query
                />
                <SortSelector sort=sort.into() set_sort=set_sort.into() />
            </div>

            <div class="pub-grid">
                <Suspense fallback=|| view! { <p>"Loading pubs..."</p> }>
                    {move || pubs.get().map(|res| match res {
                        Ok(list) => list.into_iter().map(|p| {
                            let id = p.id;
                            let name = p.name.clone();
                            let town = p.town.clone();
                            let county = p.county.clone();
                            let closed = p.closed;
                            let year_text = p.latest_year.map(|y| format!("In GBG {}", y)).unwrap_or_else(|| "In GBG".to_string());
                            view! {
                                <A href=format!("/pub/{}", id) attr:class="pub-card">
                                    <h3>{name}</h3>
                                    <p>{format!("{}, {}", town, county)}</p>
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
