use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PubSummary {
    pub id: Uuid,
    pub name: String,
    pub town: String,
    pub county: String,
    pub postcode: String,
    pub closed: bool,
}

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
    pub years: Vec<i32>,
}
