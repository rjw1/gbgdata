mod db;
mod excel;
mod geocoder;
mod parsers;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use dotenvy::dotenv;
use geocoder::Geocoder;
use sqlx::postgres::PgPoolOptions;
use std::env;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Command to run: import (default), geocode, create-admin, or migrate
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

    /// Username for the new admin (create-admin command)
    #[arg(long)]
    username: Option<String>,

    /// Password for the new admin (create-admin command)
    #[arg(long)]
    password: Option<String>,
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

    // Always run migrations
    println!("Running database migrations...");
    sqlx::migrate!("../migrations").run(&pool).await?;

    match args.command.as_str() {
        "geocode" => run_geocoder(&pool, args.batch).await?,
        "create-admin" => run_create_admin(&pool, args).await?,
        "migrate" => {
            println!("Migrations complete.");
        }
        _ => run_import(&pool, args).await?,
    }

    Ok(())
}

async fn run_create_admin(pool: &sqlx::PgPool, args: Args) -> Result<()> {
    use argon2::{
        password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
        Argon2,
    };
    use rand::distributions::{Alphanumeric, DistString};
    use totp_rs::{Algorithm, TOTP};

    let username = args
        .username
        .ok_or_else(|| anyhow::anyhow!("--username is required"))?;
    let password = args
        .password
        .ok_or_else(|| anyhow::anyhow!("--password is required"))?;

    println!("Creating admin user: {}...", username);

    // 1. Hash password
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?
        .to_string();

    // 2. Generate TOTP secret
    use rand::RngCore;
    let mut totp_secret = vec![0u8; 20];
    rand::thread_rng().fill_bytes(&mut totp_secret);
    let totp = TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        totp_secret.clone(),
        Some("GBGData".to_string()),
        username.clone(),
    )
    .map_err(|e| anyhow::anyhow!("Failed to create TOTP: {}", e))?;

    // 3. Generate recovery codes
    let mut recovery_codes = Vec::new();
    for _ in 0..5 {
        let code = Alphanumeric.sample_string(&mut rand::thread_rng(), 10);
        recovery_codes.push(code);
    }

    // Hash recovery codes before storing
    let hashed_recovery_codes: Vec<String> = recovery_codes
        .iter()
        .map(|code| {
            let salt = SaltString::generate(&mut OsRng);
            argon2
                .hash_password(code.as_bytes(), &salt)
                .unwrap()
                .to_string()
        })
        .collect();

    // 4. Save to DB
    // In a real app, we should encrypt totp_secret. For now, we'll store it as is (bytea).
    db::create_user(
        pool,
        &username,
        &password_hash,
        &totp_secret,
        hashed_recovery_codes,
    )
    .await?;

    println!("\nSUCCESS: Admin user created.");
    println!("Username: {}", username);
    println!("\nIMPORTANT: Set up your TOTP authenticator app now.");
    println!("TOTP Secret (Base32): {}", totp.get_secret_base32());
    println!("TOTP Setup URI: {}", totp.get_url());
    println!("\nRECOVERY CODES (Store these securely!):");
    for code in &recovery_codes {
        println!("  {}", code);
    }

    Ok(())
}

async fn run_import(pool: &sqlx::PgPool, args: Args) -> Result<()> {
    let file_path = args
        .file
        .unwrap_or_else(|| "GBG counties one sheet Duncan 2025.xlsx".to_string());

    println!(
        "Importing from {} (Format: {:?})...",
        file_path, args.format
    );

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
        "SELECT id, name, COALESCE(address, '') as address, COALESCE(town, '') as town, COALESCE(postcode, '') as postcode, COALESCE(region, '') as region
         FROM pubs WHERE location IS NULL AND closed = false LIMIT $1",
        limit
    )
    .fetch_all(pool)
    .await?;

    let total = pubs.len();
    println!("Found {} pubs. Starting geocoding...", total);
    let geocoder = Geocoder::new();

    if std::env::var("NOMINATIM_URL")
        .unwrap_or_default()
        .is_empty()
    {
        println!("WARNING: NOMINATIM_URL not set. Skipping geocoding.");
    }

    for (i, p) in pubs.into_iter().enumerate() {
        let name = p.name;
        let town = p.town.unwrap_or_default();
        let address = p.address.unwrap_or_default();
        let postcode = p.postcode.unwrap_or_default();
        let region = p.region.unwrap_or_default();

        println!("[{}/{}] Geocoding {} in {}...", i + 1, total, name, town);

        match geocoder
            .geocode(&name, &address, &town, &postcode, &region)
            .await
        {
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
