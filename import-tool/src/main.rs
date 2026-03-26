mod excel;
mod db;
mod geocoder;
mod parsers;

use sqlx::postgres::PgPoolOptions;
use std::env;
use dotenvy::dotenv;
use anyhow::Result;
use clap::{Parser, ValueEnum};
use geocoder::Geocoder;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Command to run: import (default) or geocode
    #[arg(index = 1, default_value = "import")]
    command: String,

    /// Path to the file to import
    #[arg(short, long)]
    file: Option<String>,

    /// Format of the file
    #[arg(short = 'F', long, value_enum, default_value = "excel")]
    format: Format,

    /// Batch size for geocoding
    #[arg(short, long, default_value_t = 100)]
    batch: i64,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
enum Format {
    Excel,
    Csv,
    Json,
    Parquet,
}

#[tokio::main]
async fn main() -> Result<()> {
    dotenv().ok();
    let args = Args::parse();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    if args.command == "geocode" {
        run_geocoder(&pool, args.batch).await?;
    } else {
        run_import(&pool, args).await?;
    }

    Ok(())
}

async fn run_import(pool: &sqlx::PgPool, args: Args) -> Result<()> {
    let file_path = args.file.unwrap_or_else(|| "GBG counties one sheet Duncan 2025.xlsx".to_string());
    
    println!("Importing from {} (Format: {:?})...", file_path, args.format);

    let pubs = match args.format {
        Format::Excel => excel::parse_excel(&file_path)?,
        Format::Csv => parsers::parse_csv(&file_path)?,
        Format::Json => parsers::parse_json(&file_path)?,
        Format::Parquet => parsers::parse_parquet(&file_path)?,
    };

    println!("Found {} pubs. Starting import...", pubs.len());

    for (i, p) in pubs.into_iter().enumerate() {
        if let Err(e) = db::insert_pub(pool, &p).await {
            eprintln!("Error importing pub {}: {}", p.name, e);
        }
        if (i + 1) % 100 == 0 {
            println!("Imported {} pubs...", i + 1);
        }
    }

    println!("Import complete! Refreshing statistics view...");
    db::refresh_pub_stats(pool).await?;
    println!("Statistics refreshed.");
    Ok(())
}

async fn run_geocoder(pool: &sqlx::PgPool, limit: i64) -> Result<()> {
    println!("Fetching {} pubs needing geocoding...", limit);
    
    let pubs = sqlx::query!(
        "SELECT id, name, COALESCE(address, '') as address, COALESCE(town, '') as town, COALESCE(postcode, '') as postcode 
         FROM pubs WHERE location IS NULL AND closed = false LIMIT $1",
        limit
    )
    .fetch_all(pool)
    .await?;

    let total = pubs.len();
    println!("Found {} pubs. Starting geocoding...", total);
    let geocoder = Geocoder::new();

    for (i, p) in pubs.into_iter().enumerate() {
        let name = p.name;
        let town = p.town.unwrap_or_default();
        let address = p.address.unwrap_or_default();
        let postcode = p.postcode.unwrap_or_default();

        println!("[{}/{}] Geocoding {} in {}...", i + 1, total, name, town);
        
        match geocoder.geocode(&address, &town, &postcode).await {
            Ok(Some((lat, lon))) => {
                db::update_pub_location(pool, p.id, lat, lon).await?;
                println!("  Found: {}, {}", lat, lon);
            }
            Ok(None) => {
                println!("  Not found");
            }
            Err(e) => {
                eprintln!("  Error: {}", e);
            }
        }
    }

    println!("Geocoding complete! Refreshing statistics view...");
    db::refresh_pub_stats(pool).await?;
    println!("Statistics refreshed.");
    Ok(())
}
