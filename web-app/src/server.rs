use leptos::prelude::*;
use crate::models::{PubSummary, PubDetail, CountySummary, CountyDetails};
use uuid::Uuid;

#[server(GetCounties, "/api")]
pub async fn get_counties() -> Result<Vec<CountySummary>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let counties = sqlx::query_as!(
        CountySummary,
        r#"SELECT county as "name!", COUNT(*) as "pub_count!"
           FROM pubs 
           WHERE county IS NOT NULL AND county != ''
           GROUP BY county 
           ORDER BY county"#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(counties)
}

#[server(GetCountyDetails, "/api")]
pub async fn get_county_details(county: String) -> Result<CountyDetails, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let towns = sqlx::query_as!(
        TownSummary,
        r#"SELECT town as "name!", COUNT(*) as "pub_count!"
           FROM pubs 
           WHERE county = $1 AND town IS NOT NULL AND town != ''
           GROUP BY town 
           ORDER BY town"#,
        county
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let outcodes = sqlx::query_as!(
        OutcodeSummary,
        r#"SELECT SPLIT_PART(postcode, ' ', 1) as "name!", COUNT(*) as "pub_count!"
           FROM pubs 
           WHERE county = $1 AND postcode IS NOT NULL AND postcode != ''
           GROUP BY 1
           ORDER BY 1"#,
        county
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(CountyDetails {
        name: county,
        towns,
        outcodes,
    })
}

#[server(GetPubsByLocation, "/api")]
pub async fn get_pubs_by_location(county: String, town: Option<String>, outcode: Option<String>) -> Result<Vec<PubSummary>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let pubs = if let Some(t) = town {
        sqlx::query_as!(
            PubSummary,
            r#"SELECT p.id, p.name, 
                      COALESCE(p.town, '') as "town!", 
                      COALESCE(p.county, '') as "county!", 
                      COALESCE(p.postcode, '') as "postcode!", 
                      COALESCE(p.closed, false) as "closed!",
                      NULL::float8 as distance_meters,
                      s.latest_year
               FROM pubs p
               LEFT JOIN pub_stats s ON p.id = s.pub_id
               WHERE p.county = $1 AND p.town = $2
               ORDER BY p.name"#,
            county, t
        )
        .fetch_all(&pool)
        .await
    } else if let Some(o) = outcode {
        sqlx::query_as!(
            PubSummary,
            r#"SELECT p.id, p.name, 
                      COALESCE(p.town, '') as "town!", 
                      COALESCE(p.county, '') as "county!", 
                      COALESCE(p.postcode, '') as "postcode!", 
                      COALESCE(p.closed, false) as "closed!",
                      NULL::float8 as distance_meters,
                      s.latest_year
               FROM pubs p
               LEFT JOIN pub_stats s ON p.id = s.pub_id
               WHERE p.county = $1 AND SPLIT_PART(p.postcode, ' ', 1) = $2
               ORDER BY p.name"#,
            county, o
        )
        .fetch_all(&pool)
        .await
    } else {
        sqlx::query_as!(
            PubSummary,
            r#"SELECT p.id, p.name, 
                      COALESCE(p.town, '') as "town!", 
                      COALESCE(p.county, '') as "county!", 
                      COALESCE(p.postcode, '') as "postcode!", 
                      COALESCE(p.closed, false) as "closed!",
                      NULL::float8 as distance_meters,
                      s.latest_year
               FROM pubs p
               LEFT JOIN pub_stats s ON p.id = s.pub_id
               WHERE p.county = $1
               ORDER BY p.name"#,
            county
        )
        .fetch_all(&pool)
        .await
    };

    pubs.map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(GetPubs, "/api")]
pub async fn get_pubs(query: String) -> Result<Vec<PubSummary>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let pubs = sqlx::query_as!(
        PubSummary,
        r#"SELECT p.id, p.name, 
                  COALESCE(p.town, '') as "town!", 
                  COALESCE(p.county, '') as "county!", 
                  COALESCE(p.postcode, '') as "postcode!", 
                  COALESCE(p.closed, false) as "closed!",
                  NULL::float8 as distance_meters,
                  s.latest_year
           FROM pubs p
           LEFT JOIN pub_stats s ON p.id = s.pub_id
           WHERE p.name ILIKE $1 OR p.town ILIKE $1 OR p.county ILIKE $1
           ORDER BY p.name LIMIT 50"#,
        format!("%{}%", query)
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(pubs)
}

#[server(GeocodeManual, "/api")]
pub async fn geocode_manual(query: String) -> Result<Option<(f64, f64)>, ServerFnError> {
    // 1. Try to parse as lat, lon coordinates
    let coords: Vec<&str> = query.split(',').map(|s| s.trim()).collect();
    if coords.len() == 2 {
        if let (Ok(lat), Ok(lon)) = (coords[0].parse::<f64>(), coords[1].parse::<f64>()) {
            return Ok(Some((lat, lon)));
        }
    }

    // 2. Fallback to local Nominatim
    let nominatim_url = std::env::var("NOMINATIM_URL").unwrap_or_else(|_| "http://nominatim:8080/search".to_string());
    let client = reqwest::Client::builder()
        .user_agent("gbgdata-web (bob@example.com)")
        .build()
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let resp = client.get(nominatim_url)
        .query(&[
            ("q", query),
            ("format", "json".to_string()),
            ("limit", "1".to_string()),
        ])
        .send()
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    #[derive(serde::Deserialize)]
    struct NominatimResponse {
        lat: String,
        lon: String,
    }

    let results: Vec<NominatimResponse> = resp.json().await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    if let Some(res) = results.first() {
        let lat = res.lat.parse::<f64>().map_err(|e| ServerFnError::new(e.to_string()))?;
        let lon = res.lon.parse::<f64>().map_err(|e| ServerFnError::new(e.to_string()))?;
        Ok(Some((lat, lon)))
    } else {
        Ok(None)
    }
}

#[server(GetNearbyPubs, "/api")]
pub async fn get_nearby_pubs(lat: f64, lon: f64, radius_meters: f64) -> Result<Vec<PubSummary>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let pubs = sqlx::query_as!(
        PubSummary,
        r#"SELECT p.id, p.name, 
                  COALESCE(p.town, '') as "town!", 
                  COALESCE(p.county, '') as "county!", 
                  COALESCE(p.postcode, '') as "postcode!", 
                  COALESCE(p.closed, false) as "closed!",
                  ST_Distance(p.location, ST_SetSRID(ST_MakePoint($2, $1), 4326)::geography) as "distance_meters",
                  s.latest_year
           FROM pubs p
           LEFT JOIN pub_stats s ON p.id = s.pub_id
           WHERE p.location IS NOT NULL 
             AND ST_DWithin(p.location, ST_SetSRID(ST_MakePoint($2, $1), 4326)::geography, $3)
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
