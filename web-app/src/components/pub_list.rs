use leptos::prelude::*;
use crate::server::get_pubs;
use leptos_router::components::A;

#[component]
pub fn PubList() -> impl IntoView {
    let (query, set_query) = signal(String::new());

    let pubs = Resource::new(
        move || query.get(),
        move |q| async move { get_pubs(q).await },
    );

    view! {
        <div class="pub-list-container">
            <input
                type="text"
                placeholder="Search pubs, towns, or counties..."
                on:input=move |ev| {
                    set_query.set(event_target_value(&ev));
                }
                prop:value=move || query.get()
                class="search-input"
            />

            <div class="pub-grid">
                <Suspense fallback=move || view! { <p>"Loading pubs..."</p> }>
                    {move || {
                        pubs.get().map(|res| {
                            match res {
                                Ok(data) => {
                                    if data.is_empty() {
                                        view! { <p>"No pubs found."</p> }.into_any()
                                    } else {
                                        data.into_iter()
                                            .map(|p| {
                                                let id = p.id;
                                                let name = p.name.clone();
                                                let click_name = name.clone();
                                                let town = p.town.clone();
                                                let county = p.county.clone();
                                                let closed = p.closed;
                                                view! {
                                                    <A href=format!("/pub/{}", id) 
                                                       attr:class="pub-card"
                                                       on:click=move |_| {
                                                           leptos::logging::log!("Clicked pub {}", click_name);
                                                       }
                                                    >
                                                        <h3>{name}</h3>
                                                        <p>{format!("{}, {}", town, county)}</p>
                                                        {if closed {
                                                            view! { <span class="badge closed">"Closed"</span> }.into_any()
                                                        } else {
                                                            let year_text = p.latest_year.map(|y| format!("In GBG {}", y)).unwrap_or_else(|| "In GBG".to_string());
                                                            view! { <span class="badge open">{year_text}</span> }.into_any()
                                                        }}
                                                    </A>
                                                }
                                            })
                                            .collect_view()
                                            .into_any()
                                    }
                                }
                                Err(e) => view! { <p class="error">"Error: " {e.to_string()}</p> }.into_any(),
                            }
                        })
                    }}
                </Suspense>
            </div>
        </div>
    }
}
