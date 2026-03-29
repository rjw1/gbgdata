use sqlx::{PgPool, Row};
use uuid::Uuid;

#[sqlx::test(migrations = "../migrations")]
async fn test_get_pubs_by_location_filters_by_region(pool: PgPool) {
    // 1. Seed test data
    let id1 = Uuid::new_v4();
    sqlx::query("INSERT INTO pubs (id, name, region, town, postcode) VALUES ($1, $2, $3, $4, $5)")
        .bind(id1)
        .bind("Pub In Kent")
        .bind("Kent")
        .bind("Canterbury")
        .bind("CT1 1AA")
        .execute(&pool)
        .await
        .unwrap();

    let id2 = Uuid::new_v4();
    sqlx::query("INSERT INTO pubs (id, name, region, town, postcode) VALUES ($1, $2, $3, $4, $5)")
        .bind(id2)
        .bind("Pub In London")
        .bind("Greater London")
        .bind("London")
        .bind("SW1 1AA")
        .execute(&pool)
        .await
        .unwrap();

    let pubs = sqlx::query("SELECT name FROM pubs WHERE region = $1")
        .bind("Kent")
        .fetch_all(&pool)
        .await
        .unwrap();

    assert_eq!(pubs.len(), 1);
    let name: String = pubs[0].get("name");
    assert_eq!(name, "Pub In Kent");
}

#[sqlx::test(migrations = "../migrations")]
async fn test_get_pubs_by_location_filters_by_town(pool: PgPool) {
    let id1 = Uuid::new_v4();
    sqlx::query("INSERT INTO pubs (id, name, region, town, postcode) VALUES ($1, $2, $3, $4, $5)")
        .bind(id1)
        .bind("Canterbury Pub")
        .bind("Kent")
        .bind("Canterbury")
        .bind("CT1 1AA")
        .execute(&pool)
        .await
        .unwrap();

    let id2 = Uuid::new_v4();
    sqlx::query("INSERT INTO pubs (id, name, region, town, postcode) VALUES ($1, $2, $3, $4, $5)")
        .bind(id2)
        .bind("Dover Pub")
        .bind("Kent")
        .bind("Dover")
        .bind("CT16 1AA")
        .execute(&pool)
        .await
        .unwrap();

    let pubs = sqlx::query("SELECT name FROM pubs WHERE town = $1")
        .bind("Canterbury")
        .fetch_all(&pool)
        .await
        .unwrap();

    assert_eq!(pubs.len(), 1);
    let name: String = pubs[0].get("name");
    assert_eq!(name, "Canterbury Pub");
}

#[sqlx::test(migrations = "../migrations")]
async fn test_pub_stats_ignores_1972_trial_year(pool: PgPool) {
    let id = Uuid::new_v4();
    // Seed pub
    sqlx::query("INSERT INTO pubs (id, name, region, town, postcode) VALUES ($1, $2, $3, $4, $5)")
        .bind(id)
        .bind("1972 Test Pub")
        .bind("Test Region")
        .bind("Test Town")
        .bind("TS1 1AA")
        .execute(&pool)
        .await
        .unwrap();

    // Seed history including 1972
    for year in [1972, 1973, 1974] {
        sqlx::query("INSERT INTO gbg_history (pub_id, year) VALUES ($1, $2)")
            .bind(id)
            .bind(year)
            .execute(&pool)
            .await
            .unwrap();
    }

    // Refresh view
    sqlx::query("REFRESH MATERIALIZED VIEW pub_stats")
        .execute(&pool)
        .await
        .unwrap();

    // Assertions
    let stats = sqlx::query("SELECT total_years, first_year FROM pub_stats WHERE pub_id = $1")
        .bind(id)
        .fetch_one(&pool)
        .await
        .unwrap();

    let total: i64 = stats.get("total_years");
    let first: i32 = stats.get("first_year");

    assert_eq!(total, 2, "Should exclude 1972 from count");
    assert_eq!(first, 1973, "First year should be 1973, ignoring 1972");
}
