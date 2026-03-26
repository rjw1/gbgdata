use leptos::*;
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
                                        view! { <p>"No pubs found."</p> }.into_view()
                                    } else {
                                        data.into_iter()
                                            .map(|p| {
                                                view! {
                                                    <A href=format!("/pub/{}", p.id) class="pub-card">
                                                        <h3>{p.name}</h3>
                                                        <p>{format!("{}, {}", p.town, p.county)}</p>
                                                        {if p.closed {
                                                            view! { <span class="badge closed">"Closed"</span> }.into_view()
                                                        } else {
                                                            view! { <span class="badge open">"In GBG"</span> }.into_view()
                                                        }}
                                                    </A>
                                                }
                                            })
                                            .collect_view()
                                    }
                                }
                                Err(e) => view! { <p class="error">"Error: " {e.to_string()}</p> }.into_view(),
                            }
                        })
                    }}
                </Suspense>
            </div>
        </div>
    }
}
