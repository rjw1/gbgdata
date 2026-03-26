use leptos::prelude::*;
use crate::server::{get_nearby_pubs, geocode_manual};
use leptos_router::components::A;

#[component]
pub fn NearMe() -> impl IntoView {
    let (center_coords, set_center_coords) = signal(None::<(f64, f64)>);
    let (radius_km, set_radius_km) = signal(5.0);
    let (search_query, set_search_query) = signal(String::new());
    let (error, set_error) = signal(None::<String>);
    let (is_loading, set_loading) = signal(false);

    let pubs = Resource::new(
        move || (center_coords.get(), radius_km.get()),
        move |(coords, r)| async move {
            match coords {
                Some((lat, lon)) => get_nearby_pubs(lat, lon, r * 1000.0).await,
                None => Ok(Vec::new()),
            }
        },
    );

    let handle_search = move || {
        let query = search_query.get();
        if query.trim().is_empty() { return; }
        
        set_loading.set(true);
        set_error.set(None);
        
        leptos::task::spawn_local(async move {
            match geocode_manual(query).await {
                Ok(Some(coords)) => {
                    set_center_coords.set(Some(coords));
                }
                Ok(None) => {
                    set_error.set(Some("Location not found.".to_string()));
                }
                Err(e) => {
                    set_error.set(Some(e.to_string()));
                }
            }
            set_loading.set(false);
        });
    };

    let handle_gps = move |_| {
        use leptos::wasm_bindgen::{prelude::Closure, JsCast, JsValue};
        
        set_loading.set(true);
        let window = web_sys::window().expect("no global `window` exists");
        let navigator = window.navigator();
        let geolocation = navigator.geolocation().expect("geolocation not available");

        let success_callback = move |pos: JsValue| {
            let pos_obj = js_sys::Object::from(pos);
            let coords_obj = js_sys::Reflect::get(&pos_obj, &JsValue::from_str("coords")).unwrap();
            let lat = js_sys::Reflect::get(&coords_obj, &JsValue::from_str("latitude")).unwrap().as_f64().unwrap();
            let lon = js_sys::Reflect::get(&coords_obj, &JsValue::from_str("longitude")).unwrap().as_f64().unwrap();
            
            set_center_coords.set(Some((lat, lon)));
            set_error.set(None);
            set_loading.set(false);
        };

        let error_callback = move |err: JsValue| {
            let message = js_sys::Reflect::get(&err, &JsValue::from_str("message")).unwrap().as_string().unwrap_or_default();
            set_error.set(Some(message));
            set_loading.set(false);
        };

        let success_closure = Closure::wrap(Box::new(success_callback) as Box<dyn FnMut(JsValue)>);
        let error_closure = Closure::wrap(Box::new(error_callback) as Box<dyn FnMut(JsValue)>);

        let _ = geolocation.get_current_position_with_error_callback(
            success_closure.as_ref().unchecked_ref(),
            Some(error_closure.as_ref().unchecked_ref()),
        );

        success_closure.forget();
        error_closure.forget();
    };

    view! {
        <div class="near-me-container">
            <h1>"Find GBG Pubs"</h1>
            
            <div class="search-controls">
                <div class="search-row">
                    <input
                        type="text"
                        placeholder="Town, Postcode, or Lat,Lon..."
                        on:input=move |ev| set_search_query.set(event_target_value(&ev))
                        on:keydown=move |ev| {
                            if ev.key() == "Enter" {
                                handle_search();
                            }
                        }
                        prop:value=move || search_query.get()
                        class="location-input"
                    />
                    <button 
                        on:click=move |_| handle_search() 
                        class="search-btn" 
                        disabled=move || is_loading.get()
                    >
                        "Search"
                    </button>
                    <button 
                        on:click=handle_gps 
                        class="gps-btn" 
                        disabled=move || is_loading.get()
                    >
                        "GPS"
                    </button>
                </div>

                <div class="radius-row">
                    <label>"Radius: " {move || radius_km.get()} " km"</label>
                    <div class="radius-inputs">
                        <input
                            type="range"
                            min="1"
                            max="50"
                            step="1"
                            on:input=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                    set_radius_km.set(val);
                                }
                            }
                            prop:value=move || radius_km.get()
                        />
                        <input
                            type="number"
                            min="1"
                            max="100"
                            on:input=move |ev| {
                                if let Ok(val) = event_target_value(&ev).parse::<f64>() {
                                    set_radius_km.set(val);
                                }
                            }
                            prop:value=move || radius_km.get()
                            class="radius-num"
                        />
                    </div>
                </div>
            </div>

            {move || is_loading.get().then(|| view! { <p class="loading">"Updating location..."</p> })}
            {move || error.get().map(|err| view! { <p class="error">"Error: " {err}</p> })}

            <div class="pub-grid">
                <Suspense fallback=move || view! { <p>"Searching for nearby pubs..."</p> }>
                    {move || {
                        pubs.get().map(|res| {
                            match res {
                                Ok(data) => {
                                    if center_coords.get().is_some() && data.is_empty() {
                                        view! { <p>"No GBG pubs found within this distance."</p> }.into_any()
                                    } else {
                                        data.into_iter()
                                            .map(|p| {
                                                let dist = p.distance_meters.map(|d| format!("{:.1}km away", d / 1000.0)).unwrap_or_default();
                                                let id = p.id;
                                                let name = p.name.clone();
                                                let town = p.town.clone();
                                                let county = p.county.clone();
                                                view! {
                                                    <A href=format!("/pub/{}", id) attr:class="pub-card">
                                                        <h3>{name}</h3>
                                                        <p>{format!("{}, {}", town, county)}</p>
                                                        {if p.closed {
                                                            view! { <span class="badge closed">"Closed"</span> }.into_any()
                                                        } else {
                                                            let year_text = p.latest_year.map(|y| format!("In GBG {}", y)).unwrap_or_else(|| "In GBG".to_string());
                                                            view! { <span class="badge open">{year_text}</span> }.into_any()
                                                        }}
                                                        <span class="distance-tag">{dist}</span>
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
