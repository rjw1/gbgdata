use leptos::prelude::*;
use crate::models::{PubSummary, PubDetail, RegionSummary, RegionDetails, TownSummary, OutcodeSummary, YearSummary, SortMode};
use uuid::Uuid;

#[cfg(feature = "ssr")]
fn get_order_by(sort: Option<SortMode>, default: &str) -> String {
    match sort.unwrap_or_default() {
        SortMode::Name => "ORDER BY p.name ASC".to_string(),
        SortMode::Streak => "ORDER BY COALESCE(s.current_streak, 0) DESC, p.name ASC".to_string(),
        SortMode::TotalEntries => "ORDER BY COALESCE(s.total_years, 0) DESC, p.name ASC".to_string(),
        SortMode::Distance => format!("ORDER BY {} ASC, p.name ASC", default),
    }
}

#[server(GetYears, "/api")]
pub async fn get_years() -> Result<Vec<YearSummary>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let years = sqlx::query_as::<_, YearSummary>(
        r#"SELECT year, COUNT(*) as "pub_count"
           FROM gbg_history 
           GROUP BY year 
           ORDER BY year DESC"#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(years)
}

#[server(GetYearRegions, "/api")]
pub async fn get_year_regions(year: i32) -> Result<Vec<RegionSummary>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let regions = sqlx::query_as::<_, RegionSummary>(
        r#"SELECT p.region as "name", COUNT(*) as "pub_count"
           FROM pubs p
           JOIN gbg_history h ON p.id = h.pub_id
           WHERE h.year = $1 AND p.region IS NOT NULL AND p.region != ''
           GROUP BY p.region 
           ORDER BY p.region"#
    )
    .bind(year)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(regions)
}

#[server(GetRegions, "/api")]
pub async fn get_regions() -> Result<Vec<RegionSummary>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let regions = sqlx::query_as::<_, RegionSummary>(
        r#"SELECT region as "name", COUNT(*) as "pub_count"
           FROM pubs 
           WHERE region IS NOT NULL AND region != ''
           GROUP BY region 
           ORDER BY region"#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(regions)
}

#[server(GetRegionDetails, "/api")]
pub async fn get_region_details(region: String, year: Option<i32>) -> Result<RegionDetails, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let towns_query = if year.is_some() {
        r#"SELECT town as "name", COUNT(*) as "pub_count"
           FROM pubs p
           JOIN gbg_history h ON p.id = h.pub_id
           WHERE p.region = $1 AND h.year = $2 AND town IS NOT NULL AND town != ''
           GROUP BY town 
           ORDER BY town"#
    } else {
        r#"SELECT town as "name", COUNT(*) as "pub_count"
           FROM pubs 
           WHERE region = $1 AND town IS NOT NULL AND town != ''
           GROUP BY town 
           ORDER BY town"#
    };

    let mut towns_q = sqlx::query_as::<_, TownSummary>(towns_query).bind(&region);
    if let Some(y) = year { towns_q = towns_q.bind(y); }
    let towns = towns_q.fetch_all(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let outcodes_query = if year.is_some() {
        r#"SELECT SPLIT_PART(postcode, ' ', 1) as "name", COUNT(*) as "pub_count"
           FROM pubs p
           JOIN gbg_history h ON p.id = h.pub_id
           WHERE p.region = $1 AND h.year = $2 AND postcode IS NOT NULL AND postcode != ''
           GROUP BY 1
           ORDER BY 1"#
    } else {
        r#"SELECT SPLIT_PART(postcode, ' ', 1) as "name", COUNT(*) as "pub_count"
           FROM pubs 
           WHERE region = $1 AND postcode IS NOT NULL AND postcode != ''
           GROUP BY 1
           ORDER BY 1"#
    };

    let mut outcodes_q = sqlx::query_as::<_, OutcodeSummary>(outcodes_query).bind(&region);
    if let Some(y) = year { outcodes_q = outcodes_q.bind(y); }
    let outcodes = outcodes_q.fetch_all(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(RegionDetails {
        name: region,
        towns,
        outcodes,
    })
}

#[server(GetPubsByLocation, "/api")]
pub async fn get_pubs_by_location(region: String, town: Option<String>, outcode: Option<String>, year: Option<i32>, sort: Option<SortMode>) -> Result<Vec<PubSummary>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let mut query = String::from(
        r#"SELECT p.id, p.name, 
                  COALESCE(p.town, '') as town, 
                  COALESCE(p.region, '') as region, 
                  p.country_code,
                  COALESCE(p.postcode, '') as postcode, 
                  COALESCE(p.closed, false) as closed,
                  NULL::float8 as distance_meters,
                  s.latest_year,
                  s.total_years as total_years_rank,
                  s.current_streak
           FROM pubs p
           LEFT JOIN pub_stats s ON p.id = s.pub_id"#
    );

    if year.is_some() {
        query.push_str(" JOIN gbg_history h ON p.id = h.pub_id");
    }

    query.push_str(" WHERE p.region = $1");

    let mut binds: Vec<String> = vec![region];
    let mut param_idx = 2;

    if let Some(t) = town {
        query.push_str(&format!(" AND p.town = ${}", param_idx));
        binds.push(t);
        param_idx += 1;
    } else if let Some(o) = outcode {
        query.push_str(&format!(" AND SPLIT_PART(p.postcode, ' ', 1) = ${}", param_idx));
        binds.push(o);
        param_idx += 1;
    }

    if let Some(_y) = year {
        query.push_str(&format!(" AND h.year = ${}", param_idx));
    }

    query.push_str(&format!(" {}", get_order_by(sort, "p.name")));

    // Handle types properly - since year is i32, we need a custom query builder or fixed variants
    let pubs = if let Some(y) = year {
        if binds.len() == 2 {
            sqlx::query_as::<_, PubSummary>(&query).bind(&binds[0]).bind(&binds[1]).bind(y).fetch_all(&pool).await
        } else {
            sqlx::query_as::<_, PubSummary>(&query).bind(&binds[0]).bind(y).fetch_all(&pool).await
        }
    } else {
        if binds.len() == 2 {
            sqlx::query_as::<_, PubSummary>(&query).bind(&binds[0]).bind(&binds[1]).fetch_all(&pool).await
        } else {
            sqlx::query_as::<_, PubSummary>(&query).bind(&binds[0]).fetch_all(&pool).await
        }
    }.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(pubs)
}

#[server(GetPubs, "/api")]
pub async fn get_pubs(query: String, sort: Option<SortMode>) -> Result<Vec<PubSummary>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let pubs = sqlx::query_as::<_, PubSummary>(
        &format!(
            r#"SELECT p.id, p.name, 
                  COALESCE(p.town, '') as town, 
                  COALESCE(p.region, '') as region, 
                  p.country_code,
                  COALESCE(p.postcode, '') as postcode, 
                  COALESCE(p.closed, false) as closed,
                  NULL::float8 as distance_meters,
                  s.latest_year,
                  s.total_years as total_years_rank,
                  s.current_streak
           FROM pubs p
           LEFT JOIN pub_stats s ON p.id = s.pub_id
           WHERE p.name ILIKE $1 OR p.town ILIKE $1 OR p.region ILIKE $1
           {} LIMIT 50"#,
            get_order_by(sort, "p.name")
        )
    )
    .bind(format!("%{}%", query))
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(pubs)
}

#[server(GetRankedPubs, "/api")]
pub async fn get_ranked_pubs(sort: Option<SortMode>) -> Result<Vec<PubSummary>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let pubs = sqlx::query_as::<_, PubSummary>(
        &format!(
            r#"SELECT p.id, p.name, 
                  COALESCE(p.town, '') as town, 
                  COALESCE(p.region, '') as region, 
                  p.country_code,
                  COALESCE(p.postcode, '') as postcode, 
                  COALESCE(p.closed, false) as closed,
                  NULL::float8 as distance_meters,
                  s.latest_year,
                  s.total_years as total_years_rank,
                  s.current_streak
           FROM pubs p
           JOIN pub_stats s ON p.id = s.pub_id
           {} LIMIT 100"#,
            get_order_by(sort, "s.total_years DESC")
        )
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
pub async fn get_nearby_pubs(lat: f64, lon: f64, radius_meters: f64, sort: Option<SortMode>) -> Result<Vec<PubSummary>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let pubs = sqlx::query_as::<_, PubSummary>(
        &format!(
            r#"SELECT p.id, p.name, 
                  COALESCE(p.town, '') as town, 
                  COALESCE(p.region, '') as region, 
                  p.country_code,
                  COALESCE(p.postcode, '') as postcode, 
                  COALESCE(p.closed, false) as closed,
                  ST_Distance(p.location, ST_SetSRID(ST_MakePoint($2, $1), 4326)::geography) as distance_meters,
                  s.latest_year,
                  s.total_years as total_years_rank,
                  s.current_streak
           FROM pubs p
           LEFT JOIN pub_stats s ON p.id = s.pub_id
           WHERE p.location IS NOT NULL 
             AND ST_DWithin(p.location, ST_SetSRID(ST_MakePoint($2, $1), 4326)::geography, $3)
           {} LIMIT 50"#,
            get_order_by(sort, "distance_meters")
        )
    )
    .bind(lat)
    .bind(lon)
    .bind(radius_meters)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(pubs)
}

#[server(GetPubDetail, "/api")]
pub async fn get_pub_detail(id: Uuid) -> Result<PubDetail, ServerFnError> {
    use sqlx::{PgPool, Row};
    use leptos::context::use_context;
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let row = sqlx::query(
        r#"SELECT p.id, p.name, 
                  COALESCE(p.address, '') as address, 
                  COALESCE(p.town, '') as town, 
                  COALESCE(p.region, '') as region, 
                  p.country_code,
                  COALESCE(p.postcode, '') as postcode, 
                  COALESCE(p.closed, false) as closed,
                  p.untappd_id, p.google_maps_id, p.whatpub_id, p.rgl_id,
                  ST_Y(p.location::geometry) as lat,
                  ST_X(p.location::geometry) as lon,
                  COALESCE(s.current_streak, 0) as current_streak,
                  COALESCE(s.last_5_years, 0) as last_5_years,
                  COALESCE(s.last_10_years, 0) as last_10_years,
                  COALESCE(s.total_years, 0) as total_years,
                  s.first_year,
                  s.latest_year
           FROM pubs p
           LEFT JOIN pub_stats s ON p.id = s.pub_id
           WHERE p.id = $1"#
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let years = sqlx::query_scalar::<_, i32>(
        "SELECT year FROM gbg_history WHERE pub_id = $1 ORDER BY year DESC"
    )
    .bind(id)
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(PubDetail {
        id: row.get("id"),
        name: row.get("name"),
        address: row.get("address"),
        town: row.get("town"),
        region: row.get("region"),
        country_code: row.get("country_code"),
        postcode: row.get("postcode"),
        closed: row.get("closed"),
        untappd_id: row.get("untappd_id"),
        google_maps_id: row.get("google_maps_id"),
        whatpub_id: row.get("whatpub_id"),
        rgl_id: row.get("rgl_id"),
        lat: row.get("lat"),
        lon: row.get("lon"),
        current_streak: row.get("current_streak"),
        last_5_years: row.get("last_5_years"),
        last_10_years: row.get("last_10_years"),
        total_years: row.get("total_years"),
        first_year: row.get("first_year"),
        latest_year: row.get("latest_year"),
        years,
    })
}
