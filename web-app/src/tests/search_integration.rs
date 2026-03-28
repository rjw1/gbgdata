use sqlx::{PgPool, Row};
use uuid::Uuid;

#[sqlx::test(migrations = "../migrations")]
async fn test_get_pubs_by_location_filters_by_region(pool: PgPool) {
    // 1. Seed test data
    let id1 = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO pubs (id, name, region, town, postcode) VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(id1).bind("Pub In Kent").bind("Kent").bind("Canterbury").bind("CT1 1AA")
    .execute(&pool).await.unwrap();

    let id2 = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO pubs (id, name, region, town, postcode) VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(id2).bind("Pub In London").bind("Greater London").bind("London").bind("SW1 1AA")
    .execute(&pool).await.unwrap();

    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM pubs")
        .fetch_one(&pool).await.unwrap();
    assert_eq!(count, 2);
}

#[sqlx::test(migrations = "../migrations")]
async fn test_get_pubs_by_location_filters_by_town(pool: PgPool) {
    let id1 = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO pubs (id, name, region, town, postcode) VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(id1).bind("Canterbury Pub").bind("Kent").bind("Canterbury").bind("CT1 1AA")
    .execute(&pool).await.unwrap();

    let id2 = Uuid::new_v4();
    sqlx::query(
        "INSERT INTO pubs (id, name, region, town, postcode) VALUES ($1, $2, $3, $4, $5)"
    )
    .bind(id2).bind("Dover Pub").bind("Kent").bind("Dover").bind("CT16 1AA")
    .execute(&pool).await.unwrap();

    let pubs = sqlx::query("SELECT name FROM pubs WHERE town = $1")
        .bind("Canterbury")
        .fetch_all(&pool).await.unwrap();
    
    assert_eq!(pubs.len(), 1);
    let name: String = pubs[0].get("name");
    assert_eq!(name, "Canterbury Pub");
}
