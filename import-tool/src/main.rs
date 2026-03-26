mod excel;
mod db;
mod geocoder;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use anyhow::Context;
use geocoder::Geocoder;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL")
        .context("DATABASE_URL must be set in .env or environment")?;
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .context("Failed to connect to the database")?;

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "geocode" {
        geocode_pubs(&pool).await?;
    } else {
        import_pubs(&pool).await?;
    }

    Ok(())
}

async fn import_pubs(pool: &sqlx::PgPool) -> anyhow::Result<()> {
    let excel_path = "GBG counties one sheet Duncan 2025.xlsx";
    println!("Parsing Excel file: {}...", excel_path);
    
    let pubs = excel::parse_excel(excel_path)
        .context("Failed to parse Excel file")?;
    
    println!("Found {} pubs. Starting import...", pubs.len());
    
    let mut imported = 0;
    for p in pubs {
        if let Err(e) = db::insert_pub(pool, &p).await {
            eprintln!("Failed to insert pub '{}': {:?}", p.name, e);
        } else {
            imported += 1;
            if imported % 100 == 0 {
                println!("Imported {} pubs...", imported);
            }
        }
    }
    
    println!("Import complete! Successfully imported {} pubs.", imported);
    Ok(())
}

async fn geocode_pubs(pool: &sqlx::PgPool) -> anyhow::Result<()> {
    let geocoder = Geocoder::new();
    
    let pubs_to_geocode = sqlx::query!(
        "SELECT id, address, town, postcode FROM pubs WHERE location IS NULL LIMIT 100"
    )
    .fetch_all(pool)
    .await?;

    println!("Found {} pubs to geocode. (Batch limit 100)", pubs_to_geocode.len());

    for p in pubs_to_geocode {
        println!("Geocoding: {}, {}...", p.town.as_deref().unwrap_or(""), p.postcode.as_deref().unwrap_or(""));
        match geocoder.geocode(
            p.address.as_deref().unwrap_or(""),
            p.town.as_deref().unwrap_or(""),
            p.postcode.as_deref().unwrap_or("")
        ).await {
            Ok(Some((lat, lon))) => {
                db::update_pub_location(pool, p.id, lat, lon).await?;
                println!("  Found: {}, {}", lat, lon);
            }
            Ok(None) => {
                println!("  Not found.");
            }
            Err(e) => {
                eprintln!("  Error: {:?}", e);
                break; // Stop on fatal errors (like 403)
            }
        }
    }

    println!("Geocoding batch complete.");
    Ok(())
}
