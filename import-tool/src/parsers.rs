use anyhow::{Result, Context};
use crate::excel::ImportPub;
use std::fs::File;
use std::io::BufReader;
use parquet::arrow::arrow_reader::ParquetRecordBatchReaderBuilder;
use arrow::array::{Array, StringArray, BooleanArray, Float64Array};
use uuid::Uuid;

pub fn parse_csv(path: &str) -> Result<Vec<ImportPub>> {
    let file = File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);
    let mut pubs = Vec::new();

    for result in rdr.deserialize() {
        let record: serde_json::Value = result?;
        
        let years = record["years"].as_str()
            .unwrap_or_default()
            .split(';')
            .filter_map(|s| s.parse::<i32>().ok())
            .collect();

        let name = record["name"].as_str().unwrap_or_default().trim().to_string();
        if !name.is_empty() {
            pubs.push(ImportPub {
                id: record["id"].as_str().and_then(|s| Uuid::parse_str(s).ok()),
                name,
                address: record["address"].as_str().unwrap_or_default().trim().to_string(),
                town: record["town"].as_str().unwrap_or_default().trim().to_string(),
                county: record["county"].as_str().unwrap_or_default().trim().to_string(),
                postcode: record["postcode"].as_str().unwrap_or_default().trim().to_string(),
                closed: record["closed"].as_str().map(|s| s == "true").unwrap_or(false),
                lat: record["lat"].as_str().and_then(|s| s.parse::<f64>().ok()),
                lon: record["lon"].as_str().and_then(|s| s.parse::<f64>().ok()),
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
            p.county = p.county.trim().to_string();
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
        let county_col = batch.column(4).as_any().downcast_ref::<StringArray>().context("Missing county col")?;
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
                    county: county_col.value(i).trim().to_string(),
                    postcode: postcode_col.value(i).trim().to_string(),
                    closed: closed_col.value(i),
                    lat: if lat_col.is_null(i) { None } else { Some(lat_col.value(i)) },
                    lon: if lon_col.is_null(i) { None } else { Some(lon_col.value(i)) },
                    years: Vec::new(),
                });
            }
        }
    }
    
    Ok(pubs)
}
