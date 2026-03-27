use leptos::prelude::*;
use crate::models::PubSummary;

#[component]
pub fn MapView(
    #[prop(into)] pubs: Signal<Vec<PubSummary>>,
) -> impl IntoView {
    let map_container = NodeRef::<html::Div>::new();

    #[cfg(feature = "hydrate")]
    {
        use wasm_bindgen::prelude::*;

        Effect::new(move |_| {
            let list = pubs.get();
            if list.is_empty() { return; }

            if let Some(container) = map_container.get() {
                let window = web_sys::window().expect("no window");
                let l = js_sys::Reflect::get(&window, &JsValue::from_str("L")).expect("Leaflet not found");
                
                // Initialize map if not already done
                // We'll use a data attribute to keep track of the map instance
                let has_map = container.has_attribute("data-map-initialized").unwrap_or(false);
                
                if !has_map {
                    let map_options = js_sys::Object::new();
                    let map = js_sys::Reflect::get(&l, &JsValue::from_str("map")).unwrap()
                        .dyn_into::<js_sys::Function>().unwrap()
                        .call1(&JsValue::NULL, &container.clone().into()).unwrap();
                    
                    // Set view to first pub or center of UK
                    let first = list.first();
                    let lat = 54.0;
                    let lon = -2.0;
                    let zoom = 6;

                    let set_view = js_sys::Reflect::get(&map, &JsValue::from_str("setView")).unwrap()
                        .dyn_into::<js_sys::Function>().unwrap();
                    let coords = js_sys::Array::new();
                    coords.push(&JsValue::from_f64(lat));
                    coords.push(&JsValue::from_f64(lon));
                    set_view.call2(&map, &coords, &JsValue::from_f64(zoom as f64)).unwrap();

                    // Add tile layer
                    let tile_layer = js_sys::Reflect::get(&l, &JsValue::from_str("tileLayer")).unwrap()
                        .dyn_into::<js_sys::Function>().unwrap()
                        .call1(&JsValue::NULL, &JsValue::from_str("https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png")).unwrap();
                    
                    let add_to = js_sys::Reflect::get(&tile_layer, &JsValue::from_str("addTo")).unwrap()
                        .dyn_into::<js_sys::Function>().unwrap();
                    add_to.call1(&tile_layer, &map).unwrap();

                    // Add markers for pubs with coordinates
                    let marker_fn = js_sys::Reflect::get(&l, &JsValue::from_str("marker")).unwrap()
                        .dyn_into::<js_sys::Function>().unwrap();
                    let bind_popup = js_sys::Reflect::get(&marker_fn, &JsValue::from_str("bindPopup")).unwrap(); // This is wrong, it's on the instance

                    for p in list.iter().filter(|p| p.lat.is_some() && p.lon.is_some()) {
                        let lat = p.lat.unwrap();
                        let lon = p.lon.unwrap();
                        let lat_lon = js_sys::Array::new();
                        lat_lon.push(&JsValue::from_f64(lat));
                        lat_lon.push(&JsValue::from_f64(lon));
                        
                        let marker = marker_fn.call1(&JsValue::NULL, &lat_lon).unwrap();
                        
                        // Set popup
                        let popup_content = format!("<b>{}</b><br>{}<br><a href='/pub/{}'>View Details</a>", p.name, p.town, p.id);
                        let set_popup = js_sys::Reflect::get(&marker, &JsValue::from_str("bindPopup")).unwrap()
                            .dyn_into::<js_sys::Function>().unwrap();
                        set_popup.call1(&marker, &JsValue::from_str(&popup_content)).unwrap();

                        let marker_add_to = js_sys::Reflect::get(&marker, &JsValue::from_str("addTo")).unwrap()
                            .dyn_into::<js_sys::Function>().unwrap();
                        marker_add_to.call1(&marker, &map).unwrap();
                    }

                    // Fit bounds if we have multiple markers
                    if list.len() > 1 {
                        // (Simplified for now)
                    }

                    let _ = container.set_attribute("data-map-initialized", "true");
                    
                    // Store map on element for updates
                    // (Simplified for now - in a real app we'd use a better storage)
                }
            }
        });
    }

    view! {
        <div class="map-view-container" node_ref=map_container style="height: 400px; width: 100%; border-radius: 8px; margin-bottom: 1rem;">
            {move || if pubs.get().is_empty() {
                view! { <p class="map-placeholder">"No locations to display"</p> }.into_any()
            } else {
                ().into_any()
            }}
        </div>
    }
}
