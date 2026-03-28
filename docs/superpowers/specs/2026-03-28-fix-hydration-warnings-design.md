# Design: Fix Hydration Mismatch Warnings

This document outlines the strategy for resolving hydration mismatch warnings in the Leptos 0.8 application by ensuring all resource reads within `view!` macros are properly wrapped in `<Suspense>` or `<Transition>` boundaries.

## 1. Problem
In Leptos 0.8, reading a resource (like `user.get()`) during hydration outside of a specialized component (like `<Suspense/>`) causes mismatch errors because the client-side initial state might not align with the server-rendered HTML before the resource has fully "re-hydrated".

## 2. Solution
Wrap all UI elements that depend on the `user` resource in `<Suspense>` boundaries with minimal fallbacks (usually `|| ()`). This allows Leptos to track the resource access reactively and coordinate the hydration process correctly.

## 3. Targeted Components

### 3.1 `pub_list.rs`
- **Location**: Bulk edit toggle button.
- **Fix**: Wrap the `<Show>` component that checks `user.role` in a `<Suspense fallback=|| ()>`.

### 3.2 `pub_detail.rs`
- **Location**: Admin suggestion banner, Suggest Update button, Edit button, and Log Visit button.
- **Fix**: Wrap each of these conditional sections in `<Suspense fallback=|| ()>`.

### 3.3 `setup_2fa.rs`
- **Location**: Hidden input for `user_id` inside the 2FA verification form.
- **Fix**: Ensure the entire `verify-section` or at least the form is contained within a boundary that safely accesses the `user` resource.

### 3.4 `admin.rs`
- **Location**: Main transition that matches on `user.get()`.
- **Fix**: Verify that the existing `<Transition>` correctly handles the resource read and ensure no nested reads escape the boundary.

### 3.5 `profile.rs`
- **Location**: User profile data display.
- **Fix**: Ensure the existing `<Suspense>` fully encapsulates all calls to `user.get()`.

## 4. Implementation Steps
1.  **Surgical Updates**: Apply `<Suspense>` wrappers to the identified components.
2.  **Verification**: 
    *   Run `cargo check` to ensure no syntax errors.
    *   Observe browser console logs during local development to confirm warnings are gone.
