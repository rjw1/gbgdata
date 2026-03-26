use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use crate::server::{get_counties, get_county_details, get_pubs_by_location, get_years, get_year_counties};

#[component]
pub fn ExportButtons(
    #[prop(into)] county: Option<String>,
    #[prop(into)] town: Option<String>,
    #[prop(into)] outcode: Option<String>,
    #[prop(into)] year: Option<i32>,
) -> impl IntoView {
    let mut query_params = Vec::new();
    if let Some(ref c) = county { query_params.push(format!("county={}", c)); }
    if let Some(ref t) = town { query_params.push(format!("town={}", t)); }
    if let Some(ref o) = outcode { query_params.push(format!("outcode={}", o)); }
    if let Some(y) = year { query_params.push(format!("year={}", y)); }
    
    let query_string = if query_params.is_empty() {
        String::new()
    } else {
        format!("?{}", query_params.join("&"))
    };

    let mut name_parts = vec!["gbg-pubs".to_string()];
    if let Some(y) = year { name_parts.push(y.to_string()); }
    if let Some(ref c) = county { name_parts.push(c.replace(" ", "_")); }
    if let Some(ref t) = town { name_parts.push(t.replace(" ", "_")); }
    if let Some(ref o) = outcode { name_parts.push(o.replace(" ", "_")); }
    let base_name = name_parts.join("-");

    view! {
        <div class="export-container">
            <span class="export-label">"Export: "</span>
            <div class="export-group">
                <a href=format!("/export/json{}", query_string) class="export-link" rel="external" download=format!("{}.json", base_name)>"JSON"</a>
                <a href=format!("/export/csv{}", query_string) class="export-link" rel="external" download=format!("{}.csv", base_name)>"CSV"</a>
                <a href=format!("/export/parquet{}", query_string) class="export-link" rel="external" download=format!("{}.parquet", base_name)>"Parquet"</a>
            </div>
        </div>
    }
}

#[component]
pub fn Breadcrumbs(
    #[prop(into)] county: Option<String>,
    #[prop(into)] town: Option<String>,
    #[prop(into)] outcode: Option<String>,
    #[prop(into)] year: Option<i32>,
) -> impl IntoView {
    view! {
        <nav class="breadcrumbs">
            <A href="/">"Home"</A>
            " > "
            <A href="/explore">"Explore"</A>
            {move || year.map(|y| view! { 
                " > " <A href=format!("/explore/year/{}", y)>{y}</A> 
            })}
            {move || county.clone().map(|c| {
                let url = if let Some(y) = year { format!("/explore/year/{}/{}", y, c) } else { format!("/explore/{}", c) };
                view! { " > " <A href=url>{c}</A> }
            })}
            {move || town.clone().map(|t| view! { " > " {t} })}
            {move || outcode.clone().map(|o| view! { " > " {o} })}
        </nav>
    }
}

#[component]
pub fn ExplorerHome() -> impl IntoView {
    let counties = Resource::new(|| (), |_| async move { get_counties().await });
    let years = Resource::new(|| (), |_| async move { get_years().await });

    view! {
        <div class="explorer-container">
            <div class="explorer-header">
                <Breadcrumbs county=None town=None outcode=None year=None />
                <ExportButtons county=None town=None outcode=None year=None />
            </div>
            
            <section>
                <h1>"Browse by Year"</h1>
                <Suspense fallback=|| view! { <p>"Loading years..."</p> }>
                    {move || years.get().map(|res| match res {
                        Ok(list) => view! {
                            <div class="category-grid small">
                                {list.into_iter().map(|y| {
                                    let year = y.year;
                                    view! {
                                        <A href=format!("/explore/year/{}", year) attr:class="category-card">
                                            <h3>{year}</h3>
                                            <p>{y.pub_count}</p>
                                        </A>
                                    }
                                }).collect_view()}
                            </div>
                        }.into_any(),
                        Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
                    })}
                </Suspense>
            </section>

            <section>
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
            </section>
        </div>
    }
}

#[component]
pub fn YearDashboard() -> impl IntoView {
    let params = use_params_map();
    let year = move || params.get().get("year").and_then(|y| y.parse::<i32>().ok()).unwrap_or(2026);
    
    let counties = Resource::new(
        move || year(),
        |y| async move { get_year_counties(y).await }
    );

    view! {
        <div class="explorer-container">
            <div class="explorer-header">
                <Breadcrumbs county=None town=None outcode=None year=Some(year()) />
                <ExportButtons county=None town=None outcode=None year=Some(year()) />
            </div>
            <h1>{move || format!("GBG {} Counties", year())}</h1>
            <Suspense fallback=|| view! { <p>"Loading counties..."</p> }>
                {move || counties.get().map(|res| match res {
                    Ok(list) => view! {
                        <div class="category-grid">
                            {list.into_iter().map(|c| {
                                let name = c.name.clone();
                                let y = year();
                                view! {
                                    <A href=format!("/explore/year/{}/{}", y, name) attr:class="category-card">
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
    let year = move || params.get().get("year").and_then(|y| y.parse::<i32>().ok());
    
    let details = Resource::new(
        move || (county(), year()),
        |(c, y)| async move { get_county_details(c, y).await }
    );

    view! {
        <div class="explorer-container">
            <div class="explorer-header">
                <Breadcrumbs county=Some(county()) town=None outcode=None year=year() />
                <ExportButtons county=Some(county()) town=None outcode=None year=year() />
            </div>
            <Suspense fallback=|| view! { <p>"Loading county details..."</p> }>
                {move || details.get().map(|res| match res {
                    Ok(d) => {
                        let name_title = d.name.clone();
                        let y_opt = year();
                        view! {
                            <h1>
                                {if let Some(y) = y_opt { format!("GBG {} Pubs in {}", y, name_title) }
                                 else { format!("GBG Pubs in {}", name_title) }}
                            </h1>
                            
                            <section>
                                <h2>"Browse by Town"</h2>
                                <div class="category-grid small">
                                    {d.towns.into_iter().map(|t| {
                                        let t_name = t.name.clone();
                                        let c_name = d.name.clone();
                                        let url = if let Some(y) = y_opt { format!("/explore/year/{}/{}/town/{}", y, c_name, t_name) }
                                                 else { format!("/explore/{}/town/{}", c_name, t_name) };
                                        view! {
                                            <A href=url attr:class="category-card">
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
                                        let url = if let Some(y) = y_opt { format!("/explore/year/{}/{}/outcode/{}", y, c_name, o_name) }
                                                 else { format!("/explore/{}/outcode/{}", c_name, o_name) };
                                        view! {
                                            <A href=url attr:class="category-card">
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
    let year = move || params.get().get("year").and_then(|y| y.parse::<i32>().ok());

    let pubs = Resource::new(
        move || (county(), town(), outcode(), year()),
        |(c, t, o, y)| async move { get_pubs_by_location(c, t, o, y).await }
    );

    view! {
        <div class="explorer-container">
            <div class="explorer-header">
                <Breadcrumbs county=Some(county()) town=town() outcode=outcode() year=year() />
                <ExportButtons county=Some(county()) town=town() outcode=outcode() year=year() />
            </div>
            <h1>
                {move || {
                    let loc = if let Some(t) = town() { format!("in {}", t) } 
                             else if let Some(o) = outcode() { format!("in {}", o) }
                             else { format!("in {}", county()) };
                    
                    if let Some(y) = year() {
                        format!("GBG {} Pubs {}", y, loc)
                    } else {
                        format!("GBG Pubs {}", loc)
                    }
                }}
            </h1>

            <div class="pub-grid">
                <Suspense fallback=|| view! { <p>"Loading pubs..."</p> }>
                    {move || pubs.get().map(|res| match res {
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
