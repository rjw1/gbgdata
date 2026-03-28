use leptos::prelude::*;
use crate::models::PubDetail;
use crate::server::SuggestUpdate;

#[derive(Clone, Copy, PartialEq, Default)]
enum SuggestionCategory {
    #[default]
    Menu,
    Closed,
    Links,
    Location,
    History,
    General,
}

#[component]
pub fn SuggestUpdateModal(pub_data: PubDetail, on_close: Callback<()>) -> impl IntoView {
    let (category, set_category) = signal(SuggestionCategory::Menu);
    let suggest_action = ServerAction::<SuggestUpdate>::new();
    
    // Form state (initialized with current values)
    let (closed, set_closed) = signal(pub_data.closed);
    let (whatpub_id, set_whatpub_id) = signal(pub_data.whatpub_id.clone());
    let (google_maps_id, set_google_maps_id) = signal(pub_data.google_maps_id.clone());
    let (untappd_id, set_untappd_id) = signal(pub_data.untappd_id.clone());
    let (address, set_address) = signal(pub_data.address.clone());
    let (town, set_town) = signal(pub_data.town.clone());
    let (region, set_region) = signal(pub_data.region.clone());
    let (postcode, set_postcode) = signal(pub_data.postcode.clone());
    let (years, set_years) = signal(pub_data.years.clone());
    let (notes, set_notes) = signal(String::new());

    let current_year = chrono::Datelike::year(&chrono::Local::now());
    let all_years = Memo::new(move |_| (1972..=current_year).rev().collect::<Vec<i32>>());

    let on_submit = move |ev: leptos::web_sys::SubmitEvent| {
        ev.prevent_default();
        
        let mut final_data = serde_json::to_value(pub_data.clone()).unwrap();
        let obj = final_data.as_object_mut().unwrap();
        
        // Apply modifications
        obj.insert("closed".to_string(), serde_json::Value::Bool(closed.get()));
        obj.insert("whatpub_id".to_string(), serde_json::to_value(whatpub_id.get()).unwrap());
        obj.insert("google_maps_id".to_string(), serde_json::to_value(google_maps_id.get()).unwrap());
        obj.insert("untappd_id".to_string(), serde_json::to_value(untappd_id.get()).unwrap());
        obj.insert("address".to_string(), serde_json::Value::String(address.get()));
        obj.insert("town".to_string(), serde_json::Value::String(town.get()));
        obj.insert("region".to_string(), serde_json::Value::String(region.get()));
        obj.insert("postcode".to_string(), serde_json::Value::String(postcode.get()));
        obj.insert("years".to_string(), serde_json::to_value(years.get()).unwrap());
        
        // Add meta notes if any
        if !notes.get().is_empty() {
            obj.insert("suggestion_notes".to_string(), serde_json::Value::String(notes.get()));
        }

        suggest_action.dispatch(SuggestUpdate {
            pub_id: pub_data.id,
            suggested_data: final_data,
        });
    };

    let on_submit_stored = StoredValue::new(on_submit);

    Effect::new(move |_| {
        if let Some(Ok(())) = suggest_action.value().get() {
            on_close.run(());
        }
    });

    view! {
        <div class="edit-pub-modal">
            <div class="modal-content">
                <div class="modal-header">
                    <h3>"Suggest an Update"</h3>
                    <button class="btn btn-ghost close-btn" on:click=move |_| on_close.run(())>"×"</button>
                </div>

                <div class="suggestion-wizard">
                    <Show when=move || category.get() == SuggestionCategory::Menu>
                        <p>"What would you like to correct?"</p>
                        <div class="category-grid">
                            <button class="category-card" on:click=move |_| set_category.set(SuggestionCategory::Closed)>
                                <h3>"🚫 Report Closed"</h3>
                            </button>
                            <button class="category-card" on:click=move |_| set_category.set(SuggestionCategory::Links)>
                                <h3>"🔗 Add/Fix Links"</h3>
                            </button>
                            <button class="category-card" on:click=move |_| set_category.set(SuggestionCategory::Location)>
                                <h3>"📍 Location/Address"</h3>
                            </button>
                            <button class="category-card" on:click=move |_| set_category.set(SuggestionCategory::History)>
                                <h3>"📅 GBG History"</h3>
                            </button>
                            <button class="category-card" on:click=move |_| set_category.set(SuggestionCategory::General)>
                                <h3>"📝 General"</h3>
                            </button>
                        </div>
                    </Show>

                    <Show when=move || category.get() != SuggestionCategory::Menu>
                        <form on:submit=move |ev| on_submit_stored.with_value(|f| f(ev)) class="edit-form">
                            {move || match category.get() {
                                SuggestionCategory::Closed => view! {
                                    <div class="form-group checkbox">
                                        <label>
                                            <input type="checkbox" checked=closed on:change=move |ev| set_closed.set(event_target_checked(&ev)) />
                                            " Mark as Closed"
                                        </label>
                                    </div>
                                }.into_any(),
                                SuggestionCategory::Links => view! {
                                    <div class="form-grid">
                                        <div class="form-group">
                                            <label>"WhatPub ID"</label>
                                            <input type="text" value=whatpub_id on:input=move |ev| set_whatpub_id.set(Some(event_target_value(&ev))) />
                                        </div>
                                        <div class="form-group">
                                            <label>"Google Place ID"</label>
                                            <input type="text" value=google_maps_id on:input=move |ev| set_google_maps_id.set(Some(event_target_value(&ev))) />
                                        </div>
                                        <div class="form-group">
                                            <label>"Untappd Venue ID"</label>
                                            <input type="text" value=untappd_id on:input=move |ev| set_untappd_id.set(Some(event_target_value(&ev))) />
                                        </div>
                                    </div>
                                }.into_any(),
                                SuggestionCategory::Location => view! {
                                    <div class="form-grid">
                                        <div class="form-group full-width">
                                            <label>"Address"</label>
                                            <input type="text" value=address on:input=move |ev| set_address.set(event_target_value(&ev)) />
                                        </div>
                                        <div class="form-group">
                                            <label>"Town"</label>
                                            <input type="text" value=town on:input=move |ev| set_town.set(event_target_value(&ev)) />
                                        </div>
                                        <div class="form-group">
                                            <label>"Region"</label>
                                            <input type="text" value=region on:input=move |ev| set_region.set(event_target_value(&ev)) />
                                        </div>
                                        <div class="form-group">
                                            <label>"Postcode"</label>
                                            <input type="text" value=postcode on:input=move |ev| set_postcode.set(event_target_value(&ev)) />
                                        </div>
                                    </div>
                                }.into_any(),
                                SuggestionCategory::History => view! {
                                    <div class="history-grid">
                                        {all_years.get().into_iter().map(|year| {
                                            let is_checked = move || years.get().contains(&year);
                                            view! {
                                                <label class="year-toggle">
                                                    <input type="checkbox" checked=is_checked on:change=move |_| {
                                                        set_years.update(|y| {
                                                            if y.contains(&year) { y.retain(|&x| x != year); } else { y.push(year); y.sort_by(|a, b| b.cmp(a)); }
                                                        });
                                                    } />
                                                    {year}
                                                </label>
                                            }
                                        }).collect_view()}
                                    </div>
                                }.into_any(),
                                _ => ().into_any(),
                            }}

                            <div class="form-group full-width">
                                <label>"Optional Notes / Explanation"</label>
                                <textarea on:input=move |ev| set_notes.set(event_target_value(&ev)) placeholder="Explain your suggestion..."></textarea>
                            </div>

                            <div class="form-actions">
                                <button type="submit" class="btn btn-primary" disabled=suggest_action.pending()>
                                    {move || if suggest_action.pending().get() { "Submitting..." } else { "Submit Suggestion" }}
                                </button>
                                <button type="button" class="btn btn-ghost" on:click=move |_| set_category.set(SuggestionCategory::Menu)>"Back"</button>
                            </div>
                        </form>
                    </Show>
                </div>
            </div>
        </div>
    }
}
