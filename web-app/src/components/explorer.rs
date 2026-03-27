use leptos::prelude::*;
use leptos_router::components::A;
use leptos_router::hooks::use_params_map;
use crate::server::{get_regions, get_region_details, get_pubs_by_location, get_years, get_year_regions};
use crate::models::{SortMode};
use crate::components::sort::SortSelector;
use crate::components::map::MapView;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
pub enum ViewMode {
    #[default]
    List,
    Map,
}

#[component]
pub fn ExportButtons(
    #[prop(into)] region: Option<String>,
    #[prop(into)] town: Option<String>,
    #[prop(into)] outcode: Option<String>,
    #[prop(into)] year: Option<i32>,
) -> impl IntoView {
    let mut query_params = Vec::new();
    if let Some(ref r) = region { query_params.push(format!("region={}", r)); }
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
    if let Some(ref r) = region { name_parts.push(r.replace(" ", "_")); }
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
    #[prop(into)] region: Option<String>,
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
            {move || region.clone().map(|r| {
                let url = if let Some(y) = year { format!("/explore/year/{}/{}", y, r) } else { format!("/explore/{}", r) };
                view! { " > " <A href=url>{r}</A> }
            })}
            {move || town.clone().map(|t| view! { " > " {t} })}
            {move || outcode.clone().map(|o| view! { " > " {o} })}
        </nav>
    }
}

#[component]
pub fn ExplorerHome() -> impl IntoView {
    let regions = Resource::new(|| (), |_| async move { get_regions().await });
    let years = Resource::new(|| (), |_| async move { get_years().await });

    view! {
        <div class="explorer-container">
            <div class="explorer-header">
                <Breadcrumbs region=None town=None outcode=None year=None />
                <ExportButtons region=None town=None outcode=None year=None />
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
                <h1>"Browse by Region"</h1>
                <Suspense fallback=|| view! { <p>"Loading regions..."</p> }>
                    {move || regions.get().map(|res| match res {
                        Ok(list) => view! {
                            <div class="category-grid">
                                {list.into_iter().map(|r| {
                                    let name = r.name.clone();
                                    view! {
                                        <A href=format!("/explore/{}", name) attr:class="category-card">
                                            <h3>{name}</h3>
                                            <p>{r.pub_count}</p>
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
    
    let regions = Resource::new(
        move || year(),
        |y| async move { get_year_regions(y).await }
    );

    view! {
        <div class="explorer-container">
            <div class="explorer-header">
                <Breadcrumbs region=None town=None outcode=None year=Some(year()) />
                <ExportButtons region=None town=None outcode=None year=Some(year()) />
            </div>
            <h1>{move || format!("GBG {} Regions", year())}</h1>
            <Suspense fallback=|| view! { <p>"Loading regions..."</p> }>
                {move || regions.get().map(|res| match res {
                    Ok(list) => view! {
                        <div class="category-grid">
                            {list.into_iter().map(|r| {
                                let name = r.name.clone();
                                let y = year();
                                view! {
                                    <A href=format!("/explore/year/{}/{}", y, name) attr:class="category-card">
                                        <h3>{name}</h3>
                                        <p>{r.pub_count}</p>
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
pub fn RegionDashboard() -> impl IntoView {
    let params = use_params_map();
    let region = move || params.get().get("region").map(String::from).unwrap_or_default();
    let year = move || params.get().get("year").and_then(|y| y.parse::<i32>().ok());
    
    let details = Resource::new(
        move || (region(), year()),
        |(r, y)| async move { get_region_details(r, y).await }
    );

    view! {
        <div class="explorer-container">
            <div class="explorer-header">
                <Breadcrumbs region=Some(region()) town=None outcode=None year=year() />
                <ExportButtons region=Some(region()) town=None outcode=None year=year() />
            </div>
            <Suspense fallback=|| view! { <p>"Loading region details..."</p> }>
                {move || details.get().map(|res| match res {
                    Ok(d) => {
                        let name_title = d.name.clone();
                        let y_opt = year();
                        view! {
                            <h1>
                                {if let Some(y) = y_opt { format!("GBG {} Pubs in {}", y, name_title) }
                                 else { format!("GBG Pubs in {}", name_title) }}
                            </h1>

                            <div class="direct-actions">
                                {
                                    let url = if let Some(y) = y_opt { format!("/explore/year/{}/{}/all", y, d.name) }
                                             else { format!("/explore/{}/all", d.name) };
                                    view! {
                                        <A href=url attr:class="category-card highlight">
                                            <h3>"View All Pubs"</h3>
                                            <p>"See every pub in this region as a single list"</p>
                                        </A>
                                    }
                                }
                            </div>
                            
                            <section>
                                <h2>"Browse by Town"</h2>
                                <div class="category-grid small">
                                    {d.towns.into_iter().map(|t| {
                                        let t_name = t.name.clone();
                                        let r_name = d.name.clone();
                                        let url = if let Some(y) = y_opt { format!("/explore/year/{}/{}/town/{}", y, r_name, t_name) }
                                                 else { format!("/explore/{}/town/{}", r_name, t_name) };
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
                                        let r_name = d.name.clone();
                                        let url = if let Some(y) = y_opt { format!("/explore/year/{}/{}/outcode/{}", y, r_name, o_name) }
                                                 else { format!("/explore/{}/outcode/{}", r_name, o_name) };
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
    let region = move || params.get().get("region").map(String::from).unwrap_or_default();
    let town = move || params.get().get("town").map(String::from);
    let outcode = move || params.get().get("outcode").map(String::from);
    let year = move || params.get().get("year").and_then(|y| y.parse::<i32>().ok());
    let (sort, set_sort) = signal(SortMode::default());
    let (view_mode, set_view_mode) = signal(ViewMode::default());
    let (open_only, set_open_only) = signal(false);

    let pubs = Resource::new(
        move || (region(), town(), outcode(), year(), sort.get(), open_only.get()),
        |(r, t, o, y, s, open)| async move { get_pubs_by_location(r, t, o, y, Some(s), Some(open)).await }
    );

    view! {
        <div class="explorer-container">
            <div class="explorer-header">
                <Breadcrumbs region=Some(region()) town=town() outcode=outcode() year=year() />
                <div class="header-controls">
                    <div class="view-toggle">
                        <button 
                            class=move || format!("toggle-btn {}", if view_mode.get() == ViewMode::List { "active" } else { "" })
                            on:click=move |_| set_view_mode.set(ViewMode::List)
                        >
                            "📋 List"
                        </button>
                        <button 
                            class=move || format!("toggle-btn {}", if view_mode.get() == ViewMode::Map { "active" } else { "" })
                            on:click=move |_| set_view_mode.set(ViewMode::Map)
                        >
                            "🗺️ Map"
                        </button>
                    </div>
                    <label class="open-only-toggle">
                        <input 
                            type="checkbox" 
                            on:change=move |ev| set_open_only.set(event_target_checked(&ev))
                            prop:checked=open_only
                        />
                        " Open only"
                    </label>
                    <SortSelector 
                        sort=Signal::from(sort) 
                        on_change=Callback::new(move |mode| set_sort.set(mode)) 
                    />
                    <ExportButtons region=Some(region()) town=town() outcode=outcode() year=year() />
                </div>
            </div>
            <h1>
                {move || {
                    let loc = if let Some(t) = town() { format!("in {}", t) } 
                             else if let Some(o) = outcode() { format!("in {}", o) }
                             else { format!("in {}", region()) };
                    
                    if let Some(y) = year() {
                        format!("GBG {} Pubs {}", y, loc)
                    } else {
                        format!("GBG Pubs {}", loc)
                    }
                }}
            </h1>

            <Suspense fallback=|| view! { <p>"Loading pubs..."</p> }>
                {move || pubs.get().map(|res| match res {
                    Ok(list) => {
                        if view_mode.get() == ViewMode::Map {
                            view! {
                                <MapView pubs=Signal::from(list) />
                            }.into_any()
                        } else {
                            view! {
                                <div class="pub-grid">
                                    {list.into_iter().map(|p| {
                                        let id = p.id;
                                        let name = p.name.clone();
                                        let town_p = p.town.clone();
                                        let region_p = p.region.clone();
                                        let closed = p.closed;
                                        let total = p.total_years_rank.unwrap_or(0);
                                        let streak = p.current_streak.unwrap_or(0);
                                        let year_text = p.latest_year.map(|y| format!("In GBG {}", y)).unwrap_or_else(|| "In GBG".to_string());
                                        
                                        view! {
                                            <A href=format!("/pub/{}", id) attr:class="pub-card">
                                                <h3>{name}</h3>
                                                <p>{format!("{}, {}", town_p, region_p)}</p>
                                                
                                                <div class="card-stats">
                                                    <div class=format!("stat-badge {}", if sort.get() == SortMode::TotalEntries { "highlight" } else { "" })>
                                                        <span class="count">{total}</span>
                                                        <span class="label">" entries"</span>
                                                    </div>
                                                    <div class=format!("stat-badge {}", if sort.get() == SortMode::Streak { "highlight" } else { "" })>
                                                        <span class="count">{streak}</span>
                                                        <span class="label">" streak"</span>
                                                    </div>
                                                </div>

                                                {if closed {
                                                    view! { <span class="badge closed">"Closed"</span> }.into_any()
                                                } else {
                                                    view! { <span class="badge open">{year_text}</span> }.into_any()
                                                }}
                                            </A>
                                        }
                                    }).collect_view()}
                                </div>
                            }.into_any()
                        }
                    },
                    Err(e) => view! { <p class="error">{e.to_string()}</p> }.into_any(),
                })}
            </Suspense>
        </div>
    }
}
