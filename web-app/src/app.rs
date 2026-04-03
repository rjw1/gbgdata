use crate::components::about::About;
use crate::components::admin::AdminDashboard;
use crate::components::explorer::{ExplorerHome, LocationPubList, RegionDashboard, YearDashboard};
use crate::components::login::LoginForm;
use crate::components::my_visits::MyVisits;
use crate::components::near_me::NearMe;
use crate::components::profile::Profile;
use crate::components::pub_detail::PubDetail;
use crate::components::pub_list::PubList;
use crate::components::rankings::Rankings;
use crate::components::register::RegisterPage;
use crate::components::setup_2fa::Setup2FA;
use crate::server::Logout;
use leptos::prelude::*;
use leptos_meta::{provide_meta_context, Meta, Stylesheet, Title};
use leptos_router::{
    components::A,
    components::{Route, Router, Routes},
    path,
};

#[component]
pub fn ThemeToggle() -> impl IntoView {
    let (theme, set_theme) = signal(String::from("system"));

    Effect::new(move |_| {
        let storage = window().local_storage().ok().flatten();
        if let Some(s) = storage {
            if let Ok(Some(saved)) = s.get_item("theme") {
                set_theme.set(saved);
            }
        }
    });

    Effect::new(move |_| {
        let t = theme.get();
        let document = document().document_element().expect("no document element");
        let storage = window().local_storage().ok().flatten();

        if t == "system" {
            let _ = document.remove_attribute("data-theme");
        } else {
            let _ = document.set_attribute("data-theme", &t);
        }

        if let Some(s) = storage {
            let _ = s.set_item("theme", &t);
        }
    });

    let toggle = move |_| {
        set_theme.update(|t| {
            *t = match t.as_str() {
                "light" => "dark".to_string(),
                "dark" => "system".to_string(),
                _ => "light".to_string(),
            };
        });
    };

    view! {
        <button on:click=toggle class="theme-toggle">
            {move || match theme.get().as_str() {
                "light" => "☀️ Light",
                "dark" => "🌙 Dark",
                _ => "🌓 Auto",
            }}
        </button>
    }
}

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
                <Meta name="robots" content="noindex, nofollow, noarchive, noai, noimageai"/>
                <Meta name="googlebot" content="noindex, nofollow, noarchive, noai, noimageai"/>
                <Meta name="bingbot" content="noindex, nofollow, noarchive, noai, noimageai"/>
                <link rel="manifest" href="/assets/manifest.json"/>
                <script src="/assets/service-worker-reg.js" defer></script>
                <script src="/assets/theme-init.js"></script>
                <Stylesheet id="leptos" href="/pkg/web-app.css"/>
                <link rel="stylesheet" href="https://unpkg.com/leaflet@1.9.4/dist/leaflet.css" integrity="sha256-p4NxAoJBhIIN+hmNHrzRCf9tD/miZyoHS5obTRR9BMY=" crossorigin=""/>
                <script src="https://unpkg.com/leaflet@1.9.4/dist/leaflet.js" integrity="sha256-20nQCchB9co0qIjJZRGuk2/Z9VM+kNiyxNV1lvTlZBo=" crossorigin=""></script>
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
            <RouterContent />
        </Router>
    }
}

#[component]
fn RouterContent() -> impl IntoView {
    let user = Resource::new(|| (), |_| crate::server::get_current_user());
    let site_settings = Resource::new(|| (), |_| crate::server::get_site_settings());
    let navigate = leptos_router::hooks::use_navigate();
    let location = leptos_router::hooks::use_location();

    Effect::new(move |_| {
        if let Some(Ok(Some(u))) = user.get() {
            let path = location.pathname.get();
            if !u.totp_setup_completed
                && path != "/setup-2fa"
                && path != "/login"
                && path != "/about"
            {
                navigate("/setup-2fa", Default::default());
            }
        }
    });

    let logout_action = ServerAction::<Logout>::new();

    view! {
        <main>
            <nav>
                <div class="nav-links">
                    <A href="/">"Home"</A>
                    " | "
                    <A href="/explore">"Explore"</A>
                    " | "
                    <A href="/near-me">"Near Me"</A>
                    " | "
                    <A href="/my-visits">"My Visits"</A>
                    " | "
                    <A href="/rankings">"Rankings"</A>
                    " | "
                    <A href="/about">"About"</A>
                    " | "
                    <A href="/profile">"Profile"</A>
                    <Suspense fallback=|| ()>
                        <Show when=move || matches!(site_settings.get(), Some(Ok(s)) if s.private_mode)>
                            " | "
                            <span class="badge-private">"PRIVATE"</span>
                        </Show>
                    </Suspense>
                    <Suspense fallback=|| ()>
                        <Show when=move || matches!(user.get(), Some(Ok(Some(ref u))) if u.role == "admin" || u.role == "owner")>
                            " | "
                            <A href="/admin">"Admin"</A>
                        </Show>
                    </Suspense>
                    <Suspense fallback=|| view! { " | " <A href="/login">"Login"</A> }>
                        {move || match user.get() {
                            Some(Ok(Some(_))) => view! {
                                " | "
                                <ActionForm action=logout_action>
                                    <button type="submit" class="logout-link">"Logout"</button>
                                </ActionForm>
                            }.into_any(),
                            _ => view! {
                                " | "
                                <A href="/login">"Login"</A>
                            }.into_any(),
                        }}
                    </Suspense>
                </div>
                <ThemeToggle />
            </nav>

            <Routes fallback=|| view! { "Page not found." }>
                <Route path=path!("/") view=PubList/>
                <Route path=path!("/login") view=LoginForm/>
                <Route path=path!("/register") view=RegisterPage/>
                <Route path=path!("/setup-2fa") view=Setup2FA/>
                <Route path=path!("/profile") view=Profile/>
                <Route path=path!("/admin") view=AdminDashboard/>
                <Route path=path!("/near-me") view=NearMe/>
                <Route path=path!("/my-visits") view=MyVisits/>
                <Route path=path!("/rankings") view=Rankings/>
                <Route path=path!("/about") view=About/>
                <Route path=path!("/pub/:id") view=PubDetail/>

                <Route path=path!("/explore") view=ExplorerHome/>
                <Route path=path!("/explore/:region/all") view=LocationPubList/>
                <Route path=path!("/explore/:region/town/:town") view=LocationPubList/>
                <Route path=path!("/explore/:region/outcode/:outcode") view=LocationPubList/>
                <Route path=path!("/explore/:region") view=RegionDashboard/>

                <Route path=path!("/explore/year/:year") view=YearDashboard/>
                <Route path=path!("/explore/year/:year/:region/all") view=LocationPubList/>
                <Route path=path!("/explore/year/:year/:region/town/:town") view=LocationPubList/>
                <Route path=path!("/explore/year/:year/:region/outcode/:outcode") view=LocationPubList/>
                <Route path=path!("/explore/year/:year/:region") view=RegionDashboard/>
            </Routes>
            <footer>
                <p>"Note: Pub locations are determined via automated geocoding and may not be 100% accurate. Distance calculations are estimates."</p>
                <p>"Disclaimer: While we track if a pub has been reported as closed, this information may be out of date. Always check with the pub before visiting."</p>
            </footer>
        </main>
    }
}
