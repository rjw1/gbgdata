use axum::{
    extract::{Query, State, FromRef},
    response::IntoResponse,
    Json,
};
use leptos::prelude::LeptosOptions;
use serde::Deserialize;
use sqlx::PgPool;
use crate::models::PubDetail;
use anyhow::Result;

#[derive(Clone)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: PgPool,
}

impl FromRef<AppState> for LeptosOptions {
    fn from_ref(state: &AppState) -> Self {
        state.leptos_options.clone()
    }
}

#[derive(Deserialize)]
pub struct ExportFilter {
    pub county: Option<String>,
    pub town: Option<String>,
    pub outcode: Option<String>,
}

pub async fn get_export_data(pool: &PgPool, filter: ExportFilter) -> Result<Vec<PubDetail>> {
    let mut query = String::from(
        r#"SELECT p.id, p.name, 
                  COALESCE(p.address, '') as "address", 
                  COALESCE(p.town, '') as "town", 
                  COALESCE(p.county, '') as "county", 
                  COALESCE(p.postcode, '') as "postcode", 
                  COALESCE(p.closed, false) as "closed",
                  p.untappd_id, p.google_maps_id, p.whatpub_id, p.rgl_id,
                  ST_Y(p.location::geometry) as lat,
                  ST_X(p.location::geometry) as lon,
                  COALESCE(s.current_streak, 0) as "current_streak",
                  COALESCE(s.last_5_years, 0) as "last_5_years",
                  COALESCE(s.last_10_years, 0) as "last_10_years",
                  COALESCE(s.total_years, 0) as "total_years",
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

    let pubs = sqlx::query_as::<sqlx::Postgres, PubDetail>(&query)
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

pub async fn export_parquet(
    State(state): State<AppState>,
    Query(filter): Query<ExportFilter>,
) -> impl IntoResponse {
    use axum::http::header;
    use arrow::array::{StringArray, BooleanArray, Float64Array, Int32Array, Int64Array};
    use arrow::datatypes::{DataType, Field, Schema};
    use arrow::record_batch::RecordBatch;
    use parquet::arrow::arrow_writer::ArrowWriter;
    use std::sync::Arc;

    match get_export_data(&state.pool, filter).await {
        Ok(data) => {
            let schema = Arc::new(Schema::new(vec![
                Field::new("id", DataType::Utf8, false),
                Field::new("name", DataType::Utf8, false),
                Field::new("address", DataType::Utf8, false),
                Field::new("town", DataType::Utf8, false),
                Field::new("county", DataType::Utf8, false),
                Field::new("postcode", DataType::Utf8, false),
                Field::new("closed", DataType::Boolean, false),
                Field::new("lat", DataType::Float64, true),
                Field::new("lon", DataType::Float64, true),
                Field::new("current_streak", DataType::Int32, false),
                Field::new("total_years", DataType::Int64, false),
            ]));

            let id_array = StringArray::from(data.iter().map(|p| p.id.to_string()).collect::<Vec<String>>());
            let name_array = StringArray::from(data.iter().map(|p| p.name.clone()).collect::<Vec<String>>());
            let addr_array = StringArray::from(data.iter().map(|p| p.address.clone()).collect::<Vec<String>>());
            let town_array = StringArray::from(data.iter().map(|p| p.town.clone()).collect::<Vec<String>>());
            let county_array = StringArray::from(data.iter().map(|p| p.county.clone()).collect::<Vec<String>>());
            let post_array = StringArray::from(data.iter().map(|p| p.postcode.clone()).collect::<Vec<String>>());
            let closed_array = BooleanArray::from(data.iter().map(|p| p.closed).collect::<Vec<bool>>());
            let lat_array = Float64Array::from(data.iter().map(|p| p.lat).collect::<Vec<Option<f64>>>());
            let lon_array = Float64Array::from(data.iter().map(|p| p.lon).collect::<Vec<Option<f64>>>());
            let streak_array = Int32Array::from(data.iter().map(|p| p.current_streak).collect::<Vec<i32>>());
            let total_array = Int64Array::from(data.iter().map(|p| p.total_years).collect::<Vec<i64>>());

            let batch = RecordBatch::try_new(schema.clone(), vec![
                Arc::new(id_array),
                Arc::new(name_array),
                Arc::new(addr_array),
                Arc::new(town_array),
                Arc::new(county_array),
                Arc::new(post_array),
                Arc::new(closed_array),
                Arc::new(lat_array),
                Arc::new(lon_array),
                Arc::new(streak_array),
                Arc::new(total_array),
            ]).unwrap();

            let mut buf = Vec::new();
            let mut writer = ArrowWriter::try_new(&mut buf, schema, None).unwrap();
            writer.write(&batch).unwrap();
            writer.close().unwrap();

            (
                [(header::CONTENT_TYPE, "application/vnd.apache.parquet"), (header::CONTENT_DISPOSITION, "attachment; filename=\"pubs.parquet\"")],
                buf
            ).into_response()
        }
        Err(e) => (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
