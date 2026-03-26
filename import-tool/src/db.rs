use sqlx::{PgPool, Postgres, Transaction};
use crate::excel::RawPub;
use anyhow::Result;

pub async fn insert_pub(pool: &PgPool, p: &RawPub) -> Result<()> {
    let mut tx: Transaction<'_, Postgres> = pool.begin().await?;

    let row: (uuid::Uuid,) = sqlx::query_as(
        "INSERT INTO pubs (name, address, town, county, postcode, closed) 
         VALUES ($1, $2, $3, $4, $5, $6) RETURNING id"
    )
    .bind(&p.name)
    .bind(&p.address)
    .bind(&p.town)
    .bind(&p.county)
    .bind(&p.postcode)
    .bind(p.closed)
    .fetch_one(&mut *tx)
    .await?;

    let pub_id = row.0;

    for year in &p.years {
        sqlx::query(
            "INSERT INTO gbg_history (pub_id, year) VALUES ($1, $2) ON CONFLICT DO NOTHING"
        )
        .bind(pub_id)
        .bind(year)
        .execute(&mut *tx)
        .await?;
    }

    tx.commit().await?;
    Ok(())
}
