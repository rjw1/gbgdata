use leptos::prelude::*;
use crate::server::get_pub_detail;
use crate::components::stat_ring::StatRing;
use leptos_router::hooks::use_params_map;
use uuid::Uuid;

#[component]
pub fn PubDetail() -> impl IntoView {
    let params = use_params_map();
    let id = move || {
        let params_val = params.get();
        let id_str = params_val.get("id")?.clone();
        Uuid::parse_str(&id_str).ok()
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

                                        {if let (Some(lat), Some(lon)) = (p.lat, p.lon) {
                                            let map_url = format!("https://www.openstreetmap.org/export/embed.html?bbox={},{},{},{}&layer=mapnik&marker={},{}", lon - 0.005, lat - 0.005, lon + 0.005, lat + 0.005, lat, lon);
                                            view! {
                                                <div class="map-container">
                                                    <iframe 
                                                        width="100%" 
                                                        height="300" 
                                                        style="border:0" 
                                                        src=map_url
                                                    ></iframe>
                                                </div>
                                            }.into_any()
                                        } else {
                                            view! { <div class="no-map">"Coordinates not available"</div> }.into_any()
                                        }}

                                        <div class="stats-dashboard">
                                            <StatRing value=p.last_5_years max=5 label="Last 5 Years".to_string() />
                                            
                                            <div class="hero-streak">
                                                <span class="streak-number">{p.current_streak}</span>
                                                <span class="streak-label">"Year Streak"</span>
                                            </div>

                                            <StatRing value=p.last_10_years max=10 label="Last 10 Years".to_string() />
                                        </div>

                                        <div class="stats-card">
                                            <h2>"Historical Data"</h2>
                                            <div class="stats-grid">
                                                <div class="stat-item">
                                                    <span class="stat-label">"Total Inclusions"</span>
                                                    <span class="stat-value">{p.total_years}</span>
                                                </div>
                                                <div class="stat-item">
                                                    <span class="stat-label">"First Inclusion"</span>
                                                    <span class="stat-value">{p.first_year.unwrap_or(0)}</span>
                                                </div>
                                                <div class="stat-item">
                                                    <span class="stat-label">"Latest Inclusion"</span>
                                                    <span class="stat-value">{p.latest_year.unwrap_or(0)}</span>
                                                </div>
                                            </div>
                                            
                                            <h3>"Guide Inclusion Years"</h3>
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
