#[cfg(feature = "ssr")]
pub mod ssr_export {
    use crate::models::PubDetail;
    use anyhow::Result;
    use axum::{
        extract::{FromRef, Query, State},
        response::IntoResponse,
        Json,
    };
    use leptos::prelude::LeptosOptions;
    use serde::Deserialize;
    use sqlx::{PgPool, Row};

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
        pub region: Option<String>,
        pub town: Option<String>,
        pub outcode: Option<String>,
        pub year: Option<i32>,
    }

    impl ExportFilter {
        pub fn get_filename(&self, ext: &str) -> String {
            let mut parts = vec!["gbg-pubs".to_string()];
            if let Some(y) = self.year {
                parts.push(y.to_string());
            }
            if let Some(ref r) = self.region {
                parts.push(r.replace(" ", "_"));
            }
            if let Some(ref t) = self.town {
                parts.push(t.replace(" ", "_"));
            }
            if let Some(ref o) = self.outcode {
                parts.push(o.replace(" ", "_"));
            }
            format!("{}.{}", parts.join("-"), ext)
        }
    }

    pub async fn get_export_data(pool: &PgPool, filter: &ExportFilter) -> Result<Vec<PubDetail>> {
        let mut query = String::from(
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
                      s.latest_year,
                      COALESCE((SELECT ARRAY_AGG(year ORDER BY year DESC) FROM gbg_history WHERE pub_id = p.id), ARRAY[]::integer[]) as years
               FROM pubs p
               LEFT JOIN pub_stats s ON p.id = s.pub_id"#,
        );

        if filter.year.is_some() {
            query.push_str(" JOIN gbg_history h ON p.id = h.pub_id");
        }

        query.push_str(" WHERE 1=1");

        if let Some(ref r) = filter.region {
            query.push_str(&format!(" AND p.region = '{}'", r.replace("'", "''")));
        }
        if let Some(ref t) = filter.town {
            query.push_str(&format!(" AND p.town = '{}'", t.replace("'", "''")));
        }
        if let Some(ref o) = filter.outcode {
            query.push_str(&format!(
                " AND SPLIT_PART(p.postcode, ' ', 1) = '{}'",
                o.replace("'", "''")
            ));
        }
        if let Some(y) = filter.year {
            query.push_str(&format!(" AND h.year = {}", y));
        }

        query.push_str(" ORDER BY p.name");

        let rows = sqlx::query(&query).fetch_all(pool).await?;

        let pubs = rows
            .into_iter()
            .map(|row| PubDetail {
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
                years: row.get("years"),
            })
            .collect();

        Ok(pubs)
    }

    pub fn pub_list_to_csv(data: Vec<PubDetail>) -> Result<Vec<u8>> {
        let mut wtr = csv::WriterBuilder::new()
            .has_headers(true)
            .from_writer(Vec::new());

        wtr.write_record([
            "id",
            "name",
            "address",
            "town",
            "region",
            "postcode",
            "closed",
            "untappd_id",
            "google_maps_id",
            "whatpub_id",
            "rgl_id",
            "lat",
            "lon",
            "current_streak",
            "last_5_years",
            "last_10_years",
            "total_years",
            "first_year",
            "latest_year",
            "years",
        ])?;

        for p in data {
            let years_str = p
                .years
                .iter()
                .map(|y| y.to_string())
                .collect::<Vec<_>>()
                .join(";");
            wtr.write_record([
                p.id.to_string(),
                p.name,
                p.address,
                p.town,
                p.region,
                p.postcode,
                p.closed.to_string(),
                p.untappd_id.unwrap_or_default(),
                p.google_maps_id.unwrap_or_default(),
                p.whatpub_id.unwrap_or_default(),
                p.rgl_id.unwrap_or_default(),
                p.lat.map(|v| v.to_string()).unwrap_or_default(),
                p.lon.map(|v| v.to_string()).unwrap_or_default(),
                p.current_streak.to_string(),
                p.last_5_years.to_string(),
                p.last_10_years.to_string(),
                p.total_years.to_string(),
                p.first_year.map(|v| v.to_string()).unwrap_or_default(),
                p.latest_year.map(|v| v.to_string()).unwrap_or_default(),
                years_str,
            ])?;
        }
        wtr.into_inner().map_err(|e| e.into_error().into())
    }

    pub fn pub_list_to_parquet(data: Vec<PubDetail>) -> Result<Vec<u8>, anyhow::Error> {
        use arrow::array::{BooleanArray, Float64Array, Int32Array, Int64Array, StringArray};
        use arrow::datatypes::{DataType, Field, Schema};
        use arrow::record_batch::RecordBatch;
        use parquet::arrow::arrow_writer::ArrowWriter;
        use std::sync::Arc;

        let schema = Arc::new(Schema::new(vec![
            Field::new("id", DataType::Utf8, false),
            Field::new("name", DataType::Utf8, false),
            Field::new("address", DataType::Utf8, false),
            Field::new("town", DataType::Utf8, false),
            Field::new("region", DataType::Utf8, false),
            Field::new("postcode", DataType::Utf8, false),
            Field::new("closed", DataType::Boolean, false),
            Field::new("lat", DataType::Float64, true),
            Field::new("lon", DataType::Float64, true),
            Field::new("current_streak", DataType::Int32, false),
            Field::new("total_years", DataType::Int64, false),
        ]));

        let id_array = StringArray::from(
            data.iter()
                .map(|p| p.id.to_string())
                .collect::<Vec<String>>(),
        );
        let name_array =
            StringArray::from(data.iter().map(|p| p.name.clone()).collect::<Vec<String>>());
        let addr_array = StringArray::from(
            data.iter()
                .map(|p| p.address.clone())
                .collect::<Vec<String>>(),
        );
        let town_array =
            StringArray::from(data.iter().map(|p| p.town.clone()).collect::<Vec<String>>());
        let region_array = StringArray::from(
            data.iter()
                .map(|p| p.region.clone())
                .collect::<Vec<String>>(),
        );
        let post_array = StringArray::from(
            data.iter()
                .map(|p| p.postcode.clone())
                .collect::<Vec<String>>(),
        );
        let closed_array = BooleanArray::from(data.iter().map(|p| p.closed).collect::<Vec<bool>>());
        let lat_array =
            Float64Array::from(data.iter().map(|p| p.lat).collect::<Vec<Option<f64>>>());
        let lon_array =
            Float64Array::from(data.iter().map(|p| p.lon).collect::<Vec<Option<f64>>>());
        let streak_array =
            Int32Array::from(data.iter().map(|p| p.current_streak).collect::<Vec<i32>>());
        let total_array =
            Int64Array::from(data.iter().map(|p| p.total_years).collect::<Vec<i64>>());

        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(id_array),
                Arc::new(name_array),
                Arc::new(addr_array),
                Arc::new(town_array),
                Arc::new(region_array),
                Arc::new(post_array),
                Arc::new(closed_array),
                Arc::new(lat_array),
                Arc::new(lon_array),
                Arc::new(streak_array),
                Arc::new(total_array),
            ],
        )?;

        let mut buf = Vec::new();
        let mut writer = ArrowWriter::try_new(&mut buf, schema, None)?;
        writer.write(&batch)?;
        writer.close()?;

        Ok(buf)
    }

    pub async fn export_json(
        State(state): State<AppState>,
        Query(filter): Query<ExportFilter>,
    ) -> impl IntoResponse {
        use axum::http::header;
        match get_export_data(&state.pool, &filter).await {
            Ok(data) => (
                [
                    (header::CONTENT_TYPE, "application/json"),
                    (
                        header::CONTENT_DISPOSITION,
                        &format!("attachment; filename=\"{}\"", filter.get_filename("json")),
                    ),
                ],
                Json(data),
            )
                .into_response(),
            Err(e) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }

    pub async fn export_csv(
        State(state): State<AppState>,
        Query(filter): Query<ExportFilter>,
    ) -> impl IntoResponse {
        use axum::http::header;

        match get_export_data(&state.pool, &filter).await {
            Ok(data) => match pub_list_to_csv(data) {
                Ok(csv_data) => (
                    [
                        (header::CONTENT_TYPE, "text/csv"),
                        (
                            header::CONTENT_DISPOSITION,
                            &format!("attachment; filename=\"{}\"", filter.get_filename("csv")),
                        ),
                    ],
                    csv_data,
                )
                    .into_response(),
                Err(e) => {
                    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
                }
            },
            Err(e) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }

    pub async fn export_parquet(
        State(state): State<AppState>,
        Query(filter): Query<ExportFilter>,
    ) -> impl IntoResponse {
        use axum::http::header;

        match get_export_data(&state.pool, &filter).await {
            Ok(data) => match pub_list_to_parquet(data) {
                Ok(buf) => (
                    [
                        (header::CONTENT_TYPE, "application/vnd.apache.parquet"),
                        (
                            header::CONTENT_DISPOSITION,
                            &format!(
                                "attachment; filename=\"{}\"",
                                filter.get_filename("parquet")
                            ),
                        ),
                    ],
                    buf,
                )
                    .into_response(),
                Err(e) => {
                    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
                }
            },
            Err(e) => {
                (axum::http::StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
            }
        }
    }
}

#[cfg(feature = "ssr")]
pub use ssr_export::*;
