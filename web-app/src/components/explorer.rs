use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use crate::server::{get_counties, get_county_details, get_pubs_by_location};
use crate::models::{CountyDetails, PubSummary};

#[component]
pub fn ExportButtons(
    #[prop(into)] county: Option<String>,
    #[prop(into)] town: Option<String>,
    #[prop(into)] outcode: Option<String>,
) -> impl IntoView {
    let mut query_params = Vec::new();
    if let Some(ref c) = county { query_params.push(format!("county={}", c)); }
    if let Some(ref t) = town { query_params.push(format!("town={}", t)); }
    if let Some(ref o) = outcode { query_params.push(format!("outcode={}", o)); }
    
    let query_string = if query_params.is_empty() {
        String::new()
    } else {
        format!("?{}", query_params.join("&"))
    };

    view! {
        <div class="export-container">
            <span class="export-label">"Export: "</span>
            <div class="export-group">
                <a href=format!("/export/json{}", query_string) class="export-link" download="pubs.json">"JSON"</a>
                <a href=format!("/export/csv{}", query_string) class="export-link" download="pubs.csv">"CSV"</a>
                <a href=format!("/export/parquet{}", query_string) class="export-link" download="pubs.parquet">"Parquet"</a>
            </div>
        </div>
    }
}

#[component]
pub fn Breadcrumbs(
    #[prop(into)] county: Option<String>,
    #[prop(into)] town: Option<String>,
    #[prop(into)] outcode: Option<String>,
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
            <div class="explorer-header">
                <Breadcrumbs county=None town=None outcode=None />
                <ExportButtons county=None town=None outcode=None />
            </div>
            <h1>"Browse by County"</h1>
            <Suspense fallback=|| view! { <p>"Loading counties..."</p> }>
                {move || counties.get().map(|res| match res {
                    Ok(list) => view! {
                        <div class="category-grid">
                            {list.into_iter().map(|c| {
                                let name = c.name.clone();
                                view! {
                                    <A href=format!("/explore/{}", name) attr:class="category-card">
                                        <h3>{name}</h3>
                                        <p>{c.pub_count}</p>
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
    let county = move || params.get().get("county").map(String::from).unwrap_or_default();
    
    let details = Resource::new(
        move || county(),
        |c| async move { get_county_details(c).await }
    );

    view! {
        <div class="explorer-container">
            <div class="explorer-header">
                <Breadcrumbs county=Some(county()) town=None outcode=None />
                <ExportButtons county=Some(county()) town=None outcode=None />
            </div>
            <Suspense fallback=|| view! { <p>"Loading county details..."</p> }>
                {move || details.get().map(|res: Result<CountyDetails, ServerFnError>| match res {
                    Ok(d) => {
                        let name_title = d.name.clone();
                        view! {
                            <h1>{format!("GBG Pubs in {}", name_title)}</h1>
                            
                            <section>
                                <h2>"Browse by Town"</h2>
                                <div class="category-grid small">
                                    {d.towns.into_iter().map(|t| {
                                        let t_name = t.name.clone();
                                        let c_name = d.name.clone();
                                        view! {
                                            <A href=format!("/explore/{}/town/{}", c_name, t_name) attr:class="category-card">
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
                                            <A href=format!("/explore/{}/outcode/{}", c_name, o_name) attr:class="category-card">
                                                <h4>{o_name}</h4>
                                                <p>{o.pub_count}</p>
                                            </A>
                                        }
                                    }).collect_view()}
                                </div>
                            </section>
                        }.into_any()
                    },
                    Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
                })}
            </Suspense>
        </div>
    }
}

#[component]
pub fn LocationPubList() -> impl IntoView {
    let params = use_params_map();
    let county = move || params.get().get("county").map(String::from).unwrap_or_default();
    let town = move || params.get().get("town").map(String::from);
    let outcode = move || params.get().get("outcode").map(String::from);

    let pubs = Resource::new(
        move || (county(), town(), outcode()),
        |(c, t, o)| async move { get_pubs_by_location(c, t, o).await }
    );

    view! {
        <div class="explorer-container">
            <div class="explorer-header">
                <Breadcrumbs county=Some(county()) town=town() outcode=outcode() />
                <ExportButtons county=Some(county()) town=town() outcode=outcode() />
            </div>
            <h1>
                {move || if let Some(t) = town() { format!("Pubs in {}", t) } 
                         else if let Some(o) = outcode() { format!("Pubs in {}", o) }
                         else { format!("Pubs in {}", county()) }}
            </h1>

            <div class="pub-grid">
                <Suspense fallback=|| view! { <p>"Loading pubs..."</p> }>
                    {move || pubs.get().map(|res: Result<Vec<PubSummary>, ServerFnError>| match res {
                        Ok(list) => list.into_iter().map(|p| {
                            let id = p.id;
                            let name = p.name.clone();
                            let town_p = p.town.clone();
                            let county_p = p.county.clone();
                            let closed = p.closed;
                            let year_text = p.latest_year.map(|y| format!("In GBG {}", y)).unwrap_or_else(|| "In GBG".to_string());
                            view! {
                                <A href=format!("/pub/{}", id) attr:class="pub-card">
                                    <h3>{name}</h3>
                                    <p>{format!("{}, {}", town_p, county_p)}</p>
                                    {if closed {
                                        view! { <span class="badge closed">"Closed"</span> }.into_any()
                                    } else {
                                        view! { <span class="badge open">{year_text}</span> }.into_any()
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
