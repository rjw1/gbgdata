use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub role: String,
    pub totp_setup_completed: bool,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PubSummary {
    pub id: Uuid,
    pub name: String,
    pub town: String,
    pub region: String,
    pub country_code: Option<String>,
    pub postcode: String,
    pub closed: bool,
    pub distance_meters: Option<f64>,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub latest_year: Option<i32>,
    pub total_years_rank: Option<i64>,
    pub current_streak: Option<i32>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
pub enum SortMode {
    #[default]
    Name,
    Streak,
    TotalEntries,
    Distance,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PubDetail {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub town: String,
    pub region: String,
    pub country_code: Option<String>,
    pub postcode: String,
    pub closed: bool,
    pub untappd_id: Option<String>,
    pub google_maps_id: Option<String>,
    pub whatpub_id: Option<String>,
    pub rgl_id: Option<String>,
    pub untappd_verified: bool,
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
pub struct RegionSummary {
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

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct YearSummary {
    pub year: i32,
    pub pub_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RegionDetails {
    pub name: String,
    pub towns: Vec<TownSummary>,
    pub outcodes: Vec<OutcodeSummary>,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VisitRecord {
    pub id: Uuid,
    pub pub_id: Uuid,
    pub pub_name: String,
    pub visit_date: chrono::NaiveDate,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FlickrPhotoInfo {
    pub flickr_id: String,
    pub title: String,
    pub owner_name: String,
    pub image_url: String,
    pub original_url: String,
    pub license_type: String,
    pub license_url: String,
    pub is_cc_licensed: bool,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PubPhoto {
    pub id: Uuid,
    pub pub_id: Uuid,
    pub flickr_id: Option<String>,
    pub image_url: String,
    pub original_url: String,
    pub owner_name: String,
    pub license_type: String,
    pub license_url: String,
    pub is_cc_licensed: bool,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SuggestedUpdate {
    pub id: Uuid,
    pub pub_id: Uuid,
    pub pub_name: String,
    pub user_id: Uuid,
    pub username: String,
    pub status: String,
    pub suggested_data: serde_json::Value,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AuditLogEntry {
    pub id: i32,
    pub username: String,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Uuid,
    pub timestamp: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserInvite {
    pub id: Uuid,
    pub role: String,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub used_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserCredential {
    pub user_id: Uuid,
    pub credential_id: Vec<u8>,
    pub public_key: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserAuthStatus {
    pub user_id: Option<Uuid>,
    pub has_passkeys: bool,
    pub totp_required: bool,
}
