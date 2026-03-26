use crate::components::pub_list::PubList;
use crate::components::pub_detail::PubDetail;
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
    // We don't call provide_meta_context() here because it's called in shell for SSR
    // and Leptos will handle it during hydration.

    view! {
        <Router>
            <main>
                <nav>
                    <A href="/">"Home"</A>
                </nav>
                <Routes fallback=|| view! { "Page not found." }>
                    <Route path=path!("/") view=PubList/>
                    <Route path=path!("/pub/:id") view=PubDetail/>
                </Routes>
            </main>
        </Router>
    }
}
