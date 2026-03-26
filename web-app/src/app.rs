use crate::components::pub_list::PubList;
use crate::components::pub_detail::PubDetail;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, Stylesheet, Title};
use leptos_router::{
    components::{Route, Router, Routes},
    StaticSegment, ParamSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone() />
                <HydrationScripts options/>
                <Meta name="description" content="Good Beer Guide Pub Explorer"/>
                <Meta name="google" content="notranslate"/>
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
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Stylesheet id="leptos" href="/pkg/web-app.css"/>

        // sets the document title
        <Title text="gbgdata"/>

        // content for this welcome page
        <Router>
            <main>
                <nav>
                    <a href="/">"Home"</a>
                </nav>
                <Routes fallback=|| view! { "Page not found." }>
                    <Route path=StaticSegment("") view=PubList/>
                    <Route path=(StaticSegment("pub"), ParamSegment("id")) view=PubDetail/>
                </Routes>
            </main>
        </Router>
    }
}
