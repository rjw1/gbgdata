use leptos::prelude::*;
use leptos_router::components::A;
use crate::server::{get_nearby_pubs, geocode_manual};
use crate::models::{PubSummary, SortMode};
use wasm_bindgen::prelude::*;
use crate::components::sort::SortSelector;

#[component]
pub fn NearMe() -> impl IntoView {
    let (lat_lon, set_lat_lon) = signal(None::<(f64, f64)>);
    let (radius, set_radius) = signal(5000.0); // 5km default
    let (search_text, set_search_text) = signal(String::new());
    let (loading, set_loading) = signal(false);
    let (error, set_error) = signal(None::<String>);
    let (sort, set_sort) = signal(SortMode::Distance); // Default to Distance for Near Me

    let pubs = Resource::new(
        move || (lat_lon.get(), radius.get(), sort.get()),
        |(coords, r, s)| async move {
            if let Some((lat, lon)) = coords {
                get_nearby_pubs(lat, lon, r, Some(s)).await
            } else {
                Ok(Vec::new())
            }
        }
    );

    let get_gps = move |_| {
        set_loading.set(true);
        set_error.set(None);
        
        let window = web_sys::window().expect("no window");
        let navigator = window.navigator();
        let geolocation = navigator.geolocation().expect("no geolocation");

        let success_cb = Closure::wrap(Box::new(move |pos: JsValue| {
            let coords = js_sys::Reflect::get(&pos, &JsValue::from_str("coords")).unwrap();
            let lat = js_sys::Reflect::get(&coords, &JsValue::from_str("latitude")).unwrap().as_f64().unwrap();
            let lon = js_sys::Reflect::get(&coords, &JsValue::from_str("longitude")).unwrap().as_f64().unwrap();
            
            set_lat_lon.set(Some((lat, lon)));
            set_loading.set(false);
        }) as Box<dyn FnMut(JsValue)>);

        let error_cb = Closure::wrap(Box::new(move |err: JsValue| {
            let msg = js_sys::Reflect::get(&err, &JsValue::from_str("message")).unwrap().as_string().unwrap_or_else(|| "Unknown error".into());
            set_error.set(Some(msg));
            set_loading.set(false);
        }) as Box<dyn FnMut(JsValue)>);

        let _ = geolocation.get_current_position(
            success_cb.as_ref().unchecked_ref(),
            Some(error_cb.as_ref().unchecked_ref()),
        );

        success_cb.forget();
        error_cb.forget();
    };

    let handle_search = move |_| {
        let query = search_text.get();
        if query.trim().is_empty() { return; }
        
        set_loading.set(true);
        set_error.set(None);

        spawn_local(async move {
            match geocode_manual(query).await {
                Ok(Some(coords)) => {
                    set_lat_lon.set(Some(coords));
                }
                Ok(None) => {
                    set_error.set(Some("Location not found".into()));
                }
                Err(e) => {
                    set_error.set(Some(e.to_string()));
                }
            }
            set_loading.set(false);
        });
    };

    view! {
        <div class="near-me-container">
            <h1>"Pubs Near Me"</h1>
            
            <div class="search-controls">
                <div class="search-row">
                    <input 
                        type="text" 
                        placeholder="Enter Town or Postcode..." 
                        class="location-input"
                        on:input=move |ev| set_search_text.set(event_target_value(&ev))
                        on:keydown=move |ev| { if ev.key() == "Enter" { handle_search(()); } }
                        prop:value=search_text
                    />
                    <button class="location-btn" on:click=handle_search disabled=loading>
                        "🔍 Search"
                    </button>
                    <button class="gps-btn" on:click=get_gps disabled=loading>
                        "📍 Use GPS"
                    </button>
                </div>

                <div class="radius-row">
                    <label>"Search Radius: "{move || (radius.get() / 1000.0).to_string()}" km"</label>
                    <div class="radius-inputs">
                        <input 
                            type="range" 
                            min="500" 
                            max="50000" 
                            step="500"
                            prop:value=move || radius.get().to_string()
                            on:input=move |ev| set_radius.set(event_target_value(&ev).parse().unwrap_or(5000.0))
                        />
                        <input 
                            type="number" 
                            class="radius-num"
                            prop:value=move || (radius.get() / 1000.0).to_string()
                            on:input=move |ev| set_radius.set(event_target_value(&ev).parse::<f64>().unwrap_or(5.0) * 1000.0)
                        />
                    </div>
                </div>

                <div class="sort-row">
                    <SortSelector sort=sort.into() set_sort=set_sort.into() show_distance=lat_lon.get().is_some() />
                </div>
            </div>

            {move || if loading.get() {
                view! { <p class="loading">"Locating..."</p> }.into_any()
            } else {
                core::iter::empty().collect_view().into_any()
            }}

            {move || error.get().map(|msg| view! { <p class="error">{msg}</p> })}

            <div class="pub-grid">
                <Suspense fallback=|| view! { <p>"Finding pubs..."</p> }>
                    {move || pubs.get().map(|res| match res {
                        Ok(list) => list.into_iter().map(|p| {
                            let id = p.id;
                            let name = p.name.clone();
                            let town = p.town.clone();
                            let county = p.county.clone();
                            let dist = p.distance_meters.map(|d| format!("{:.1} km", d / 1000.0));
                            view! {
                                <A href=format!("/pub/{}", id) attr:class="pub-card">
                                    <h3>{name}</h3>
                                    <p>{format!("{}, {}", town, county)}</p>
                                    {dist.map(|d| view! { <span class="distance-tag">{d}</span> })}
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
