use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use crate::server::{get_counties, get_county_details, get_pubs_by_location};

#[component]
pub fn Breadcrumbs(
    county: Option<String>,
    town: Option<String>,
    outcode: Option<String>,
) -> impl IntoView {
    view! {
        <nav class="breadcrumbs">
            <A href="/">"Home"</A>
            " > "
            <A href="/explore">"Explore"</A>
            {move || county.clone().map(|c| view! { 
                " > " <A href=format!("/explore/{}", c)>{c}</A> 
            })}
            {move || town.clone().map(|t| view! { " > " {t} })}
            {move || outcode.clone().map(|o| view! { " > " {o} })}
        </nav>
    }
}

#[component]
pub fn ExplorerHome() -> impl IntoView {
    let counties = Resource::new(|| (), |_| async move { get_counties().await });

    view! {
        <div class="explorer-container">
            <Breadcrumbs county=None town=None outcode=None />
            <h1>"Browse by County"</h1>
            <Suspense fallback=|| view! { <p>"Loading counties..."</p> }>
                {move || counties.get().map(|res| match res {
                    Ok(list) => view! {
                        <div class="category-grid">
                            {list.into_iter().map(|c| {
                                let name = c.name.clone();
                                view! {
                                    <A href=format!("/explore/{}", name) class="category-card">
                                        <h3>{name}</h3>
                                        <p>{c.pub_count} " Pubs"</p>
                                    </A>
                                }
                            }).collect_view()}
                        </div>
                    }.into_any(),
                    Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
                })}
            </Suspense>
        </div>
    }
}

#[component]
pub fn CountyDashboard() -> impl IntoView {
    let params = use_params_map();
    let county = move || params.get().get("county").cloned().unwrap_or_default();
    
    let details = Resource::new(
        move || county(),
        |c| async move { get_county_details(c).await }
    );

    view! {
        <div class="explorer-container">
            <Breadcrumbs county=Some(county()) town=None outcode=None />
            <Suspense fallback=|| view! { <p>"Loading county details..."</p> }>
                {move || details.get().map(|res| match res {
                    Ok(d) => view! {
                        <h1>{format!("GBG Pubs in {}", d.name)}</h1>
                        
                        <section>
                            <h2>"Browse by Town"</h2>
                            <div class="category-grid small">
                                {d.towns.into_iter().map(|t| {
                                    let t_name = t.name.clone();
                                    let c_name = d.name.clone();
                                    view! {
                                        <A href=format!("/explore/{}/town/{}", c_name, t_name) class="category-card">
                                            <h4>{t_name}</h4>
                                            <p>{t.pub_count}</p>
                                        </A>
                                    }
                                }).collect_view()}
                            </div>
                        </section>

                        <section>
                            <h2>"Browse by Postcode"</h2>
                            <div class="category-grid small">
                                {d.outcodes.into_iter().map(|o| {
                                    let o_name = o.name.clone();
                                    let c_name = d.name.clone();
                                    view! {
                                        <A href=format!("/explore/{}/outcode/{}", c_name, o_name) class="category-card">
                                            <h4>{o_name}</h4>
                                            <p>{o.pub_count}</p>
                                        </A>
                                    }
                                }).collect_view()}
                            </div>
                        </section>
                    }.into_any(),
                    Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
                })}
            </Suspense>
        </div>
    }
}

#[component]
pub fn LocationPubList() -> impl IntoView {
    let params = use_params_map();
    let county = move || params.get().get("county").cloned().unwrap_or_default();
    let town = move || params.get().get("town").cloned();
    let outcode = move || params.get().get("outcode").cloned();

    let pubs = Resource::new(
        move || (county(), town(), outcode()),
        |(c, t, o)| async move { get_pubs_by_location(c, t, o).await }
    );

    view! {
        <div class="explorer-container">
            <Breadcrumbs county=Some(county()) town=town() outcode=outcode() />
            <h1>
                {move || if let Some(t) = town() { format!("Pubs in {}", t) } 
                         else if let Some(o) = outcode() { format!("Pubs in {}", o) }
                         else { format!("Pubs in {}", county()) }}
            </h1>

            <div class="pub-grid">
                <Suspense fallback=|| view! { <p>"Loading pubs..."</p> }>
                    {move || pubs.get().map(|res| match res {
                        Ok(list) => list.into_iter().map(|p| {
                            let id = p.id;
                            let name = p.name.clone();
                            let town = p.town.clone();
                            let county = p.county.clone();
                            let closed = p.closed;
                            view! {
                                <A href=format!("/pub/{}", id) attr:class="pub-card">
                                    <h3>{name}</h3>
                                    <p>{format!("{}, {}", town, county)}</p>
                                    {if closed {
                                        view! { <span class="badge closed">"Closed"</span> }.into_any()
                                    } else {
                                        view! { <span class="badge open">"In GBG"</span> }.into_any()
                                    }}
                                </A>
                            }
                        }).collect_view().into_any(),
                        Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
                    })}
                </Suspense>
            </div>
        </div>
    }
}
