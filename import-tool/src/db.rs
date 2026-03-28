use sqlx::postgres::PgPool;
use anyhow::Result;
use crate::excel::ImportPub;
use uuid::Uuid;

pub async fn insert_pub(pool: &PgPool, pub_data: &ImportPub) -> Result<Uuid> {
    let pub_id: Uuid = if let Some(existing_id) = pub_data.id {
        sqlx::query_scalar!(
            r#"UPDATE pubs SET 
                  name = $1, address = $2, town = $3, region = $4, country_code = $5, postcode = $6, 
                  closed = $7, 
                  location = CASE WHEN $8::float8 IS NOT NULL AND $9::float8 IS NOT NULL 
                             THEN ST_SetSRID(ST_MakePoint($9, $8), 4326)::geography 
                             ELSE location END,
                  untappd_id = COALESCE($11, untappd_id),
                  untappd_verified = CASE WHEN $12 THEN true ELSE untappd_verified END
               WHERE id = $10
               RETURNING id"#,
            pub_data.name,
            pub_data.address,
            pub_data.town,
            pub_data.region,
            pub_data.country_code,
            pub_data.postcode,
            pub_data.closed,
            pub_data.lat,
            pub_data.lon,
            existing_id,
            pub_data.untappd_id,
            pub_data.untappd_verified
        )
        .fetch_one(pool)
        .await?
    } else {
        sqlx::query_scalar!(
            r#"INSERT INTO pubs (name, address, town, region, country_code, postcode, closed, location, untappd_id, untappd_verified)
               VALUES ($1, $2, $3, $4, $5, $6, $7, 
                       CASE WHEN $8::float8 IS NOT NULL AND $9::float8 IS NOT NULL 
                            THEN ST_SetSRID(ST_MakePoint($9, $8), 4326)::geography 
                            ELSE NULL END,
                       $10, $11)
               ON CONFLICT (name, town, postcode) 
               DO UPDATE SET 
                  address = EXCLUDED.address,
                  region = EXCLUDED.region,
                  country_code = EXCLUDED.country_code,
                  closed = EXCLUDED.closed,
                  location = COALESCE(EXCLUDED.location, pubs.location),
                  untappd_id = COALESCE(EXCLUDED.untappd_id, pubs.untappd_id),
                  untappd_verified = CASE WHEN EXCLUDED.untappd_verified THEN true ELSE pubs.untappd_verified END
               RETURNING id"#,
            pub_data.name,
            pub_data.address,
            pub_data.town,
            pub_data.region,
            pub_data.country_code,
            pub_data.postcode,
            pub_data.closed,
            pub_data.lat,
            pub_data.lon,
            pub_data.untappd_id,
            pub_data.untappd_verified
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

pub async fn update_untappd_details(pool: &PgPool, pub_id: Uuid, untappd_id: &str, verified: bool) -> Result<()> {
    sqlx::query!(
        "UPDATE pubs SET untappd_id = $1, untappd_verified = $2 WHERE id = $3",
        untappd_id,
        verified,
        pub_id
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn refresh_pub_stats(pool: &PgPool) -> Result<()> {
    sqlx::query!("REFRESH MATERIALIZED VIEW pub_stats")
        .execute(pool)
        .await?;
    Ok(())
}
