use leptos::prelude::*;
use crate::models::{PubSummary, PubDetail};
use uuid::Uuid;

#[server(GetPubs, "/api")]
pub async fn get_pubs(query: String) -> Result<Vec<PubSummary>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let pubs = sqlx::query_as!(
        PubSummary,
        r#"SELECT id, name, 
                  COALESCE(town, '') as "town!", 
                  COALESCE(county, '') as "county!", 
                  COALESCE(postcode, '') as "postcode!", 
                  COALESCE(closed, false) as "closed!",
                  NULL::float8 as distance_meters
           FROM pubs 
           WHERE name ILIKE $1 OR town ILIKE $1 OR county ILIKE $1
           ORDER BY name LIMIT 50"#,
        format!("%{}%", query)
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(pubs)
}

#[server(GetNearbyPubs, "/api")]
pub async fn get_nearby_pubs(lat: f64, lon: f64, radius_meters: f64) -> Result<Vec<PubSummary>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let pubs = sqlx::query_as!(
        PubSummary,
        r#"SELECT id, name, 
                  COALESCE(town, '') as "town!", 
                  COALESCE(county, '') as "county!", 
                  COALESCE(postcode, '') as "postcode!", 
                  COALESCE(closed, false) as "closed!",
                  ST_Distance(location, ST_SetSRID(ST_MakePoint($2, $1), 4326)::geography) as "distance_meters"
           FROM pubs 
           WHERE location IS NOT NULL 
             AND ST_DWithin(location, ST_SetSRID(ST_MakePoint($2, $1), 4326)::geography, $3)
           ORDER BY distance_meters LIMIT 50"#,
        lat, lon, radius_meters
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(pubs)
}

#[server(GetPubDetail, "/api")]
pub async fn get_pub_detail(id: Uuid) -> Result<PubDetail, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let pub_info = sqlx::query!(
        r#"SELECT p.id, p.name, 
                  COALESCE(p.address, '') as "address!", 
                  COALESCE(p.town, '') as "town!", 
                  COALESCE(p.county, '') as "county!", 
                  COALESCE(p.postcode, '') as "postcode!", 
                  COALESCE(p.closed, false) as "closed!",
                  p.untappd_id, p.google_maps_id, p.whatpub_id, p.rgl_id,
                  ST_Y(p.location::geometry) as lat,
                  ST_X(p.location::geometry) as lon,
                  COALESCE(s.current_streak, 0) as "current_streak!",
                  COALESCE(s.last_5_years, 0) as "last_5_years!",
                  COALESCE(s.last_10_years, 0) as "last_10_years!",
                  COALESCE(s.total_years, 0) as "total_years!",
                  s.first_year,
                  s.latest_year
           FROM pubs p
           LEFT JOIN pub_stats s ON p.id = s.pub_id
           WHERE p.id = $1"#,
        id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let years = sqlx::query_scalar!(
        "SELECT year FROM gbg_history WHERE pub_id = $1 ORDER BY year DESC",
        id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(PubDetail {
        id: pub_info.id,
        name: pub_info.name,
        address: pub_info.address,
        town: pub_info.town,
        county: pub_info.county,
        postcode: pub_info.postcode,
        closed: pub_info.closed,
        untappd_id: pub_info.untappd_id,
        google_maps_id: pub_info.google_maps_id,
        whatpub_id: pub_info.whatpub_id,
        rgl_id: pub_info.rgl_id,
        lat: pub_info.lat,
        lon: pub_info.lon,
        current_streak: pub_info.current_streak,
        last_5_years: pub_info.last_5_years,
        last_10_years: pub_info.last_10_years,
        total_years: pub_info.total_years,
        first_year: pub_info.first_year,
        latest_year: pub_info.latest_year,
        years,
    })
}
