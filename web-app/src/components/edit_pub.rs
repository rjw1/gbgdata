use leptos::prelude::*;
use crate::models::PubDetail;
use crate::server::{UpdatePub};

#[component]
pub fn EditPub(pub_data: PubDetail, on_close: Callback<()>) -> impl IntoView {
    let update_action = ServerAction::<UpdatePub>::new();
    
    // Form fields as signals initialized with pub_data
    let (name, set_name) = signal(pub_data.name.clone());
    let (address, set_address) = signal(pub_data.address.clone());
    let (town, set_town) = signal(pub_data.town.clone());
    let (region, set_region) = signal(pub_data.region.clone());
    let (postcode, set_postcode) = signal(pub_data.postcode.clone());
    let (closed, set_closed) = signal(pub_data.closed);
    let (lat, set_lat) = signal(pub_data.lat);
    let (lon, set_lon) = signal(pub_data.lon);
    let (untappd_id, set_untappd_id) = signal(pub_data.untappd_id.clone());
    let (google_maps_id, set_google_maps_id) = signal(pub_data.google_maps_id.clone());
    let (whatpub_id, set_whatpub_id) = signal(pub_data.whatpub_id.clone());
    let (rgl_id, set_rgl_id) = signal(pub_data.rgl_id.clone());
    let (years, set_years) = signal(pub_data.years.clone());

    // Photo state
    let (flickr_url, set_flickr_url) = signal(String::new());
    let (photo_title, set_photo_title) = signal(String::new());
    let (photo_owner, set_photo_owner) = signal(String::new());
    let (photo_image_url, set_photo_image_url) = signal(String::new());
    let (photo_original_url, set_photo_original_url) = signal(String::new());
    let (photo_license, set_photo_license) = signal(String::new());
    let (photo_license_url, set_photo_license_url) = signal(String::new());
    let (photo_is_cc, set_photo_is_cc) = signal(true);

    let fetch_flickr = ServerAction::<crate::server::FetchFlickrPhoto>::new();
    let add_photo_action = ServerAction::<crate::server::AddPubPhoto>::new();

    Effect::new(move |_| {
        if let Some(Ok(info)) = fetch_flickr.value().get() {
            set_photo_title.set(info.title);
            set_photo_owner.set(info.owner_name);
            set_photo_image_url.set(info.image_url);
            set_photo_original_url.set(info.original_url);
            set_photo_license.set(info.license_type);
            set_photo_license_url.set(info.license_url);
            set_photo_is_cc.set(info.is_cc_licensed);
        }
    });

    let on_fetch_flickr = move |_| {
        fetch_flickr.dispatch(crate::server::FetchFlickrPhoto {
            url_or_id: flickr_url.get(),
        });
    };

    let current_year = chrono::Datelike::year(&chrono::Local::now());
    let all_years: Vec<i32> = (1972..=current_year).rev().collect();

    let toggle_year = move |year: i32| {
        set_years.update(|y| {
            if y.contains(&year) {
                y.retain(|&x| x != year);
            } else {
                y.push(year);
                y.sort_by(|a, b| b.cmp(a));
            }
        });
    };

    let on_submit = move |ev: leptos::web_sys::SubmitEvent| {
        ev.prevent_default();
        update_action.dispatch(UpdatePub {
            id: pub_data.id,
            name: name.get(),
            address: address.get(),
            town: town.get(),
            region: region.get(),
            country_code: pub_data.country_code.clone(),
            postcode: postcode.get(),
            closed: closed.get(),
            lat: lat.get(),
            lon: lon.get(),
            untappd_id: untappd_id.get(),
            google_maps_id: google_maps_id.get(),
            whatpub_id: whatpub_id.get(),
            rgl_id: rgl_id.get(),
            years: years.get(),
        });

        if !photo_image_url.get().is_empty() && photo_is_cc.get() {
            add_photo_action.dispatch(crate::server::AddPubPhoto {
                pub_id: pub_data.id,
                flickr_info: crate::models::FlickrPhotoInfo {
                    flickr_id: flickr_url.get(), // Might be empty if manual
                    title: photo_title.get(),
                    owner_name: photo_owner.get(),
                    image_url: photo_image_url.get(),
                    original_url: photo_original_url.get(),
                    license_type: photo_license.get(),
                    license_url: photo_license_url.get(),
                    is_cc_licensed: photo_is_cc.get(),
                }
            });
        }
    };

    Effect::new(move |_| {
        if let Some(Ok(())) = update_action.value().get() {
            on_close.run(());
        }
    });

    view! {
        <div class="edit-pub-modal">
            <div class="modal-content">
                <div class="modal-header">
                    <h3>"Edit Pub: " {move || pub_data.name.clone()}</h3>
                    <button class="btn btn-ghost close-btn" on:click=move |_| on_close.run(())>"×"</button>
                </div>
                <form on:submit=on_submit class="edit-form">
                    <div class="form-grid">
                        <div class="form-group">
                            <label>"Name"</label>
                            <input type="text" value=name on:input=move |ev| set_name.set(event_target_value(&ev)) required />
                        </div>
                        <div class="form-group">
                            <label>"Town"</label>
                            <input type="text" value=town on:input=move |ev| set_town.set(event_target_value(&ev)) required />
                        </div>
                        <div class="form-group">
                            <label>"Region"</label>
                            <input type="text" value=region on:input=move |ev| set_region.set(event_target_value(&ev)) required />
                        </div>
                        <div class="form-group">
                            <label>"Postcode"</label>
                            <input type="text" value=postcode on:input=move |ev| set_postcode.set(event_target_value(&ev)) required />
                        </div>
                        <div class="form-group full-width">
                            <label>"Address"</label>
                            <textarea on:input=move |ev| set_address.set(event_target_value(&ev))>{move || address.get()}</textarea>
                        </div>
                        <div class="form-group">
                            <label>"Latitude"</label>
                            <input type="number" step="any" value=move || lat.get().map(|v| v.to_string()).unwrap_or_default() 
                                on:input=move |ev| set_lat.set(event_target_value(&ev).parse().ok()) />
                        </div>
                        <div class="form-group">
                            <label>"Longitude"</label>
                            <input type="number" step="any" value=move || lon.get().map(|v| v.to_string()).unwrap_or_default() 
                                on:input=move |ev| set_lon.set(event_target_value(&ev).parse().ok()) />
                        </div>
                        <div class="form-group checkbox">
                            <label>
                                <input type="checkbox" checked=closed on:change=move |ev| set_closed.set(event_target_checked(&ev)) />
                                " Reported Closed"
                            </label>
                        </div>
                    </div>

                    <h4>"External IDs"</h4>
                    <div class="form-grid">
                        <div class="form-group">
                            <label>"Untappd ID"</label>
                            <input type="text" value=move || untappd_id.get().unwrap_or_default() 
                                on:input=move |ev| set_untappd_id.set(Some(event_target_value(&ev))) />
                        </div>
                        <div class="form-group">
                            <label>"WhatPub ID"</label>
                            <input type="text" value=move || whatpub_id.get().unwrap_or_default() 
                                on:input=move |ev| set_whatpub_id.set(Some(event_target_value(&ev))) />
                        </div>
                        <div class="form-group">
                            <label>"Google Maps ID"</label>
                            <div class="input-with-button">
                                <input type="text" value=move || google_maps_id.get().unwrap_or_default() 
                                    on:input=move |ev| set_google_maps_id.set(Some(event_target_value(&ev))) />
                                <button type="button" class="btn btn-secondary btn-sm" on:click=move |_| {
                                    let _ = window().open_with_url_and_target("https://developers.google.com/maps/documentation/javascript/examples/places-placeid-finder", "_blank");
                                }>"Find ID"</button>
                            </div>
                            <p class="field-helper">
                                {move || format!("Search for '{}, {}' in the finder tool.", name.get(), postcode.get())}
                            </p>
                        </div>
                        <div class="form-group">
                            <label>"RGL ID"</label>
                            <input type="text" value=move || rgl_id.get().unwrap_or_default() 
                                on:input=move |ev| set_rgl_id.set(Some(event_target_value(&ev))) />
                        </div>
                    </div>

                    <h4>"GBG History"</h4>
                    <div class="history-grid">
                        {all_years.into_iter().map(|year| {
                            let is_checked = move || years.get().contains(&year);
                            view! {
                                <label class="year-toggle">
                                    <input type="checkbox" checked=is_checked on:change=move |_| toggle_year(year) />
                                    {year}
                                    {if year == 1972 { " (Trial)" } else { "" }}
                                </label>
                            }
                        }).collect_view()}
                    </div>

                    <h4>"Add Photo"</h4>
                    <div class="photo-management">
                        <div class="form-group">
                            <label>"Flickr URL or Photo ID"</label>
                            <div class="input-with-button">
                                <input type="text" value=flickr_url on:input=move |ev| set_flickr_url.set(event_target_value(&ev)) placeholder="https://www.flickr.com/photos/..." />
                                <button type="button" class="btn btn-secondary btn-sm" on:click=on_fetch_flickr disabled=fetch_flickr.pending()>
                                    {move || if fetch_flickr.pending().get() { "Fetching..." } else { "Fetch Details" }}
                                </button>
                            </div>
                        </div>

                        <div class="form-grid">
                            <div class="form-group full-width">
                                <label>"Photo Title"</label>
                                <input type="text" value=photo_title on:input=move |ev| set_photo_title.set(event_target_value(&ev)) />
                            </div>
                            <div class="form-group">
                                <label>"Image URL (Direct link)"</label>
                                <input type="text" value=photo_image_url on:input=move |ev| set_photo_image_url.set(event_target_value(&ev)) />
                            </div>
                            <div class="form-group">
                                <label>"Owner/Attribution Name"</label>
                                <input type="text" value=photo_owner on:input=move |ev| set_photo_owner.set(event_target_value(&ev)) />
                            </div>
                            <div class="form-group">
                                <label>"Original Source URL"</label>
                                <input type="text" value=photo_original_url on:input=move |ev| set_photo_original_url.set(event_target_value(&ev)) />
                            </div>
                            <div class="form-group">
                                <label>"License Type"</label>
                                <input type="text" value=photo_license on:input=move |ev| set_photo_license.set(event_target_value(&ev)) />
                            </div>
                            <div class="form-group">
                                <label>"License URL"</label>
                                <input type="text" value=photo_license_url on:input=move |ev| set_photo_license_url.set(event_target_value(&ev)) />
                            </div>
                            <div class="form-group checkbox">
                                <label>
                                    <input type="checkbox" checked=photo_is_cc on:change=move |ev| set_photo_is_cc.set(event_target_checked(&ev)) />
                                    " Creative Commons Licensed"
                                </label>
                            </div>
                        </div>
                        {move || if !photo_image_url.get().is_empty() {
                            view! {
                                <div class="photo-preview">
                                    <img src=photo_image_url.get() style="max-width: 200px; border-radius: 4px;" />
                                </div>
                            }.into_any()
                        } else {
                            ().into_any()
                        }}
                    </div>

                    <div class="form-actions">
                        <button type="submit" class="btn btn-primary" disabled=update_action.pending()>
                            {move || if update_action.pending().get() { "Saving..." } else { "Save Changes" }}
                        </button>
                        <button type="button" class="btn btn-ghost" on:click=move |_| on_close.run(())>"Cancel"</button>
                    </div>
                    {move || update_action.value().get().map(|v| {
                        if let Err(e) = v {
                            view! { <p class="error">{e.to_string()}</p> }.into_any()
                        } else {
                            ().into_any()
                        }
                    })}
                </form>
            </div>
        </div>
    }
}
