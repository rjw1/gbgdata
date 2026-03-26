use anyhow::Result;
use crate::excel::RawPub;
use sqlx::{PgPool, Postgres, Transaction};

/// Inserts a pub and its historical inclusion records into the database.
///
/// This operation is performed within a transaction to ensure that either both the pub
/// and its history are recorded, or nothing is recorded in case of an error.
pub async fn insert_pub(pool: &PgPool, raw_pub: &RawPub) -> Result<()> {
    let mut tx: Transaction<'_, Postgres> = pool.begin().await?;

    // Insert the pub and get the generated UUID
    let pub_id: uuid::Uuid = sqlx::query_scalar(
        "INSERT INTO pubs (name, address, town, county, postcode, closed) 
         VALUES ($1, $2, $3, $4, $5, $6) 
         RETURNING id",
    )
    .bind(&raw_pub.name)
    .bind(&raw_pub.address)
    .bind(&raw_pub.town)
    .bind(&raw_pub.county)
    .bind(&raw_pub.postcode)
    .bind(raw_pub.closed)
    .fetch_one(&mut *tx)
    .await?;

    // Insert each year the pub was in the Good Beer Guide
    for year in &raw_pub.years {
        sqlx::query(
            "INSERT INTO gbg_history (pub_id, year) 
             VALUES ($1, $2) 
             ON CONFLICT (pub_id, year) DO NOTHING",
        )
        .bind(pub_id)
        .bind(year)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}
