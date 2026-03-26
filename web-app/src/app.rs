use crate::components::pub_list::PubList;
use crate::components::pub_detail::PubDetail;
use crate::components::near_me::NearMe;
use crate::components::explorer::{ExplorerHome, CountyDashboard, LocationPubList};
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    path,
    components::A,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    provide_meta_context();
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <Meta name="description" content="Good Beer Guide Pub Explorer"/>
                <link rel="manifest" href="/assets/manifest.json"/>
                <script>
                    "if ('serviceWorker' in navigator) { window.addEventListener('load', () => { navigator.serviceWorker.register('/assets/sw.js'); }); }"
                </script>
                <Stylesheet id="leptos" href="/pkg/web-app.css"/>
                <Title text="gbgdata - Pub Explorer"/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    view! {
        <Router>
            <main>
                <nav>
                    <A href="/">"Home"</A>
                    " | "
                    <A href="/explore">"Explore"</A>
                    " | "
                    <A href="/near-me">"Near Me"</A>
                </nav>
                <Routes fallback=|| view! { "Page not found." }>
                    <Route path=path!("/") view=PubList/>
                    <Route path=path!("/near-me") view=NearMe/>
                    <Route path=path!("/pub/:id") view=PubDetail/>
                    
                    <Route path=path!("/explore") view=ExplorerHome/>
                    <Route path=path!("/explore/:county") view=CountyDashboard/>
                    <Route path=path!("/explore/:county/town/:town") view=LocationPubList/>
                    <Route path=path!("/explore/:county/outcode/:outcode") view=LocationPubList/>
                </Routes>
            </main>
        </Router>
    }
}
