use calamine::{Reader, Xlsx, open_workbook};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImportPub {
    pub id: Option<Uuid>,
    pub name: String,
    pub address: String,
    pub town: String,
    #[serde(alias = "county")]
    pub region: String,
    #[serde(default)]
    pub country_code: String,
    pub postcode: String,
    pub closed: bool,
    pub lat: Option<f64>,
    pub lon: Option<f64>,
    pub untappd_id: Option<String>,
    #[serde(default)]
    pub untappd_verified: bool,
    pub years: Vec<i32>,
}

pub fn parse_excel(path: &str) -> Result<Vec<ImportPub>> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let sheet_name = "List of all pubs";
    let range = workbook.worksheet_range(sheet_name)?;
    
    let mut pubs = Vec::new();
    // Skip first 5 rows (header is row 4, data starts at 5)
    for row in range.rows().skip(5) {
        let country_code = row.get(0).map(|d| d.to_string()).unwrap_or_default().trim().to_string();
        let region = row.get(1).map(|d| d.to_string()).unwrap_or_default().trim().to_string();
        let town = row.get(2).map(|d| d.to_string()).unwrap_or_default().trim().to_string();
        let name = row.get(3).map(|d| d.to_string()).unwrap_or_default().trim().to_string();
        let address = row.get(4).map(|d| d.to_string()).unwrap_or_default().trim().to_string();
        let postcode = row.get(5).map(|d| d.to_string()).unwrap_or_default().trim().to_string();
        let closed_raw = row.get(10).map(|d| d.to_string()).unwrap_or_default().trim().to_uppercase();
        
        let closed = !closed_raw.is_empty() && closed_raw != "F" && closed_raw != "W";

        let mut years = Vec::new();
        for i in 11..=64 {
            if let Some(cell) = row.get(i) {
                let cell_str = cell.to_string();
                if !cell_str.trim().is_empty() {
                    if let Ok(year) = cell_str.parse::<i32>() {
                        years.push(year);
                    } else if let Ok(year_float) = cell_str.parse::<f64>() {
                         years.push(year_float as i32);
                    }
                }
            }
        }

        if !name.is_empty() {
            pubs.push(ImportPub {
                id: None,
                name,
                address,
                town,
                region,
                country_code,
                postcode,
                closed,
                lat: None,
                lon: None,
                untappd_id: None,
                untappd_verified: false,
                years,
            });
        }
    }
    Ok(pubs)
}
