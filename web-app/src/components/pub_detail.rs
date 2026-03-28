use leptos::prelude::*;
use crate::server::get_pub_detail;
use crate::components::stat_ring::StatRing;
use crate::components::edit_pub::EditPub;
use crate::components::log_visit::LogVisitModal;
use crate::components::suggest_update::SuggestUpdateModal;
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

    let user = Resource::new(|| (), |_| crate::server::get_current_user());
    let (show_edit, set_show_edit) = signal(false);
    let (show_log_visit, set_show_log_visit) = signal(false);
    let (show_suggest, set_show_suggest) = signal(false);

    let visit_status = Resource::new(
        move || (id(), show_edit.get(), show_log_visit.get()), // Refresh when edit or log closes
        move |(id, _, _)| async move {
            match id {
                Some(uuid) => crate::server::get_pub_visit_status(uuid).await,
                None => Ok(false),
            }
        },
    );

    let pub_data = Resource::new(
        move || (id(), show_edit.get()), // Refresh when edit closes
        move |(id, _)| async move {
            match id {
                Some(uuid) => get_pub_detail(uuid).await,
                None => Err(ServerFnError::new("Invalid Pub ID")),
            }
        },
    );

    let photos = Resource::new(
        move || (id(), show_edit.get()), // Refresh when edit closes
        move |(id, _)| async move {
            match id {
                Some(uuid) => crate::server::get_pub_photos(uuid).await,
                None => Ok(vec![]),
            }
        },
    );

    let process_action = ServerAction::<crate::server::ProcessSuggestedUpdate>::new();

    let pending_suggestions = Memo::new(move |_| {
        let (id, _) = (id(), show_edit.get());
        let process_finished = process_action.value().get();
        (id, process_finished)
    });

    let pending_suggestions_res = Resource::new(
        move || pending_suggestions.get(),
        |(id, _)| async move {
            if let Some(uuid) = id {
                let all = crate::server::get_suggested_updates(Some("pending".to_string())).await?;
                Ok(all.into_iter().filter(|s| s.pub_id == uuid).collect::<Vec<_>>())
            } else {
                Ok(vec![])
            }
        }
    );

    Effect::new(move |_| {
        if process_action.value().get().is_some() {
            pending_suggestions_res.refetch();
            pub_data.refetch();
        }
    });

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
                            let region = p.region.clone();
                            let postcode = p.postcode.clone();
                            let closed = p.closed;
                            let years = p.years.clone();
                            let whatpub = p.whatpub_id.clone();
                            let gmaps = p.google_maps_id.clone();
                            let untappd = p.untappd_id.clone();
                            let untappd_verified = p.untappd_verified;
                            let p_cloned = p.clone();
                            let p_cloned_2 = p.clone();

                            view! {
                                <div class="pub-detail">
                                    <Show when=move || show_edit.get()>
                                        <EditPub pub_data=p_cloned.clone() on_close=Callback::new(move |_| set_show_edit.set(false)) />
                                    </Show>
                                    <Show when=move || show_log_visit.get()>
                                        <LogVisitModal pub_id=id().unwrap() on_close=Callback::new(move |_| set_show_log_visit.set(false)) />
                                    </Show>
                                    <Show when=move || show_suggest.get()>
                                        <SuggestUpdateModal pub_data=p_cloned_2.clone() on_close=Callback::new(move |_| set_show_suggest.set(false)) />
                                    </Show>

                                    <Suspense fallback=|| ()>
                                        <Show when=move || matches!(user.get(), Some(Ok(Some(ref u))) if u.role == "admin" || u.role == "owner")>
                                            <Suspense fallback=|| ()>
                                                {move || pending_suggestions_res.get().map(|res: Result<Vec<crate::models::SuggestedUpdate>, ServerFnError>| {
                                                    match res {
                                                        Ok(list) if !list.is_empty() => {
                                                            let s = list[0].clone();
                                                            view! {
                                                                <div class="admin-suggestion-banner">
                                                                    <p>"User " <strong>{s.username}</strong> " suggested an update."</p>
                                                                    <button class="btn btn-primary" on:click=move |_| {
                                                                        process_action.dispatch(crate::server::ProcessSuggestedUpdate {
                                                                            suggestion_id: s.id,
                                                                            approve: true,
                                                                        });
                                                                    }>"Approve"</button>
                                                                    <button class="btn btn-danger" on:click=move |_| {
                                                                        process_action.dispatch(crate::server::ProcessSuggestedUpdate {
                                                                            suggestion_id: s.id,
                                                                            approve: false,
                                                                        });
                                                                    }>"Reject"</button>
                                                                </div>
                                                            }.into_any()
                                                        },
                                                        _ => ().into_any()
                                                    }
                                                })}
                                            </Suspense>
                                        </Show>
                                    </Suspense>

                                        <div class="pub-header">
                                            <h1>{name.clone()}</h1>
                                            <div class="header-actions">
                                                <Suspense fallback=|| ()>
                                                    <Show when=move || matches!(user.get(), Some(Ok(Some(_))))>
                                                        <button class="btn btn-secondary" on:click=move |_| set_show_suggest.set(true)>"Suggest Update"</button>
                                                    </Show>
                                                    <Show when=move || matches!(user.get(), Some(Ok(Some(ref u))) if u.role == "admin" || u.role == "owner")>
                                                        <button class="btn btn-secondary" on:click=move |_| set_show_edit.set(true)>"Edit"</button>
                                                    </Show>
                                                </Suspense>
                                            </div>

                                        </div>
                                        <div class="pub-info">
                                            <p class="address">{address}</p>
                                            <p class="location">{format!("{}, {}, {}", town.clone(), region, postcode.clone())}</p>
                                            {if closed {
                                                view! { <span class="badge closed">"Closed"</span> }.into_any()
                                            } else {
                                                let year_text = p.latest_year.map(|y| format!("In GBG {}", y)).unwrap_or_else(|| "In GBG".to_string());
                                                view! { <span class="badge open">{year_text}</span> }.into_any()
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
                                                    .map(|year| {
                                                        let class = if year == 1972 { "year-tag trial" } else { "year-tag" };
                                                        let label = if year == 1972 { "1972 (Trial)".to_string() } else { year.to_string() };
                                                        view! { <span class=class title=if year == 1972 { "1972 was a trial run of the GBG and is excluded from stats." } else { "" }>{label}</span> }
                                                    })
                                                    .collect_view()}
                                            </div>
                                        </div>

                                        <Suspense fallback=|| ()>
                                            <Show when=move || matches!(user.get(), Some(Ok(Some(_))))>
                                                <div class="stats-card my-activity-card">
                                                    <h2>"My Activity"</h2>
                                                    <Suspense fallback=|| view! { <p>"Loading activity..."</p> }>
                                                        {move || {
                                                            visit_status.get().map(|res| {
                                                                let has_visited = res.unwrap_or(false);
                                                                view! {
                                                                    <p>
                                                                        {if has_visited { "You have visited this pub." } else { "You haven't logged a visit here yet." }}
                                                                    </p>
                                                                    <button class="btn btn-secondary" on:click=move |_| set_show_log_visit.set(true)>
                                                                        "Log Visit"
                                                                    </button>
                                                                }
                                                            })
                                                        }}
                                                    </Suspense>
                                                </div>
                                            </Show>
                                        </Suspense>

                                        <Suspense fallback=|| view! { <p>"Loading photos..."</p> }>
                                            {move || photos.get().map(|res| {
                                                match res {
                                                    Ok(p_list) if !p_list.is_empty() => view! {
                                                        <div class="stats-card pub-photos-section">
                                                            <h2>"Photos"</h2>
                                                            <div class="pub-photos-grid">
                                                                {p_list.into_iter().map(|p| view! {
                                                                    <div class="photo-item">
                                                                        <a href=p.image_url.clone() target="_blank">
                                                                            <img src=p.image_url.clone() alt=p.owner_name.clone() style="max-width: 100%; border-radius: 8px;" />
                                                                        </a>
                                                                        <p class="attribution">
                                                                            "Photo: " 
                                                                            <a href=p.original_url.clone() target="_blank">
                                                                                {if p.flickr_id.is_some() { "Flickr" } else { "Source" }}
                                                                            </a>
                                                                            " by " <strong>{p.owner_name.clone()}</strong>
                                                                            " (" 
                                                                            <a href=p.license_url.clone() target="_blank">{p.license_type.clone()}</a>
                                                                            ")"
                                                                        </p>
                                                                    </div>
                                                                }).collect_view()}
                                                            </div>
                                                        </div>
                                                    }.into_any(),
                                                    _ => ().into_any(),
                                                }
                                            })}
                                        </Suspense>

                                        <div class="external-links">
                                            <h3>"Links"</h3>
                                            <ul>
                                                {if let Some(id) = whatpub.clone() {
                                                    view! { <li><a href=format!("https://camra.org.uk/pubs/{}", id) target="_blank">"WhatPub"</a></li> }.into_any()
                                                } else {
                                                    let search_url = format!("https://camra.org.uk/pubs/?pub_search={}+{}", name.replace(' ', "+"), postcode.replace(' ', "+"));
                                                    view! { <li><a href=search_url target="_blank" class="search-fallback">"Search on WhatPub"</a></li> }.into_any()
                                                }}
                                                
                                                {if let Some(id) = gmaps.clone() {
                                                    view! { <li><a href=format!("https://www.google.com/maps/place/?q=place_id:{}", id) target="_blank">"Google Maps"</a></li> }.into_any()
                                                } else {
                                                    let search_url = format!("https://www.google.com/maps/search/{}+{}/", name.replace(' ', "+"), postcode.replace(' ', "+"));
                                                    view! { <li><a href=search_url target="_blank" class="search-fallback">"Search on Google Maps"</a></li> }.into_any()
                                                }}

                                                {if let Some(id) = untappd.clone() {
                                                    view! {
                                                        <li>
                                                            <a href=format!("https://untappd.com/venue/{}", id) target="_blank">"Untappd"</a>
                                                            {if untappd_verified {
                                                                view! { 
                                                                    <span class="verified-badge" title="Verified on Untappd">" ✓"</span> 
                                                                    " ("
                                                                    <a href=format!("https://untappd.com/venue/{}/menu", id) target="_blank" class="menu-link">"Menu"</a>
                                                                    ")"
                                                                }.into_any()
                                                            } else {
                                                                "".into_any()
                                                            }}
                                                        </li>
                                                    }.into_any()
                                                } else {
                                                    let search_url = format!("https://untappd.com/search?q={}+{}&type=venue&sort=", name.replace(' ', "+"), town.replace(' ', "+"));
                                                    view! { <li><a href=search_url target="_blank" class="search-fallback">"Search on Untappd"</a></li> }.into_any()
                                                }}
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
