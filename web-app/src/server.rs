use leptos::prelude::*;
use crate::models::{PubSummary, PubDetail, RegionSummary, RegionDetails, YearSummary, SortMode, UserAuthStatus};
#[cfg(feature = "ssr")]
use crate::models::{TownSummary, OutcodeSummary, UserInvite};
use crate::auth::User;
use uuid::Uuid;

#[server(Login, "/api")]
pub async fn login(username: String, password: String) -> Result<Option<Uuid>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use crate::auth::verify_password;
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let user = sqlx::query!(
        "SELECT id, password_hash FROM users WHERE username = $1",
        username
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    if let Some(user) = user {
        if verify_password(&password, &user.password_hash) {
            return Ok(Some(user.id));
        }
    }

    Ok(None)
}

#[server(Verify2FA, "/api")]
pub async fn verify_2fa(user_id: Uuid, code: String) -> Result<bool, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use crate::auth::{verify_totp, verify_recovery_code, User, session};
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let session = extract::<Session>().await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let user_data = sqlx::query!(
        "SELECT id, username, role, totp_setup_completed, totp_secret_enc, recovery_codes_hash FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    let success = if code.len() == 6 && code.chars().all(|c| c.is_digit(10)) {
        verify_totp(&user_data.username, &user_data.totp_secret_enc, &code)
    } else {
        // Check recovery codes
        if verify_recovery_code(&code, &user_data.recovery_codes_hash) {
            // Remove used recovery code
            let new_codes: Vec<String> = user_data.recovery_codes_hash
                .into_iter()
                .filter(|h| !crate::auth::verify_password(&code, h))
                .collect();
            
            sqlx::query!(
                "UPDATE users SET recovery_codes_hash = $1 WHERE id = $2",
                &new_codes,
                user_id
            )
            .execute(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
            
            true
        } else {
            false
        }
    };

    if success {
        session::login(&session, &User {
            id: user_data.id,
            username: user_data.username,
            role: user_data.role,
            totp_setup_completed: user_data.totp_setup_completed,
        }).await.map_err(|e| ServerFnError::new(e.to_string()))?;
    }

    Ok(success)
}

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
pub async fn get_pubs_by_location(region: String, town: Option<String>, outcode: Option<String>, year: Option<i32>, sort: Option<SortMode>, open_only: Option<bool>) -> Result<Vec<PubSummary>, ServerFnError> {
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
                  ST_Y(p.location::geometry) as lat,
                  ST_X(p.location::geometry) as lon,
                  s.latest_year,
                  s.total_years as total_years_rank,
                  s.current_streak,
                  p.whatpub_id,
                  p.google_maps_id,
                  p.untappd_id
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

    if open_only.unwrap_or(false) {
        query.push_str(" AND p.closed = false");
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
pub async fn get_pubs(query: String, sort: Option<SortMode>, open_only: Option<bool>) -> Result<Vec<PubSummary>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let open_filter = if open_only.unwrap_or(false) { "AND p.closed = false" } else { "" };

    let pubs = sqlx::query_as::<_, PubSummary>(
        &format!(
            r#"SELECT p.id, p.name, 
                  COALESCE(p.town, '') as town, 
                  COALESCE(p.region, '') as region, 
                  p.country_code,
                  COALESCE(p.postcode, '') as postcode, 
                  COALESCE(p.closed, false) as closed,
                  NULL::float8 as distance_meters,
                  ST_Y(p.location::geometry) as lat,
                  ST_X(p.location::geometry) as lon,
                  s.latest_year,
                  s.total_years as total_years_rank,
                  s.current_streak,
                  p.whatpub_id,
                  p.google_maps_id,
                  p.untappd_id
           FROM pubs p
           LEFT JOIN pub_stats s ON p.id = s.pub_id
           WHERE (p.name ILIKE $1 OR p.town ILIKE $1 OR p.region ILIKE $1)
           {}
           {} LIMIT 50"#,
            open_filter,
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
pub async fn get_ranked_pubs(sort: Option<SortMode>, open_only: Option<bool>) -> Result<Vec<PubSummary>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let open_filter = if open_only.unwrap_or(false) { "WHERE p.closed = false" } else { "" };

    let pubs = sqlx::query_as::<_, PubSummary>(
        &format!(
            r#"SELECT p.id, p.name, 
                  COALESCE(p.town, '') as town, 
                  COALESCE(p.region, '') as region, 
                  p.country_code,
                  COALESCE(p.postcode, '') as postcode, 
                  COALESCE(p.closed, false) as closed,
                  NULL::float8 as distance_meters,
                  ST_Y(p.location::geometry) as lat,
                  ST_X(p.location::geometry) as lon,
                  s.latest_year,
                  s.total_years as total_years_rank,
                  s.current_streak,
                  p.whatpub_id,
                  p.google_maps_id,
                  p.untappd_id
           FROM pubs p
           JOIN pub_stats s ON p.id = s.pub_id
           {}
           {} LIMIT 100"#,
            open_filter,
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
    let nominatim_url = std::env::var("NOMINATIM_URL").unwrap_or_default();
    if nominatim_url.is_empty() {
        return Ok(None);
    }

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
pub async fn get_nearby_pubs(lat: f64, lon: f64, radius_meters: f64, sort: Option<SortMode>, open_only: Option<bool>) -> Result<Vec<PubSummary>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let open_filter = if open_only.unwrap_or(false) { "AND p.closed = false" } else { "" };

    let pubs = sqlx::query_as::<_, PubSummary>(
        &format!(
            r#"SELECT p.id, p.name, 
                  COALESCE(p.town, '') as town, 
                  COALESCE(p.region, '') as region, 
                  p.country_code,
                  COALESCE(p.postcode, '') as postcode, 
                  COALESCE(p.closed, false) as closed,
                  ST_Distance(p.location, ST_SetSRID(ST_MakePoint($2, $1), 4326)::geography) as distance_meters,
                  ST_Y(p.location::geometry) as lat,
                  ST_X(p.location::geometry) as lon,
                  s.latest_year,
                  s.total_years as total_years_rank,
                  s.current_streak,
                  p.whatpub_id,
                  p.google_maps_id,
                  p.untappd_id
           FROM pubs p
           LEFT JOIN pub_stats s ON p.id = s.pub_id
           WHERE p.location IS NOT NULL 
             AND ST_DWithin(p.location, ST_SetSRID(ST_MakePoint($2, $1), 4326)::geography, $3)
             {}
           {} LIMIT 50"#,
            open_filter,
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
                  p.untappd_id, p.google_maps_id, p.whatpub_id, p.rgl_id, p.untappd_verified,
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
        untappd_verified: row.get("untappd_verified"),
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

#[server(UpdatePub, "/api")]
pub async fn update_pub(
    id: Uuid,
    name: String,
    address: String,
    town: String,
    region: String,
    country_code: Option<String>,
    postcode: String,
    closed: bool,
    lat: Option<f64>,
    lon: Option<f64>,
    untappd_id: Option<String>,
    google_maps_id: Option<String>,
    whatpub_id: Option<String>,
    rgl_id: Option<String>,
    years: Vec<i32>,
) -> Result<(), ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use crate::auth::session;

    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let session = extract::<Session>().await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let user = session::get_user(&session).await
        .ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    // 1. Get old values for audit log
    let old_pub = get_pub_detail(id).await?;

    // 2. Update pub
    sqlx::query!(
        r#"UPDATE pubs SET 
            name = $1, address = $2, town = $3, region = $4, country_code = $5, postcode = $6, 
            closed = $7, 
            location = CASE WHEN $8::float8 IS NOT NULL AND $9::float8 IS NOT NULL 
                       THEN ST_SetSRID(ST_MakePoint($9, $8), 4326)::geography 
                       ELSE location END,
            untappd_id = $11, google_maps_id = $12, whatpub_id = $13, rgl_id = $14
           WHERE id = $10"#,
        name, address, town, region, country_code, postcode, closed, lat, lon, id,
        untappd_id, google_maps_id, whatpub_id, rgl_id
    )
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    // 3. Update history
    sqlx::query!("DELETE FROM gbg_history WHERE pub_id = $1", id)
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    for year in years.clone() {
        sqlx::query!("INSERT INTO gbg_history (pub_id, year) VALUES ($1, $2)", id, year)
            .execute(&pool)
            .await
            .map_err(|e| ServerFnError::new(e.to_string()))?;
    }

    // 4. Create audit log
    let old_json = serde_json::to_value(&old_pub).unwrap();
    let new_json = serde_json::json!({
        "name": name, "address": address, "town": town, "region": region,
        "country_code": country_code, "postcode": postcode, "closed": closed,
        "lat": lat, "lon": lon, "untappd_id": untappd_id,
        "google_maps_id": google_maps_id, "whatpub_id": whatpub_id,
        "rgl_id": rgl_id, "years": years
    });

    sqlx::query!(
        "INSERT INTO audit_log (user_id, action, entity_type, entity_id, old_value, new_value)
         VALUES ($1, $2, $3, $4, $5, $6)",
        user.id, "UPDATE_PUB", "pub", id, old_json, new_json
    )
    .execute(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    // 5. Refresh stats
    sqlx::query!("REFRESH MATERIALIZED VIEW pub_stats")
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[server(LogVisit, "/api")]
pub async fn log_visit(pub_id: Uuid, visit_date: String, notes: Option<String>) -> Result<(), ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use crate::auth::session;

    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;
    let session = extract::<Session>().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let user = session::get_user(&session).await.ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    let date = chrono::NaiveDate::parse_from_str(&visit_date, "%Y-%m-%d")
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    // Use query (non-macro) to avoid NaiveDate issues with macros
    sqlx::query(
        "INSERT INTO user_visits (user_id, pub_id, visit_date, notes) VALUES ($1, $2, $3, $4)
         ON CONFLICT (user_id, pub_id, visit_date) DO NOTHING"
    )
    .bind(user.id)
    .bind(pub_id)
    .bind(date)
    .bind(notes)
    .execute(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[server(GetUserVisits, "/api")]
pub async fn get_user_visits() -> Result<Vec<crate::models::VisitRecord>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use crate::auth::session;

    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;
    let session = extract::<Session>().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let user = session::get_user(&session).await.ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    let visits = sqlx::query_as!(
        crate::models::VisitRecord,
        r#"SELECT v.id, v.pub_id, p.name as "pub_name", p.town, p.region, v.visit_date as "visit_date: chrono::NaiveDate", v.notes
           FROM user_visits v
           JOIN pubs p ON v.pub_id = p.id
           WHERE v.user_id = $1
           ORDER BY v.visit_date DESC"#,
        user.id
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(visits)
}

#[server(GetPubVisitStatus, "/api")]
pub async fn get_pub_visit_status(pub_id: Uuid) -> Result<bool, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use crate::auth::session;

    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;
    let session = extract::<Session>().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let user = session::get_user(&session).await;

    if let Some(user) = user {
        let count = sqlx::query_scalar!(
            "SELECT COUNT(*) FROM user_visits WHERE user_id = $1 AND pub_id = $2",
            user.id, pub_id
        )
        .fetch_one(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(count.unwrap_or(0) > 0)
    } else {
        Ok(false)
    }
}

#[server(FetchFlickrPhoto, "/api")]
pub async fn fetch_flickr_photo(url_or_id: String) -> Result<crate::models::FlickrPhotoInfo, ServerFnError> {
    use crate::models::FlickrPhotoInfo;
    let api_key = std::env::var("FLICKR_API_KEY").map_err(|_| ServerFnError::new("FLICKR_API_KEY not set"))?;
    
    // Extract photo ID (simple version: last part of URL or just the ID)
    let photo_id = url_or_id.split('/').filter(|s| !s.is_empty()).last().ok_or_else(|| ServerFnError::new("Invalid Flickr URL or ID"))?;
    
    let client = reqwest::Client::new();
    
    // 1. Get Info
    let info_resp = client.get("https://www.flickr.com/services/rest/")
        .query(&[
            ("method", "flickr.photos.getInfo"),
            ("api_key", &api_key),
            ("photo_id", photo_id),
            ("format", "json"),
            ("nojsoncallback", "1"),
        ])
        .send().await.map_err(|e| ServerFnError::new(e.to_string()))?;
        
    let info_json: serde_json::Value = info_resp.json().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    
    if info_json["stat"] != "ok" {
        return Err(ServerFnError::new(format!("Flickr API error: {}", info_json["message"])));
    }
    
    let photo = &info_json["photo"];
    let owner_name = photo["owner"]["realname"].as_str().filter(|s| !s.is_empty())
        .unwrap_or_else(|| photo["owner"]["username"].as_str().unwrap_or("Unknown"));
    let title = photo["title"]["_content"].as_str().unwrap_or("Untitled");
    let license_id = photo["license"].as_str().unwrap_or("0");
    
    // License mapping (Flickr IDs for CC)
    // 1: Attrib-NC-SA, 2: Attrib-NC, 3: Attrib-NC-ND, 4: Attrib, 5: Attrib-SA, 6: Attrib-ND, 9: CC0, 10: Public Domain
    let (license_type, license_url, is_cc) = match license_id {
        "1" => ("Attribution-NonCommercial-ShareAlike", "https://creativecommons.org/licenses/by-nc-sa/2.0/", true),
        "2" => ("Attribution-NonCommercial", "https://creativecommons.org/licenses/by-nc/2.0/", true),
        "3" => ("Attribution-NonCommercial-NoDerivs", "https://creativecommons.org/licenses/by-nc-nd/2.0/", true),
        "4" => ("Attribution", "https://creativecommons.org/licenses/by/2.0/", true),
        "5" => ("Attribution-ShareAlike", "https://creativecommons.org/licenses/by-sa/2.0/", true),
        "6" => ("Attribution-NoDerivs", "https://creativecommons.org/licenses/by-nd/2.0/", true),
        "9" => ("CC0 1.0 Universal", "https://creativecommons.org/publicdomain/zero/1.0/", true),
        "10" => ("Public Domain Mark 1.0", "https://creativecommons.org/publicdomain/mark/1.0/", true),
        _ => ("All Rights Reserved", "https://www.flickr.com/help/usage/", false),
    };
    
    // 2. Get Sizes
    let sizes_resp = client.get("https://www.flickr.com/services/rest/")
        .query(&[
            ("method", "flickr.photos.getSizes"),
            ("api_key", &api_key),
            ("photo_id", photo_id),
            ("format", "json"),
            ("nojsoncallback", "1"),
        ])
        .send().await.map_err(|e| ServerFnError::new(e.to_string()))?;
        
    let sizes_json: serde_json::Value = sizes_resp.json().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    
    let sizes = sizes_json["sizes"]["size"].as_array().ok_or_else(|| ServerFnError::new("No sizes found"))?;
    
    // Prefer "Large" or "Medium 800" or just the largest available
    let large_size = sizes.iter().find(|s| s["label"] == "Large")
        .or_else(|| sizes.iter().find(|s| s["label"] == "Medium 800"))
        .unwrap_or_else(|| sizes.last().unwrap());
        
    let image_url = large_size["source"].as_str().unwrap().to_string();
    let original_url = format!("https://www.flickr.com/photos/{}/{}", photo["owner"]["nsid"].as_str().unwrap(), photo_id);

    Ok(FlickrPhotoInfo {
        flickr_id: photo_id.to_string(),
        title: title.to_string(),
        owner_name: owner_name.to_string(),
        image_url,
        original_url,
        license_type: license_type.to_string(),
        license_url: license_url.to_string(),
        is_cc_licensed: is_cc,
    })
}

#[server(AddPubPhoto, "/api")]
pub async fn add_pub_photo(pub_id: Uuid, flickr_info: crate::models::FlickrPhotoInfo) -> Result<(), ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use crate::auth::session;

    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;
    let session = extract::<Session>().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let user = session::get_user(&session).await.ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    // Use query (non-macro) to avoid issues with macros and nullable bools
    sqlx::query(
        r#"INSERT INTO pub_photos (pub_id, user_id, flickr_id, image_url, owner_name, license_type, license_url, original_url, is_cc_licensed)
           VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#
    )
    .bind(pub_id)
    .bind(user.id)
    .bind(flickr_info.flickr_id)
    .bind(flickr_info.image_url)
    .bind(flickr_info.owner_name)
    .bind(flickr_info.license_type)
    .bind(flickr_info.license_url)
    .bind(flickr_info.original_url)
    .bind(flickr_info.is_cc_licensed)
    .execute(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[server(GetPubPhotos, "/api")]
pub async fn get_pub_photos(pub_id: Uuid) -> Result<Vec<crate::models::PubPhoto>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    
    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;

    let photos = sqlx::query_as!(
        crate::models::PubPhoto,
        r#"SELECT id, pub_id, flickr_id, image_url, original_url, owner_name, license_type, license_url, COALESCE(is_cc_licensed, TRUE) as "is_cc_licensed!: bool"
           FROM pub_photos WHERE pub_id = $1 ORDER BY created_at DESC"#,
        pub_id
    )
    .fetch_all(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(photos)
}

#[cfg(feature = "ssr")]
fn get_webauthn() -> Result<webauthn_rs::Webauthn, ServerFnError> {
    let rp_id = std::env::var("RP_ID").unwrap_or_else(|_| "localhost".to_string());
    let rp_origin_str = std::env::var("RP_ORIGIN").unwrap_or_else(|_| "http://localhost:3000".to_string());
    let rp_origin = url::Url::parse(&rp_origin_str).map_err(|e| ServerFnError::new(e.to_string()))?;
    
    let builder = webauthn_rs::WebauthnBuilder::new(&rp_id, &rp_origin)
        .map_err(|e| ServerFnError::new(e.to_string()))?;
        
    builder.build().map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(StartPasskeyRegistration, "/api")]
pub async fn start_passkey_registration() -> Result<serde_json::Value, ServerFnError> {
    use leptos_axum::extract;
    use tower_sessions::Session;
    use crate::auth::session;

    let session = extract::<Session>().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let user = session::get_user(&session).await.ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    let webauthn = get_webauthn()?;
    
    let (challenge, registration_state) = webauthn.start_passkey_registration(user.id, &user.username, &user.username, None)
        .map_err(|e| ServerFnError::new(e.to_string()))?;
        
    session.insert("registration_state", registration_state).await.map_err(|e| ServerFnError::new(e.to_string()))?;
    
    Ok(serde_json::to_value(challenge).unwrap())
}

#[server(FinishPasskeyRegistration, "/api")]
pub async fn finish_passkey_registration(reg_response: serde_json::Value) -> Result<(), ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use crate::auth::session;
    use webauthn_rs::prelude::*;

    let reg_response: RegisterPublicKeyCredential = serde_json::from_value(reg_response).map_err(|e| ServerFnError::new(e.to_string()))?;

    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;
    let session = extract::<Session>().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let user = session::get_user(&session).await.ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    let registration_state: PasskeyRegistration = session.get("registration_state").await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("Registration state not found in session"))?;
        
    let webauthn = get_webauthn()?;
    let passkey = webauthn.finish_passkey_registration(&reg_response, &registration_state)
        .map_err(|e| ServerFnError::new(e.to_string()))?;
        
    let credential_id = passkey.cred_id().to_vec();
    let public_key = serde_json::to_vec(&passkey).map_err(|e| ServerFnError::new(e.to_string()))?;
    
    sqlx::query!(
        "INSERT INTO user_credentials (user_id, credential_id, public_key) VALUES ($1, $2, $3)",
        user.id, credential_id, public_key
    )
    .execute(&pool).await.map_err(|e: sqlx::Error| ServerFnError::new(e.to_string()))?;
    
    session.remove::<PasskeyRegistration>("registration_state").await.map_err(|e| ServerFnError::new(e.to_string()))?;
    
    Ok(())
}

#[server(StartPasskeyAuthentication, "/api")]
pub async fn start_passkey_authentication(username: String) -> Result<serde_json::Value, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use webauthn_rs::prelude::*;

    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;
    let session = extract::<Session>().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let user_creds = sqlx::query!(
        "SELECT c.public_key FROM user_credentials c JOIN users u ON c.user_id = u.id WHERE u.username = $1",
        username
    )
    .fetch_all(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;
    
    let passkeys: Vec<Passkey> = user_creds.iter()
        .map(|c| serde_json::from_slice(&c.public_key).unwrap())
        .collect();
        
    let webauthn = get_webauthn()?;
    let (challenge, authentication_state) = webauthn.start_passkey_authentication(&passkeys)
        .map_err(|e| ServerFnError::new(e.to_string()))?;
        
    session.insert("authentication_state", authentication_state).await.map_err(|e| ServerFnError::new(e.to_string()))?;
    session.insert("auth_username", username).await.map_err(|e| ServerFnError::new(e.to_string()))?;
    
    Ok(serde_json::to_value(challenge).unwrap())
}

#[server(FinishPasskeyAuthentication, "/api")]
pub async fn finish_passkey_authentication(auth_response: serde_json::Value) -> Result<bool, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use crate::auth::{session, User};
    use webauthn_rs::prelude::*;

    let auth_response: PublicKeyCredential = serde_json::from_value(auth_response).map_err(|e| ServerFnError::new(e.to_string()))?;

    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;
    let session = extract::<Session>().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    let authentication_state: PasskeyAuthentication = session.get("authentication_state").await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("Authentication state not found in session"))?;
        
    let username: String = session.get("auth_username").await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("Username not found in session"))?;
        
    let webauthn = get_webauthn()?;
    let auth_result = webauthn.finish_passkey_authentication(&auth_response, &authentication_state)
        .map_err(|e| ServerFnError::new(e.to_string()))?;
        
    // Update sign count
    let credential_id = auth_result.cred_id().to_vec();
    sqlx::query!(
        "UPDATE user_credentials SET sign_count = $1 WHERE credential_id = $2",
        auth_result.counter() as i64, credential_id
    )
    .execute(&pool).await.map_err(|e: sqlx::Error| ServerFnError::new(e.to_string()))?;
    
    // Login user
    let user_data = sqlx::query!(
        "SELECT id, username, role, totp_setup_completed FROM users WHERE username = $1",
        username
    )
    .fetch_one(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;
    
    session::login(&session, &User {
        id: user_data.id,
        username: user_data.username,
        role: user_data.role,
        totp_setup_completed: user_data.totp_setup_completed,
    }).await.map_err(|e| ServerFnError::new(e.to_string()))?;
    
    session.remove::<PasskeyAuthentication>("authentication_state").await.map_err(|e| ServerFnError::new(e.to_string()))?;
    session.remove::<String>("auth_username").await.map_err(|e| ServerFnError::new(e.to_string()))?;
    
    Ok(true)
}

#[server(SuggestUpdate, "/api")]
pub async fn suggest_update(pub_id: Uuid, suggested_data: serde_json::Value) -> Result<(), ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use crate::auth::session;

    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;
    let session = extract::<Session>().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let user = session::get_user(&session).await.ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    sqlx::query!(
        "INSERT INTO suggested_updates (pub_id, user_id, suggested_data) VALUES ($1, $2, $3)",
        pub_id, user.id, suggested_data
    )
    .execute(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[server(GetSuggestedUpdates, "/api")]
pub async fn get_suggested_updates(status: Option<String>) -> Result<Vec<crate::models::SuggestedUpdate>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use crate::auth::session;

    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;
    let session = extract::<Session>().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let user = session::get_user(&session).await.ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    if user.role != "admin" {
        return Err(ServerFnError::new("Unauthorized"));
    }

    let status_filter = status.unwrap_or_else(|| "pending".to_string());

    let suggestions = sqlx::query_as::<sqlx::Postgres, crate::models::SuggestedUpdate>(
        r#"SELECT s.id, s.pub_id, p.name as pub_name, s.user_id, u.username, s.status, s.suggested_data, s.created_at
           FROM suggested_updates s
           JOIN pubs p ON s.pub_id = p.id
           JOIN users u ON s.user_id = u.id
           WHERE s.status = $1
           ORDER BY s.created_at DESC"#
    )
    .bind(status_filter)
    .fetch_all(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(suggestions)
}

#[server(ProcessSuggestedUpdate, "/api")]
pub async fn process_suggested_update(suggestion_id: Uuid, approve: bool) -> Result<(), ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use crate::auth::session;

    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;
    let session = extract::<Session>().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let user = session::get_user(&session).await.ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    if user.role != "admin" {
        return Err(ServerFnError::new("Unauthorized"));
    }

    let status = if approve { "approved" } else { "rejected" };

    if approve {
        let suggestion = sqlx::query!(
            "SELECT pub_id, suggested_data FROM suggested_updates WHERE id = $1",
            suggestion_id
        ).fetch_one(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;

        let data = suggestion.suggested_data;
        
        // Patch the pub record
        sqlx::query!(
            "UPDATE pubs SET 
                name = COALESCE($1, name),
                address = COALESCE($2, address),
                town = COALESCE($3, town),
                region = COALESCE($4, region),
                postcode = COALESCE($5, postcode),
                closed = COALESCE($6, closed),
                whatpub_id = COALESCE($7, whatpub_id),
                google_maps_id = COALESCE($8, google_maps_id),
                untappd_id = COALESCE($9, untappd_id)
             WHERE id = $10",
            data["name"].as_str(),
            data["address"].as_str(),
            data["town"].as_str(),
            data["region"].as_str(),
            data["postcode"].as_str(),
            data["closed"].as_bool(),
            data["whatpub_id"].as_str(),
            data["google_maps_id"].as_str(),
            data["untappd_id"].as_str(),
            suggestion.pub_id
        ).execute(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;

        // Update years if present
        if let Some(years) = data["years"].as_array() {
            let years_vec: Vec<i32> = years.iter().filter_map(|v| v.as_i64().map(|y| y as i32)).collect();
            
            // This is simple but effective: clear and re-insert
            sqlx::query!("DELETE FROM gbg_history WHERE pub_id = $1", suggestion.pub_id)
                .execute(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;
                
            for year in years_vec {
                sqlx::query!("INSERT INTO gbg_history (pub_id, year) VALUES ($1, $2)", suggestion.pub_id, year)
                    .execute(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;
            }
        }
    }

    sqlx::query!(
        "UPDATE suggested_updates SET status = $1, processed_at = CURRENT_TIMESTAMP, processed_by = $2 WHERE id = $3",
        status, user.id, suggestion_id
    )
    .execute(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[server(BulkUpdatePubs, "/api")]
pub async fn bulk_update_pubs(
    region: Option<String>,
    town: Option<String>,
    outcode: Option<String>,
    closed: Option<bool>,
    untappd_verified: Option<bool>,
) -> Result<u64, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use crate::auth::session;

    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;
    let session = extract::<Session>().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let user = session::get_user(&session).await.ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    if user.role != "admin" {
        return Err(ServerFnError::new("Unauthorized"));
    }

    let mut query = String::from("UPDATE pubs SET id = id"); // No-op to start
    let mut param_idx = 1;
    let mut binds = Vec::new();

    if let Some(c) = closed {
        query.push_str(&format!(", closed = ${}", param_idx));
        param_idx += 1;
        binds.push(c.to_string());
    }
    if let Some(v) = untappd_verified {
        query.push_str(&format!(", untappd_verified = ${}", param_idx));
        param_idx += 1;
        binds.push(v.to_string());
    }

    query.push_str(" WHERE 1=1");

    if let Some(ref r) = region {
        query.push_str(&format!(" AND region = ${}", param_idx));
        param_idx += 1;
        binds.push(r.clone());
    }
    if let Some(ref t) = town {
        query.push_str(&format!(" AND town = ${}", param_idx));
        param_idx += 1;
        binds.push(t.clone());
    }
    if let Some(ref o) = outcode {
        query.push_str(&format!(" AND SPLIT_PART(postcode, ' ', 1) = ${}", param_idx));
        param_idx += 1;
        binds.push(o.clone());
    }

    // This is a bit tricky with sqlx because of dynamic number of binds and types.
    // For now, we just implement a simplified version or use a macro if possible.
    // Given the constraints, I'll use a direct query for simple cases.
    
    // ... bind manually based on what we added ...
    // Since this is complex to do generically in sqlx without a lot of boilerplate,
    // I'll stick to a more restricted but safe implementation if needed.
    
    let res = sqlx::query::<sqlx::Postgres>("UPDATE pubs SET closed = COALESCE($1, closed), untappd_verified = COALESCE($2, untappd_verified) 
                           WHERE (region = $3 OR $3 IS NULL) 
                             AND (town = $4 OR $4 IS NULL) 
                             AND (SPLIT_PART(postcode, ' ', 1) = $5 OR $5 IS NULL)")
        .bind(closed).bind(untappd_verified).bind(region).bind(town).bind(outcode)
        .execute(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(res.rows_affected())
}

#[server(ExportUserVisits, "/api")]
pub async fn export_user_visits(format: String) -> Result<String, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use crate::auth::session;

    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;
    let session = extract::<Session>().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let user = session::get_user(&session).await.ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    let visits = sqlx::query_as!(
        crate::models::VisitRecord,
        r#"SELECT v.id, v.pub_id, p.name as "pub_name", p.town, p.region, v.visit_date as "visit_date: chrono::NaiveDate", v.notes
           FROM user_visits v
           JOIN pubs p ON v.pub_id = p.id
           WHERE v.user_id = $1
           ORDER BY v.visit_date DESC"#,
        user.id
    )
    .fetch_all(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    match format.as_str() {
        "json" => {
            let json = serde_json::to_string_pretty(&visits).map_err(|e| ServerFnError::new(e.to_string()))?;
            Ok(json)
        },
        "parquet" => {
            use parquet::arrow::ArrowWriter;
            use arrow::array::{StringArray, Date32Array};
            use arrow::record_batch::RecordBatch;
            use arrow::datatypes::{Schema, Field, DataType};
            use std::sync::Arc;
            use base64::{Engine as _, engine::general_purpose::STANDARD};

            let schema = Arc::new(Schema::new(vec![
                Field::new("visit_date", DataType::Date32, false),
                Field::new("pub_name", DataType::Utf8, false),
                Field::new("town", DataType::Utf8, true),
                Field::new("region", DataType::Utf8, true),
                Field::new("notes", DataType::Utf8, true),
            ]));

            let date_array = Date32Array::from(visits.iter().map(|v| {
                // days since Unix epoch
                let epoch = chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap();
                (v.visit_date - epoch).num_days() as i32
            }).collect::<Vec<_>>());
            let name_array = StringArray::from(visits.iter().map(|v| v.pub_name.as_str()).collect::<Vec<_>>());
            let town_array = StringArray::from(visits.iter().map(|v| v.town.as_deref()).collect::<Vec<_>>());
            let region_array = StringArray::from(visits.iter().map(|v| v.region.as_deref()).collect::<Vec<_>>());
            let notes_array = StringArray::from(visits.iter().map(|v| v.notes.as_deref()).collect::<Vec<_>>());

            let batch = RecordBatch::try_new(schema.clone(), vec![
                Arc::new(date_array),
                Arc::new(name_array),
                Arc::new(town_array),
                Arc::new(region_array),
                Arc::new(notes_array),
            ]).map_err(|e| ServerFnError::new(e.to_string()))?;

            let mut buf = Vec::new();
            let mut writer = ArrowWriter::try_new(&mut buf, schema, None).map_err(|e| ServerFnError::new(e.to_string()))?;
            writer.write(&batch).map_err(|e| ServerFnError::new(e.to_string()))?;
            writer.close().map_err(|e| ServerFnError::new(e.to_string()))?;

            Ok(STANDARD.encode(buf))
        },
        _ => {
            // Default to CSV
            let mut csv = String::from("date,pub_name,town,region,notes\n");
            for v in visits {
                csv.push_str(&format!(
                    "{},\"{}\",\"{}\",\"{}\",\"{}\"\n",
                    v.visit_date,
                    v.pub_name.replace("\"", "\"\""),
                    v.town.unwrap_or_default().replace("\"", "\"\""),
                    v.region.unwrap_or_default().replace("\"", "\"\""),
                    v.notes.unwrap_or_default().replace("\"", "\"\"")
                ));
            }
            Ok(csv)
        }
    }
}

#[server(CreateInvite, "/api")]
pub async fn create_invite(role: String, expires_in_days: i64) -> Result<Uuid, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use crate::auth::session;

    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;
    let session = extract::<Session>().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let user = session::get_user(&session).await.ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    if user.role != "admin" {
        return Err(ServerFnError::new("Unauthorized"));
    }

    let expires_at = chrono::Utc::now() + chrono::Duration::days(expires_in_days);

    // Use query_scalar (non-macro) to avoid OffsetDateTime issues with macros
    let invite_id: Uuid = sqlx::query_scalar(
        "INSERT INTO user_invites (role, expires_at, created_by) VALUES ($1, $2, $3) RETURNING id"
    )
    .bind(role)
    .bind(expires_at)
    .bind(user.id)
    .fetch_one(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(invite_id)
}

#[server(GetInvites, "/api")]
pub async fn get_invites() -> Result<serde_json::Value, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use crate::auth::session;

    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;
    let session = extract::<Session>().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    let user = session::get_user(&session).await.ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    if user.role != "admin" {
        return Err(ServerFnError::new("Unauthorized"));
    }

    let invites = sqlx::query_as!(
        UserInvite,
        r#"SELECT id, role, expires_at as "expires_at: chrono::DateTime<chrono::Utc>", used_at as "used_at: chrono::DateTime<chrono::Utc>" FROM user_invites ORDER BY expires_at DESC"#
    )
    .fetch_all(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(serde_json::to_value(invites).unwrap())
}

#[server(RegisterWithInvite, "/api")]
pub async fn register_with_invite(invite_id: Uuid, username: String, password: String) -> Result<(), ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use argon2::{password_hash::SaltString, Argon2, PasswordHasher};
    use rand::rngs::OsRng;

    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;

    // 1. Verify invite
    let invite = sqlx::query_as!(
        UserInvite,
        r#"SELECT id, role, expires_at as "expires_at: chrono::DateTime<chrono::Utc>", used_at as "used_at: chrono::DateTime<chrono::Utc>" FROM user_invites WHERE id = $1"#,
        invite_id
    )
    .fetch_optional(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?
    .ok_or_else(|| ServerFnError::new("Invalid invite"))?;

    if invite.used_at.is_some() {
        return Err(ServerFnError::new("Invite already used"));
    }
    if invite.expires_at < chrono::Utc::now() {
        return Err(ServerFnError::new("Invite expired"));
    }

    // 2. Hash password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .to_string();

    // 3. Create user
    // Generate a temporary TOTP secret that must be changed
    let mut totp_secret = vec![0u8; 20];
    getrandom::getrandom(&mut totp_secret).map_err(|e| ServerFnError::new(e.to_string()))?;

    let mut tx = pool.begin().await.map_err(|e| ServerFnError::new(e.to_string()))?;
    
    sqlx::query!(
        "INSERT INTO users (username, password_hash, role, totp_setup_completed, totp_secret_enc, recovery_codes_hash) 
         VALUES ($1, $2, $3, $4, $5, $6)",
        username, password_hash, invite.role, false, totp_secret, &Vec::<String>::new()
    )
    .execute(&mut *tx).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    sqlx::query!(
        "UPDATE user_invites SET used_at = CURRENT_TIMESTAMP WHERE id = $1",
        invite_id
    )
    .execute(&mut *tx).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    tx.commit().await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}

#[server(GetAuditLogs, "/api")]
pub async fn get_audit_logs() -> Result<Vec<crate::models::AuditLogEntry>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use crate::models::AuditLogEntry;

    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let logs = sqlx::query_as::<_, AuditLogEntry>(
        r#"SELECT l.id, u.username, l.action, l.entity_type, l.entity_id, l.timestamp
           FROM audit_log l
           JOIN users u ON l.user_id = u.id
           ORDER BY l.timestamp DESC LIMIT 50"#
    )
    .fetch_all(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(logs)
}

#[server(Logout, "/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    use tower_sessions::Session;
    use leptos_axum::extract;
    use crate::auth::session;

    let session = extract::<Session>().await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    session::logout(&session).await.map_err(|e| ServerFnError::new(e.to_string()))?;
    
    leptos_axum::redirect("/login");
    Ok(())
}

#[server(GetCurrentUser, "/api")]
pub async fn get_current_user() -> Result<Option<User>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use tower_sessions::Session;
    use leptos_axum::extract;
    use crate::auth::session;

    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;
    
    if let Ok(s) = extract::<Session>().await {
        if let Some(user_session) = session::get_user(&s).await {
            // Fetch fresh data from DB
            let user = sqlx::query!(
                "SELECT id, username, role, totp_setup_completed FROM users WHERE id = $1",
                user_session.id
            ).fetch_one(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;

            leptos::logging::log!("GetCurrentUser: id={}, username={}, role={}", user.id, user.username, user.role);

            Ok(Some(User {
                id: user.id,
                username: user.username,
                role: user.role,
                totp_setup_completed: user.totp_setup_completed,
            }))
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

#[server(CheckUserAuthType, "/api")]
pub async fn check_user_auth_type(username: String) -> Result<UserAuthStatus, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use sqlx::Row;
    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;

    let user = sqlx::query("SELECT id, totp_setup_completed FROM users WHERE username = $1")
        .bind(&username)
        .fetch_optional(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    match user {
        Some(u) => {
            let user_id: uuid::Uuid = u.get("id");
            let setup_completed: bool = u.get("totp_setup_completed");
            
            let passkeys_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM user_credentials WHERE user_id = $1")
                .bind(user_id)
                .fetch_one(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;

            Ok(UserAuthStatus {
                user_id: Some(user_id),
                has_passkeys: passkeys_count > 0,
                totp_required: !setup_completed,
            })
        }
        None => Ok(UserAuthStatus {
            user_id: None,
            has_passkeys: false,
            totp_required: false,
        })
    }
}

#[server(GetTotpSetupInfo, "/api")]
pub async fn get_totp_setup_info() -> Result<serde_json::Value, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use crate::auth::session;
    use totp_rs::{Algorithm, TOTP};

    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;
    let session = extract::<Session>().await?;
    let user = session::get_user(&session).await.ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    let user_data = sqlx::query!(
        "SELECT username, totp_secret_enc FROM users WHERE id = $1",
        user.id
    ).fetch_one(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    // Generate secret if missing (empty)
    let secret_bytes = if !user_data.totp_secret_enc.is_empty() {
        user_data.totp_secret_enc
    } else {
        use rand::RngCore;
        let mut new_secret = vec![0u8; 20];
        rand::thread_rng().fill_bytes(&mut new_secret);
        
        sqlx::query!(
            "UPDATE users SET totp_secret_enc = $1 WHERE id = $2",
            &new_secret, user.id
        ).execute(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;
        
        new_secret
    };

    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret_bytes,
        Some("GBGData".to_string()),
        user_data.username.clone(),
    ).map_err(|e| ServerFnError::new(e.to_string()))?;

    let otp_url = totp.get_url();
    if otp_url.is_empty() {
        return Err(ServerFnError::new("Failed to generate OTP URL"));
    }

    let qr_code_svg = {
        use qrcodegen::{QrCode, QrCodeEcc};
        let qr = QrCode::encode_text(&otp_url, QrCodeEcc::Medium).map_err(|e| ServerFnError::new(e.to_string()))?;
        let size = qr.size();
        let border = 4;
        let total_size = size + border * 2;
        
        let mut svg = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {0} {0}\" shape-rendering=\"crispEdges\" style=\"background: white; padding: 10px; border-radius: 8px;\">\n", total_size);
        svg.push_str(&format!("  <rect width=\"{}\" height=\"{}\" fill=\"white\" />\n", total_size, total_size));
        svg.push_str(&format!("  <g transform=\"translate({}, {})\" fill=\"black\">\n", border, border));
        for y in 0..size {
            for x in 0..size {
                if qr.get_module(x, y) {
                    svg.push_str(&format!("    <rect x=\"{}\" y=\"{}\" width=\"1\" height=\"1\" />\n", x, y));
                }
            }
        }
        svg.push_str("  </g>\n</svg>");
        svg
    };
    
    let secret = totp.get_secret_base32();

    Ok(serde_json::json!({
        "qr_code": qr_code_svg,
        "url": otp_url,
        "secret": secret
    }))
}

#[server(VerifyAndCompleteTotpSetup, "/api")]
pub async fn verify_and_complete_totp_setup(user_id: Uuid, code: String) -> Result<bool, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use crate::auth::{verify_totp, User, session};
    
    let pool = use_context::<PgPool>()
        .ok_or_else(|| ServerFnError::new("Pool not found in context"))?;

    let session = extract::<Session>().await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    let user_data = sqlx::query!(
        "SELECT id, username, role, totp_secret_enc FROM users WHERE id = $1",
        user_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;

    if verify_totp(&user_data.username, &user_data.totp_secret_enc, &code) {
        sqlx::query!(
            "UPDATE users SET totp_setup_completed = true WHERE id = $1",
            user_id
        )
        .execute(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

        session::login(&session, &User {
            id: user_data.id,
            username: user_data.username,
            role: user_data.role,
            totp_setup_completed: true,
        }).await.map_err(|e| ServerFnError::new(e.to_string()))?;

        Ok(true)
    } else {
        Ok(false)
    }
}

#[server(GetMissingDataReports, "/api")]
pub async fn get_missing_data_reports(report_type: String) -> Result<Vec<PubSummary>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;

    let query = match report_type.as_str() {
        "coords" => "SELECT p.id, p.name, p.town, p.region, p.country_code, p.postcode, p.closed, NULL::float8 as distance_meters, NULL::float8 as lat, NULL::float8 as lon, s.latest_year, s.total_years as total_years_rank, s.current_streak, p.whatpub_id, p.google_maps_id, p.untappd_id FROM pubs p LEFT JOIN pub_stats s ON p.id = s.pub_id WHERE p.location IS NULL LIMIT 100",
        "ids" => "SELECT p.id, p.name, p.town, p.region, p.country_code, p.postcode, p.closed, NULL::float8 as distance_meters, CASE WHEN p.location IS NOT NULL THEN ST_Y(p.location::geometry) ELSE NULL END as lat, CASE WHEN p.location IS NOT NULL THEN ST_X(p.location::geometry) ELSE NULL END as lon, s.latest_year, s.total_years as total_years_rank, s.current_streak, p.whatpub_id, p.google_maps_id, p.untappd_id FROM pubs p LEFT JOIN pub_stats s ON p.id = s.pub_id WHERE p.whatpub_id IS NULL OR p.google_maps_id IS NULL OR p.untappd_id IS NULL LIMIT 100",
        "closed" => "SELECT p.id, p.name, p.town, p.region, p.country_code, p.postcode, p.closed, NULL::float8 as distance_meters, CASE WHEN p.location IS NOT NULL THEN ST_Y(p.location::geometry) ELSE NULL END as lat, CASE WHEN p.location IS NOT NULL THEN ST_X(p.location::geometry) ELSE NULL END as lon, s.latest_year, s.total_years as total_years_rank, s.current_streak, p.whatpub_id, p.google_maps_id, p.untappd_id FROM pubs p JOIN pub_stats s ON p.id = s.pub_id WHERE p.closed = true AND s.latest_year >= 2024 LIMIT 100",
        _ => return Err(ServerFnError::new("Invalid report type")),
    };

    let pubs = sqlx::query_as::<sqlx::Postgres, PubSummary>(query)
        .fetch_all(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(pubs)
}

#[server(GetMyPasskeys, "/api")]
pub async fn get_my_passkeys() -> Result<Vec<crate::models::UserCredential>, ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use crate::auth::session;

    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;
    let session = extract::<Session>().await?;
    let user = session::get_user(&session).await.ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    let passkeys = sqlx::query_as!(
        crate::models::UserCredential,
        "SELECT user_id, credential_id, public_key FROM user_credentials WHERE user_id = $1",
        user.id
    ).fetch_all(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(passkeys)
}

#[server(BulkUpdatePubsList, "/api")]
pub async fn bulk_update_pubs_list(ids: Vec<Uuid>, action: String, value: String) -> Result<(), ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use crate::auth::session;

    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;
    let session = extract::<Session>().await?;
    let user = session::get_user(&session).await.ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    if user.role != "admin" {
        return Err(ServerFnError::new("Only admins can perform bulk updates"));
    }

    for id in ids {
        match action.as_str() {
            "mark_closed" => {
                let is_closed = value == "true";
                sqlx::query!("UPDATE pubs SET closed = $1 WHERE id = $2", is_closed, id)
                    .execute(&pool).await.map_err(|e: sqlx::Error| ServerFnError::new(e.to_string()))?;
                
                sqlx::query!(
                    "INSERT INTO audit_log (user_id, action, entity_type, entity_id) VALUES ($1, $2, $3, $4)",
                    user.id, format!("bulk_update:closed={}", is_closed), "pub", id
                ).execute(&pool).await.map_err(|e: sqlx::Error| ServerFnError::new(e.to_string()))?;
            },
            "add_year" => {
                let year: i32 = value.parse().map_err(|_| ServerFnError::new("Invalid year"))?;
                sqlx::query!("INSERT INTO gbg_history (pub_id, year) VALUES ($1, $2) ON CONFLICT DO NOTHING", id, year)
                    .execute(&pool).await.map_err(|e: sqlx::Error| ServerFnError::new(e.to_string()))?;
                
                sqlx::query!(
                    "INSERT INTO audit_log (user_id, action, entity_type, entity_id) VALUES ($1, $2, $3, $4)",
                    user.id, format!("bulk_update:add_year={}", year), "pub", id
                ).execute(&pool).await.map_err(|e: sqlx::Error| ServerFnError::new(e.to_string()))?;
            },
            "remove_year" => {
                let year: i32 = value.parse().map_err(|_| ServerFnError::new("Invalid year"))?;
                sqlx::query!("DELETE FROM gbg_history WHERE pub_id = $1 AND year = $2", id, year)
                    .execute(&pool).await.map_err(|e: sqlx::Error| ServerFnError::new(e.to_string()))?;
                
                sqlx::query!(
                    "INSERT INTO audit_log (user_id, action, entity_type, entity_id) VALUES ($1, $2, $3, $4)",
                    user.id, format!("bulk_update:remove_year={}", year), "pub", id
                ).execute(&pool).await.map_err(|e: sqlx::Error| ServerFnError::new(e.to_string()))?;
            },
            _ => return Err(ServerFnError::new("Unsupported bulk action")),
        }
    }

    Ok(())
}

#[server(DeletePasskey, "/api")]
pub async fn delete_passkey(credential_id: Vec<u8>) -> Result<(), ServerFnError> {
    use sqlx::PgPool;
    use leptos::context::use_context;
    use leptos_axum::extract;
    use tower_sessions::Session;
    use crate::auth::session;

    let pool = use_context::<PgPool>().ok_or_else(|| ServerFnError::new("Pool not found"))?;
    let session = extract::<Session>().await?;
    let user = session::get_user(&session).await.ok_or_else(|| ServerFnError::new("Unauthorized"))?;

    sqlx::query!(
        "DELETE FROM user_credentials WHERE user_id = $1 AND credential_id = $2",
        user.id, credential_id
    ).execute(&pool).await.map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok(())
}
