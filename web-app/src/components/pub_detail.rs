use leptos::prelude::*;
use crate::server::get_pub_detail;
use leptos_router::hooks::use_params_map;
use uuid::Uuid;

#[component]
pub fn PubDetail() -> impl IntoView {
    let params = use_params_map();
    let id = move || {
        let params_val = params.get();
        let id_str = params_val.get("id");
        leptos::logging::log!
("PubDetail ID update: {:?}", id_str);
        let id_str = id_str?.clone();
        Uuid::parse_str(&id_str).ok()
    };

    let pub_data = Resource::new(
        move || id(),
        move |id| async move {
            leptos::logging::log!
("Fetching pub detail for {:?}", id);
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
                                let name = p.name.clone();
                                let address = p.address.clone();
                                let town = p.town.clone();
                                let county = p.county.clone();
                                let postcode = p.postcode.clone();
                                let closed = p.closed;
                                let years = p.years.clone();
                                let whatpub = p.whatpub_id.clone();
                                let gmaps = p.google_maps_id.clone();
                                let untappd = p.untappd_id.clone();

                                view! {
                                    <div class="pub-detail">
                                        <h1>{name}</h1>
                                        <div class="pub-info">
                                            <p class="address">{address}</p>
                                            <p class="location">{format!("{}, {}, {}", town, county, postcode)}</p>
                                            {if closed {
                                                view! { <span class="badge closed">"Closed"</span> }.into_any()
                                            } else {
                                                view! { <span class="badge open">"In GBG"</span> }.into_any()
                                            }}
                                        </div>

                                        <div class="stats-card">
                                            <h2>"GBG History"</h2>
                                            <p>"Years in Guide: " {years.len()}</p>
                                            <div class="year-grid">
                                                {years.into_iter()
                                                    .map(|year| view! { <span class="year-tag">{year}</span> })
                                                    .collect_view()}
                                            </div>
                                        </div>

                                        <div class="external-links">
                                            <h3>"Links"</h3>
                                            <ul>
                                                {whatpub.map(|id| view! { <li><a href=format!("https://whatpub.com/pubs/{}", id) target="_blank">"WhatPub"</a></li> })}
                                                {gmaps.map(|id| view! { <li><a href=format!("https://www.google.com/maps/place/?q=place_id:{}", id) target="_blank">"Google Maps"</a></li> })}
                                                {untappd.map(|id| view! { <li><a href=format!("https://untappd.com/venue/{}", id) target="_blank">"Untappd"</a></li> })}
                                            </ul>
                                        </div>
                                    </div>
                                }.into_any()
                            },
                            Err(e) => view! { <p class="error">"Error: " {e.to_string()}</p> }.into_any(),
                        }
                    })
                }}
            </Suspense>
        </div>
    }
}
