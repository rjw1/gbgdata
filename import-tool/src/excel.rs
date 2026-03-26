use calamine::{Reader, Xlsx, open_workbook};
use anyhow::Result;

#[derive(Debug)]
pub struct RawPub {
    pub name: String,
    pub address: String,
    pub town: String,
    pub county: String,
    pub postcode: String,
    pub closed: bool,
    pub years: Vec<i32>,
}

pub fn parse_excel(path: &str) -> Result<Vec<RawPub>> {
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let sheet_name = "List of all pubs";
    let range = workbook.worksheet_range(sheet_name)?;
    
    let mut pubs = Vec::new();
    // Skip first 5 rows (header is row 4, data starts at 5)
    for row in range.rows().skip(5) {
        let county = row.get(1).map(|d| d.to_string()).unwrap_or_default();
        let town = row.get(2).map(|d| d.to_string()).unwrap_or_default();
        let name = row.get(3).map(|d| d.to_string()).unwrap_or_default();
        let address = row.get(4).map(|d| d.to_string()).unwrap_or_default();
        let postcode = row.get(5).map(|d| d.to_string()).unwrap_or_default();
        let closed_raw = row.get(10).map(|d| d.to_string()).unwrap_or_default();
        let closed = !closed_raw.trim().is_empty();

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
            pubs.push(RawPub {
                name,
                address,
                town,
                county,
                postcode,
                closed,
                years,
            });
        }
    }
    Ok(pubs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_excel_real_file() {
        // This test uses the real file to verify the logic.
        // It's brittle but good for initial verification.
        // We know the file is at ../../../GBG counties one sheet Duncan 2025.xlsx relative to the project root
        // When running cargo test, it runs from the package root (import-tool/)
        let path = "../../../GBG counties one sheet Duncan 2025.xlsx";
        let result = parse_excel(path);
        assert!(result.is_ok(), "Failed to parse excel file: {:?}", result.err());
        let pubs = result.unwrap();
        assert!(!pubs.is_empty(), "Parsed zero pubs from excel file");
        
        // Basic spot check
        let first_pub = &pubs[0];
        assert!(!first_pub.name.is_empty());
        assert!(!first_pub.county.is_empty());
        println!("Parsed {} pubs", pubs.len());
        println!("First pub: {:?}", first_pub);
    }
}
