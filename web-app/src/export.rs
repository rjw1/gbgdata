use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use leptos::prelude::LeptosOptions;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use crate::models::PubDetail;

#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: PgPool,
}

#[derive(Deserialize)]
pub struct ExportFilter {
    pub county: Option<String>,
    pub town: Option<String>,
    pub outcode: Option<String>,
}

pub async fn get_export_data(pool: &PgPool, filter: ExportFilter) -> anyhow::Result<Vec<PubDetail>> {
    let mut query = String::from(
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
           WHERE 1=1"#
    );

    if let Some(ref c) = filter.county {
        query.push_str(&format!(" AND p.county = '{}'", c.replace("'", "''")));
    }
    if let Some(ref t) = filter.town {
        query.push_str(&format!(" AND p.town = '{}'", t.replace("'", "''")));
    }
    if let Some(ref o) = filter.outcode {
        query.push_str(&format!(" AND SPLIT_PART(p.postcode, ' ', 1) = '{}'", o.replace("'", "''")));
    }

    query.push_str(" ORDER BY p.name");

    let pubs = sqlx::query_as::<_, PubDetail>(&query)
        .fetch_all(pool)
        .await?;

    Ok(pubs)
}

pub async fn export_json(
    State(state): State<AppState>,
    Query(filter): Query<ExportFilter>,
) -> impl IntoResponse {
    match get_export_data(&state.pool, filter).await {
        Ok(data) => Json(data).into_response(),
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn export_csv(
    State(state): State<AppState>,
    Query(filter): Query<ExportFilter>,
) -> impl IntoResponse {
    use axum::http::header;
    
    match get_export_data(&state.pool, filter).await {
        Ok(data) => {
            let mut wtr = csv::Writer::from_writer(Vec::new());
            // Header is automatic from struct if we used Serialize, but let's be explicit or use Serialize
            for p in data {
                let _ = wtr.serialize(p);
            }
            let csv_data = wtr.into_inner().unwrap_or_default();
            
            (
                [(header::CONTENT_TYPE, "text/csv"), (header::CONTENT_DISPOSITION, "attachment; filename=\"pubs.csv\"")],
                csv_data
            ).into_response()
        }
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
