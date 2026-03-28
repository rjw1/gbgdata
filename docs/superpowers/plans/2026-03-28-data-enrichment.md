# Data Enrichment Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Implement fallback search links, a Google Place ID helper, and a Flickr/Manual photo management UI with CC license enforcement.

**Architecture:** Extend `PubDetail` for link fallbacks and photo display. Extend `EditPub` with a new photo management section and Google Maps ID helper.

**Tech Stack:** Rust (Leptos 0.8.0, Axum, SQLx), Flickr API (already implemented in backend).

---

### Task 1: Fallback Search Links in PubDetail

**Files:**
- Modify: `web-app/src/components/pub_detail.rs`

- [ ] **Step 1: Implement fallback link logic**

Modify the external links section to show search links when IDs are missing.

```rust
// web-app/src/components/pub_detail.rs
// Inside the view logic...
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
```

- [ ] **Step 2: Commit**

```bash
git add web-app/src/components/pub_detail.rs
git commit -m "feat: add fallback search links to PubDetail"
```

---

### Task 2: Google Place ID Helper in EditPub

**Files:**
- Modify: `web-app/src/components/edit_pub.rs`

- [ ] **Step 1: Add "Find ID" button and helper text**

```rust
// web-app/src/components/edit_pub.rs
<div class="form-group">
    <label>"Google Maps ID"</label>
    <div class="input-with-button">
        <input type="text" value=move || google_maps_id.get().unwrap_or_default() 
            on:input=move |ev| set_google_maps_id.set(Some(event_target_value(&ev))) />
        <button type="button" class="helper-btn" on:click=move |_| {
            let _ = window().open_with_url_and_target("https://developers.google.com/maps/documentation/javascript/examples/places-placeid-finder", "_blank");
        }>"Find ID"</button>
    </div>
    <p class="field-helper">
        {move || format!("Search for '{}, {}' in the finder tool.", name.get(), postcode.get())}
    </p>
</div>
```

- [ ] **Step 2: Commit**

```bash
git add web-app/src/components/edit_pub.rs
git commit -m "feat: add Google Place ID helper to EditPub"
```

---

### Task 3: Flickr & Manual Photo Management in EditPub

**Files:**
- Modify: `web-app/src/components/edit_pub.rs`
- Modify: `web-app/src/server.rs` (to add photo saving logic if missing)

- [ ] **Step 1: Define Photo state signals in EditPub**

```rust
let (flickr_url, set_flickr_url) = signal(String::new());
let (photo_title, set_photo_title) = signal(String::new());
let (photo_owner, set_photo_owner) = signal(String::new());
let (photo_image_url, set_photo_image_url) = signal(String::new());
let (photo_original_url, set_photo_original_url) = signal(String::new());
let (photo_license, set_photo_license) = signal(String::new());
let (photo_license_url, set_photo_license_url) = signal(String::new());
let (photo_is_cc, set_photo_is_cc) = signal(true);
```

- [ ] **Step 2: Implement Flickr Fetch logic**

```rust
let fetch_flickr = ServerAction::<crate::server::FetchFlickrPhoto>::new();

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
```

- [ ] **Step 3: Add Photo Section to UI**

```rust
<h4>"Photos"</h4>
<div class="photo-management">
    <div class="flickr-fetch">
        <label>"Flickr URL/ID"</label>
        <div class="input-with-button">
            <input type="text" value=flickr_url on:input=move |ev| set_flickr_url.set(event_target_value(&ev)) />
            <button type="button" on:click=on_fetch_flickr disabled=fetch_flickr.pending()>"Fetch"</button>
        </div>
    </div>
    
    // Manual fields (Title, Owner, Image URL, etc.)
    // ...
</div>
```

- [ ] **Step 4: Update `on_submit` to save photo**

Call `add_pub_photo` server function if a photo URL is present.

- [ ] **Step 5: Commit**

```bash
git add web-app/src/components/edit_pub.rs
git commit -m "feat: implement Flickr and manual photo management in EditPub"
```

---

### Task 4: Photo Display in PubDetail

**Files:**
- Modify: `web-app/src/components/pub_detail.rs`

- [ ] **Step 1: Fetch and display photos**

Add `Resource` to fetch photos and render them with attribution.

```rust
let photos = Resource::new(move || id(), |id| async move {
    match id {
        Some(uuid) => crate::server::get_pub_photos(uuid).await,
        None => Ok(vec![]),
    }
});

// In the view...
<Suspense fallback=|| view! { <p>"Loading photos..."</p> }>
    {move || photos.get().map(|res| {
        match res {
            Ok(p_list) => view! {
                <div class="pub-photos">
                    {p_list.into_iter().map(|p| view! {
                        <div class="photo-item">
                            <img src=p.image_url alt=p.owner_name />
                            <p class="attribution">
                                "Photo by " <a href=p.license_url target="_blank">{p.owner_name}</a>
                                " (" {p.license_type} ")"
                            </p>
                        </div>
                    }).collect_view()}
                </div>
            }.into_any(),
            _ => ().into_any(),
        }
    })}
</Suspense>
```

- [ ] **Step 2: Commit**

```bash
git add web-app/src/components/pub_detail.rs
git commit -m "feat: display pub photos with attribution in PubDetail"
```
