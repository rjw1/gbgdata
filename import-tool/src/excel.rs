use anyhow::Result;
use calamine::{open_workbook, Reader, Xlsx};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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

pub fn row_to_import_pub(row: &[calamine::Data]) -> Option<ImportPub> {
    let country_code = row
        .first()
        .map(|d| d.to_string())
        .unwrap_or_default()
        .trim()
        .to_string();
    let region = row
        .get(1)
        .map(|d| d.to_string())
        .unwrap_or_default()
        .trim()
        .to_string();
    let town = row
        .get(2)
        .map(|d| d.to_string())
        .unwrap_or_default()
        .trim()
        .to_string();
    let name = row
        .get(3)
        .map(|d| d.to_string())
        .unwrap_or_default()
        .trim()
        .to_string();
    let address = row
        .get(4)
        .map(|d| d.to_string())
        .unwrap_or_default()
        .trim()
        .to_string();
    let postcode = row
        .get(5)
        .map(|d| d.to_string())
        .unwrap_or_default()
        .trim()
        .to_string();
    let closed_raw = row
        .get(10)
        .map(|d| d.to_string())
        .unwrap_or_default()
        .trim()
        .to_uppercase();

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
        Some(ImportPub {
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
        })
    } else {
        None
    }
}

pub fn parse_excel(path: &str) -> Result<Vec<ImportPub>> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let sheet_name = "List of all pubs";
    let range = workbook.worksheet_range(sheet_name)?;

    let mut pubs = Vec::new();
    // Skip first 5 rows (header is row 4, data starts at 5)
    for row in range.rows().skip(5) {
        if let Some(pub_info) = row_to_import_pub(row) {
            pubs.push(pub_info);
        }
    }
    Ok(pubs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use calamine::Data;

    #[test]
    fn test_row_mapping() {
        let mut row = vec![Data::Empty; 15];
        row[0] = Data::String("E".to_string());
        row[1] = Data::String("Kent".to_string());
        row[2] = Data::String("Dover".to_string());
        row[3] = Data::String("The Port".to_string());
        row[4] = Data::String("1 High St".to_string());
        row[5] = Data::String("CT16 1AA".to_string());
        row[10] = Data::Empty; // not closed
        row[11] = Data::Int(2024);
        row[12] = Data::Float(2023.0);

        let pub_info = row_to_import_pub(&row).unwrap();
        assert_eq!(pub_info.name, "The Port");
        assert_eq!(pub_info.country_code, "E");
        assert_eq!(pub_info.years, vec![2024, 2023]);
        assert!(!pub_info.closed);
    }

    #[test]
    fn test_closed_mapping() {
        let mut row = vec![Data::Empty; 15];
        row[3] = Data::String("Closed Pub".to_string());
        row[10] = Data::String("Y".to_string());
        let pub_info = row_to_import_pub(&row).unwrap();
        assert!(pub_info.closed);

        row[10] = Data::String("F".to_string()); // False/Open
        let pub_info = row_to_import_pub(&row).unwrap();
        assert!(!pub_info.closed);
    }
}
