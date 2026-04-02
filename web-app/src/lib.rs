#![recursion_limit = "2048"]
pub mod app;
pub mod auth;
pub mod components;
pub mod export;
pub mod models;
pub mod server;

#[cfg(feature = "hydrate")]
#[wasm_bindgen::prelude::wasm_bindgen]
pub fn hydrate() {
    use crate::app::*;
    console_error_panic_hook::set_once();
    leptos::mount::hydrate_body(App);
}
#[cfg(all(test, feature = "ssr"))]
mod tests;
