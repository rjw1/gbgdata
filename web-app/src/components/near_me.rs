use leptos::prelude::*;
use crate::server::get_nearby_pubs;
use leptos_router::components::A;

#[component]
pub fn NearMe() -> impl IntoView {
    let (coords, set_coords) = signal(None::<(f64, f64)>);
    let (error, set_error) = signal(None::<String>);

    let pubs = Resource::new(
        move || coords.get(),
        move |c| async move {
            match c {
                Some((lat, lon)) => get_nearby_pubs(lat, lon, 5000.0).await, // 5km radius
                None => Ok(Vec::new()),
            }
        },
    );

    let get_location = move |_| {
        use leptos::wasm_bindgen::{prelude::Closure, JsCast, JsValue};
        
        let window = web_sys::window().expect("no global `window` exists");
        let navigator = window.navigator();
        let geolocation = navigator.geolocation().expect("geolocation not available");

        let success_callback = move |pos: JsValue| {
            let pos_obj = js_sys::Object::from(pos);
            let coords_obj = js_sys::Reflect::get(&pos_obj, &JsValue::from_str("coords")).unwrap();
            let lat = js_sys::Reflect::get(&coords_obj, &JsValue::from_str("latitude")).unwrap().as_f64().unwrap();
            let lon = js_sys::Reflect::get(&coords_obj, &JsValue::from_str("longitude")).unwrap().as_f64().unwrap();
            
            set_coords.set(Some((lat, lon)));
            set_error.set(None);
        };

        let error_callback = move |err: JsValue| {
            let message = js_sys::Reflect::get(&err, &JsValue::from_str("message")).unwrap().as_string().unwrap_or_default();
            set_error.set(Some(message));
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
            <h1>"Pubs Near Me"</h1>
            <button on:click=get_location class="location-btn">
                "Find My Location"
            </button>

            {move || error.get().map(|err| view! { <p class="error">"Error: " {err}</p> })}

            <div class="pub-grid">
                <Suspense fallback=move || view! { <p>"Searching for nearby pubs..."</p> }>
                    {move || {
                        pubs.get().map(|res| {
                            match res {
                                Ok(data) => {
                                    if coords.get().is_some() && data.is_empty() {
                                        view! { <p>"No GBG pubs found within 5km."</p> }.into_any()
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
