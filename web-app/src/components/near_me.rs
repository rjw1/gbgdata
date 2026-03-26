use leptos::prelude::*;
use crate::server::get_nearby_pubs;
use crate::models::PubSummary;
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
        let window = web_sys::window().expect("no global `window` exists");
        let navigator = window.navigator();
        let geolocation = navigator.geolocation().expect("geolocation not available");

        let success_callback = move |pos: web_sys::GeolocationPosition| {
            let coords = pos.coords();
            set_coords.set(Some((coords.latitude(), coords.longitude())));
            set_error.set(None);
        };

        let error_callback = move |err: web_sys::GeolocationPositionError| {
            set_error.set(Some(err.message()));
        };

        let success_closure = wasm_bindgen::prelude::Closure::wrap(Box::new(success_callback) as Box<dyn FnMut(web_sys::GeolocationPosition)>);
        let error_closure = wasm_bindgen::prelude::Closure::wrap(Box::new(error_callback) as Box<dyn FnMut(web_sys::GeolocationPositionError)>);

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
                                                view! {
                                                    <A href=format!("/pub/{}", p.id) attr:class="pub-card">
                                                        <h3>{p.name}</h3>
                                                        <p>{format!("{}, {}", p.town, p.county)}</p>
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
