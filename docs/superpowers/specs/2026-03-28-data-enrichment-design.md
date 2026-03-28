# Design Spec: Data Enrichment (Links, Google ID, Flickr)

**Date:** 2026-03-28
**Status:** Draft
**Topic:** Fallback search links, Google Maps ID helper, and Flickr photo management.

---

## 1. Objective
Enhance the accuracy and richness of pub data by providing easy ways to find missing external IDs (WhatPub, Google, Untappd) and integrating Flickr photos with mandatory CC license tracking.

## 2. User Interface Designs

### 2.1 `PubDetail` Fallback Search Links
When external IDs are missing in `pubs` table, show a "Search" fallback in the "Links" section.
- **WhatPub**: `https://camra.org.uk/pubs/?pub_search={Name}+{Postcode}`
- **Google Maps**: `https://www.google.com/maps/search/{Name}+{Postcode}/`
- **Untappd**: `https://untappd.com/search?q={Name}+{Town}&type=venue&sort=`

### 2.2 `EditPub` Google Maps ID Helper
- **Find ID Button**: A button next to the Google Maps ID field.
- **Action**: Opens `https://developers.google.com/maps/documentation/javascript/examples/places-placeid-finder` in a new tab.
- **Helper Text**: "Search for '{Name}, {Postcode}' in the finder tool to get the Place ID."

### 2.3 `EditPub` Flickr & Photo Management
A new "Photos" section in the `EditPub` modal.

#### 2.3.1 Flickr Auto-Fetch
- **Input**: "Flickr URL or ID" text field.
- **Action**: "Fetch Details" button.
- **Logic**: Calls `FetchFlickrPhoto` server function. If successful, auto-populates the manual fields below.

#### 2.3.2 Manual Photo Entry (Override)
Available if fetch fails or for non-Flickr sources:
- **Image URL**: Direct link to the image file.
- **Title/Caption**: Short description.
- **Owner Name**: For attribution.
- **Original URL**: Link back to the source page (e.g., Flickr page).
- **License Type**: e.g., "CC BY-NC-SA 2.0".
- **License URL**: Link to the license text.
- **CC Licensed Toggle**: Boolean (Must be true to save).

### 2.4 `PubDetail` Photo Display
- **Placement**: A new "Photos" section below the map or stats.
- **Attribution**: "Photo: {Title} by {Owner} ({License})". Link the Title to `Original URL` and License to `License URL`.

## 3. Data Flow & Integration

### 3.1 Server Functions
- **`FetchFlickrPhoto`**: Already exists. Returns `FlickrPhotoInfo`.
- **`AddPubPhoto`**: Already exists. Inserts into `pub_photos`.
- **`GetPubPhotos`**: Already exists. Fetches all photos for a pub.
- **`UpdatePub`**: Already modified to handle ID updates.

### 3.2 Security & Validation
- **Authentication**: Only logged-in users/admins can add photos or edit IDs.
- **License Enforcement**: The UI and Server Functions must reject any photo that is not marked as CC licensed.

## 4. Implementation Phases
1. **Phase 1**: Implement fallback search links in `PubDetail`.
2. **Phase 2**: Add Google Place ID helper button and text to `EditPub`.
3. **Phase 3**: Implement Flickr Fetch and Manual Photo entry UI in `EditPub`.
4. **Phase 4**: Add Photo display section with attribution to `PubDetail`.
