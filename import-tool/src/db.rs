use sqlx::postgres::PgPool;
use anyhow::Result;
use crate::excel::ImportPub;
use uuid::Uuid;

pub async fn insert_pub(pool: &PgPool, pub_data: &ImportPub) -> Result<Uuid> {
    let pub_id: Uuid = if let Some(existing_id) = pub_data.id {
        sqlx::query_scalar!(
            r#"UPDATE pubs SET 
                  name = $1, address = $2, town = $3, county = $4, postcode = $5, 
                  closed = $6, 
                  location = CASE WHEN $7::float8 IS NOT NULL AND $8::float8 IS NOT NULL 
                             THEN ST_SetSRID(ST_MakePoint($8, $7), 4326)::geography 
                             ELSE location END
               WHERE id = $9
               RETURNING id"#,
            pub_data.name,
            pub_data.address,
            pub_data.town,
            pub_data.county,
            pub_data.postcode,
            pub_data.closed,
            pub_data.lat,
            pub_data.lon,
            existing_id
        )
        .fetch_one(pool)
        .await?
    } else {
        sqlx::query_scalar!(
            r#"INSERT INTO pubs (name, address, town, county, postcode, closed, location)
               VALUES ($1, $2, $3, $4, $5, $6, 
                       CASE WHEN $7::float8 IS NOT NULL AND $8::float8 IS NOT NULL 
                            THEN ST_SetSRID(ST_MakePoint($8, $7), 4326)::geography 
                            ELSE NULL END)
               ON CONFLICT (name, town, postcode) 
               DO UPDATE SET 
                  address = EXCLUDED.address,
                  county = EXCLUDED.county,
                  closed = EXCLUDED.closed,
                  location = COALESCE(EXCLUDED.location, pubs.location)
               RETURNING id"#,
            pub_data.name,
            pub_data.address,
            pub_data.town,
            pub_data.county,
            pub_data.postcode,
            pub_data.closed,
            pub_data.lat,
            pub_data.lon
        )
        .fetch_one(pool)
        .await?
    };

    for year in &pub_data.years {
        sqlx::query!(
            "INSERT INTO gbg_history (pub_id, year) VALUES ($1, $2) ON CONFLICT DO NOTHING",
            pub_id,
            *year
        )
        .execute(pool)
        .await?;
    }

    Ok(pub_id)
}

pub async fn update_pub_location(pool: &PgPool, pub_id: Uuid, lat: f64, lon: f64) -> Result<()> {
    sqlx::query!(
        "UPDATE pubs SET location = ST_SetSRID(ST_MakePoint($2, $1), 4326)::geography WHERE id = $3",
        lat,
        lon,
        pub_id
    )
    .execute(pool)
    .await?;
    Ok(())
}
