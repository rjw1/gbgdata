use leptos::prelude::*;
use crate::server::{get_user_visits, ExportUserVisits};
use crate::components::map::MapView;
use crate::models::PubSummary;
use leptos::wasm_bindgen::{JsCast, JsValue};

#[component]
pub fn MyVisits() -> impl IntoView {
    let visits = Resource::new(|| (), |_| get_user_visits());
    let (view_mode, set_view_mode) = signal(String::from("list"));
    
    let export_action = ServerAction::<ExportUserVisits>::new();

    let on_export = move |_| {
        export_action.dispatch(ExportUserVisits {});
    };

    Effect::new(move |_| {
        if let Some(Ok(csv_data)) = export_action.value().get() {
            // Basic browser-side download of CSV
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let link = document.create_element("a").unwrap().dyn_into::<web_sys::HtmlAnchorElement>().unwrap();
            let blob_parts = js_sys::Array::new();
            blob_parts.push(&JsValue::from_str(&csv_data));
            let blob = web_sys::Blob::new_with_str_sequence(&blob_parts).unwrap();
            let url = web_sys::Url::create_object_url_with_blob(&blob).unwrap();
            link.set_href(&url);
            link.set_download("my_visits.csv");
            link.click();
            web_sys::Url::revoke_object_url(&url).unwrap();
        }
    });

    view! {
        <div class="my-visits-container">
            <div class="explorer-header">
                <h1>"My Visits"</h1>
                <div class="view-toggle">
                    <button 
                        class=move || if view_mode.get() == "list" { "active" } else { "" }
                        on:click=move |_| set_view_mode.set("list".to_string())
                    >
                        "List"
                    </button>
                    <button 
                        class=move || if view_mode.get() == "map" { "active" } else { "" }
                        on:click=move |_| set_view_mode.set("map".to_string())
                    >
                        "Map"
                    </button>
                </div>
            </div>

            <Suspense fallback=|| view! { <div class="loading">"Loading dashboard..."</div> }>
                {move || visits.get().map(|res| {
                    match res {
                        Ok(v) => {
                            let total_visits = v.len();
                            let unique_pubs = v.iter().map(|visit| visit.pub_id).collect::<std::collections::HashSet<_>>().len();
                            
                            let v_cloned = v.clone();
                            let pubs_for_map = Memo::new(move |_| {
                                v_cloned.iter().map(|visit| PubSummary {
                                    id: visit.pub_id,
                                    name: visit.pub_name.clone(),
                                    town: String::new(), 
                                    region: String::new(),
                                    country_code: None,
                                    postcode: String::new(),
                                    closed: false,
                                    distance_meters: None,
                                    lat: None, 
                                    lon: None,
                                    latest_year: None,
                                    total_years_rank: None,
                                    current_streak: None,
                                }).collect::<Vec<_>>()
                            });

                            view! {
                                <div class="stats-dashboard">
                                    <div class="stat-card">
                                        <span class="stat-value">{unique_pubs}</span>
                                        <span class="stat-label">"Unique Pubs"</span>
                                    </div>
                                    <div class="stat-card">
                                        <span class="stat-value">{total_visits}</span>
                                        <span class="stat-label">"Total Visits"</span>
                                    </div>
                                    <div class="stat-card action-card">
                                        <button class="export-btn" on:click=on_export disabled=export_action.pending()>
                                            {move || if export_action.pending().get() { "Exporting..." } else { "Export CSV" }}
                                        </button>
                                    </div>
                                </div>

                                <div class="visits-content">
                                    <Show when=move || view_mode.get() == "list">
                                        <div class="stats-card">
                                            <h2>"Visit History"</h2>
                                            <table class="audit-log-table">
                                                <thead>
                                                    <tr>
                                                        <th>"Date"</th>
                                                        <th>"Pub"</th>
                                                        <th>"Notes"</th>
                                                    </tr>
                                                </thead>
                                                <tbody>
                                                    {v.clone().into_iter().map(|visit| view! {
                                                        <tr>
                                                            <td>{visit.visit_date.to_string()}</td>
                                                            <td>
                                                                <a href=format!("/pub/{}", visit.pub_id)>{visit.pub_name}</a>
                                                            </td>
                                                            <td>{visit.notes.unwrap_or_default()}</td>
                                                        </tr>
                                                    }).collect_view()}
                                                </tbody>
                                            </table>
                                        </div>
                                    </Show>
                                    <Show when=move || view_mode.get() == "map">
                                        <div class="stats-card">
                                            <h2>"Visit Map"</h2>
                                            <MapView pubs=pubs_for_map />
                                        </div>
                                    </Show>
                                </div>
                            }.into_any()
                        },
                        Err(e) => view! { <p class="error">"Error loading visits: " {e.to_string()}</p> }.into_any(),
                    }
                })}
            </Suspense>
        </div>
    }
}
