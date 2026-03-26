use leptos::*;
use crate::server::get_pub_detail;
use leptos_router::hooks::use_params_map;
use uuid::Uuid;

#[component]
pub fn PubDetail() -> impl IntoView {
    let params = use_params_map();
    let id = move || {
        params
            .with(|params| params.get("id").cloned())
            .and_then(|id| Uuid::parse_str(&id).ok())
    };

    let pub_data = Resource::new(
        move || id(),
        move |id| async move {
            match id {
                Some(uuid) => get_pub_detail(uuid).await,
                None => Err(ServerFnError::new("Invalid Pub ID")),
            }
        },
    );

    view! {
        <div class="pub-detail-container">
            <Suspense fallback=move || view! { <p>"Loading pub details..."</p> }>
                {move || {
                    pub_data.get().map(|res| {
                        match res {
                            Ok(p) => {
                                view! {
                                    <div class="pub-detail">
                                        <h1>{&p.name}</h1>
                                        <div class="pub-info">
                                            <p class="address">{&p.address}</p>
                                            <p class="location">{format!("{}, {}, {}", p.town, p.county, p.postcode)}</p>
                                            {if p.closed {
                                                view! { <span class="badge closed">"Closed"</span> }.into_view()
                                            } else {
                                                view! { <span class="badge open">"In GBG"</span> }.into_view()
                                            }}
                                        </div>

                                        <div class="stats-card">
                                            <h2>"GBG History"</h2>
                                            <p>"Years in Guide: " {p.years.len()}</p>
                                            <div class="year-grid">
                                                {p.years.into_iter()
                                                    .map(|year| view! { <span class="year-tag">{year}</span> })
                                                    .collect_view()}
                                            </div>
                                        </div>

                                        <div class="external-links">
                                            <h3>"Links"</h3>
                                            <ul>
                                                {p.whatpub_id.map(|id| view! { <li><a href=format!("https://whatpub.com/pubs/{}", id) target="_blank">"WhatPub"</a></li> })}
                                                {p.google_maps_id.map(|id| view! { <li><a href=format!("https://www.google.com/maps/place/?q=place_id:{}", id) target="_blank">"Google Maps"</a></li> })}
                                                {p.untappd_id.map(|id| view! { <li><a href=format!("https://untappd.com/venue/{}", id) target="_blank">"Untappd"</a></li> })}
                                            </ul>
                                        </div>
                                    </div>
                                }.into_view()
                            }
                            Err(e) => view! { <p class="error">"Error: " {e.to_string()}</p> }.into_view(),
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
