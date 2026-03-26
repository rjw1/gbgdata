use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PubSummary {
    pub id: Uuid,
    pub name: String,
    pub town: String,
    pub county: String,
    pub postcode: String,
    pub closed: bool,
    pub distance_meters: Option<f64>,
    pub latest_year: Option<i32>,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PubDetail {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub town: String,
    pub county: String,
    pub postcode: String,
    pub closed: bool,
    pub untappd_id: Option<String>,
    pub google_maps_id: Option<String>,
    pub whatpub_id: Option<String>,
    pub rgl_id: Option<String>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub current_streak: i32,
    pub last_5_years: i64,
    pub last_10_years: i64,
    pub total_years: i64,
    pub first_year: Option<i32>,
    pub latest_year: Option<i32>,
    pub years: Vec<i32>,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CountySummary {
    pub name: String,
    pub pub_count: i64,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TownSummary {
    pub name: String,
    pub pub_count: i64,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OutcodeSummary {
    pub name: String,
    pub pub_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CountyDetails {
    pub name: String,
    pub towns: Vec<TownSummary>,
    pub outcodes: Vec<OutcodeSummary>,
}
