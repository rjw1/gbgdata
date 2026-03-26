mod excel;
mod db;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use anyhow::Context;

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

    let excel_path = "GBG counties one sheet Duncan 2025.xlsx";

    println!("Parsing Excel file: {}...", excel_path);
    
    let pubs = excel::parse_excel(excel_path)
        .context("Failed to parse Excel file")?;
    
    println!("Found {} pubs. Starting import...", pubs.len());
    
    let mut imported = 0;
    for p in pubs {
        if let Err(e) = db::insert_pub(&pool, &p).await {
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
