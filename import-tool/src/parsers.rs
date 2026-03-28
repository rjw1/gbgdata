use anyhow::{Result, Context};
use crate::excel::ImportPub;
use std::fs::File;
use std::io::BufReader;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use arrow::array::{Array, StringArray, BooleanArray, Float64Array};
use uuid::Uuid;

use std::collections::HashMap;

pub fn parse_csv(path: &str) -> Result<Vec<ImportPub>> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut pubs = Vec::new();

    for result in rdr.deserialize() {
        let record: HashMap<String, String> = result?;
        
        let years = record.get("years")
            .unwrap_or(&String::new())
            .split(';')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let name = record.get("name").map(|s| s.trim().to_string()).unwrap_or_default();
        if !name.is_empty() {
            pubs.push(ImportPub {
                id: record.get("id").and_then(|s| Uuid::parse_str(s).ok()),
                name,
                address: record.get("address").map(|s| s.trim().to_string()).unwrap_or_default(),
                town: record.get("town").map(|s| s.trim().to_string()).unwrap_or_default(),
                region: record.get("region").or(record.get("county")).map(|s| s.trim().to_string()).unwrap_or_default(),
                country_code: record.get("country_code").map(|s| s.trim().to_string()).unwrap_or_default(),
                postcode: record.get("postcode").map(|s| s.trim().to_string()).unwrap_or_default(),
                closed: record.get("closed").map(|s| s == "true").unwrap_or(false),
                lat: record.get("lat").and_then(|s| s.parse::<f64>().ok()),
                lon: record.get("lon").and_then(|s| s.parse::<f64>().ok()),
                untappd_id: record.get("untappd_id").map(|s| s.trim().to_string()),
                untappd_verified: record.get("untappd_verified").map(|s| s == "true").unwrap_or(false),
                years,
            });
        }
    }
    Ok(pubs)
}

pub fn parse_json(path: &str) -> Result<Vec<ImportPub>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let raw_pubs: Vec<ImportPub> = serde_json::from_reader(reader)?;
    
    // Filter out empty names and trim
    let pubs = raw_pubs.into_iter().filter_map(|mut p| {
        p.name = p.name.trim().to_string();
        if p.name.is_empty() {
            None
        } else {
            p.address = p.address.trim().to_string();
            p.town = p.town.trim().to_string();
            p.region = p.region.trim().to_string();
            p.postcode = p.postcode.trim().to_string();
            Some(p)
        }
    }).collect();

    Ok(pubs)
}

pub fn parse_parquet(path: &str) -> Result<Vec<ImportPub>> {
    let file = File::open(path)?;
    let builder = ParquetRecordBatchReaderBuilder::try_new(file)?;
    let mut reader = builder.build()?;
    
    let mut pubs = Vec::new();

    while let Some(batch_res) = reader.next() {
        let batch = batch_res?;
        
        let id_col = batch.column(0).as_any().downcast_ref::<StringArray>().context("Missing id col")?;
        let name_col = batch.column(1).as_any().downcast_ref::<StringArray>().context("Missing name col")?;
        let addr_col = batch.column(2).as_any().downcast_ref::<StringArray>().context("Missing address col")?;
        let town_col = batch.column(3).as_any().downcast_ref::<StringArray>().context("Missing town col")?;
        let region_col = batch.column(4).as_any().downcast_ref::<StringArray>().context("Missing region (formerly county) col")?;
        let postcode_col = batch.column(5).as_any().downcast_ref::<StringArray>().context("Missing postcode col")?;
        let closed_col = batch.column(6).as_any().downcast_ref::<BooleanArray>().context("Missing closed col")?;
        let lat_col = batch.column(7).as_any().downcast_ref::<Float64Array>().context("Missing lat col")?;
        let lon_col = batch.column(8).as_any().downcast_ref::<Float64Array>().context("Missing lon col")?;

        for i in 0..batch.num_rows() {
            let name = name_col.value(i).trim().to_string();
            if !name.is_empty() {
                pubs.push(ImportPub {
                    id: Some(Uuid::parse_str(id_col.value(i))?),
                    name,
                    address: addr_col.value(i).trim().to_string(),
                    town: town_col.value(i).trim().to_string(),
                    region: region_col.value(i).trim().to_string(),
                    country_code: String::new(), // Default for now
                    postcode: postcode_col.value(i).trim().to_string(),
                    closed: closed_col.value(i),
                    lat: if lat_col.is_null(i) { None } else { Some(lat_col.value(i)) },
                    lon: if lon_col.is_null(i) { None } else { Some(lon_col.value(i)) },
                    untappd_id: None, // Parquet schema doesn't have these yet
                    untappd_verified: false,
                    years: Vec::new(),
                });
            }
        }
    }
    
    Ok(pubs)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn test_parse_json_basic() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, r#"[
            {{ "name": " The Anchor ", "address": " 1 High St ", "town": " Town ", "region": " Region ", "country_code": "E", "postcode": " PC1 ", "closed": false, "years": [] }},
            {{ "name": "", "address": "Hidden", "town": "Town", "region": "Region", "postcode": "PC", "closed": true, "years": [] }}
        ]"#)?;

        let pubs = parse_json(file.path().to_str().unwrap())?;
        assert_eq!(pubs.len(), 1);
        assert_eq!(pubs[0].name, "The Anchor");
        assert_eq!(pubs[0].address, "1 High St");
        assert_eq!(pubs[0].town, "Town");
        assert_eq!(pubs[0].region, "Region");
        assert_eq!(pubs[0].country_code, "E");
        assert_eq!(pubs[0].postcode, "PC1");
        Ok(())
    }

    #[test]
    fn test_parse_csv_basic() -> Result<()> {
        let mut file = NamedTempFile::new()?;
        writeln!(file, "name,address,town,region,country_code,postcode,closed,years")?;
        writeln!(file, "\" The Bell \",\" 2 Main St \",\" Town \",\" Region \",\"W\",\" PC2 \",false,\"2021;2022\"")?;
        writeln!(file, "\"\",\"Hidden\",\"Town\",\"Region\",\"\",\"PC\",true,\"\"")?;

        let result = parse_csv(file.path().to_str().unwrap())?;
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "The Bell");
        assert_eq!(result[0].address, "2 Main St");
        assert_eq!(result[0].region, "Region");
        assert_eq!(result[0].country_code, "W");
        assert_eq!(result[0].years, vec![2021, 2022]);
        Ok(())
    }
}
