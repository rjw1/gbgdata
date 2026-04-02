use crate::server::get_pubs_by_location_db;
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

    let pubs = get_pubs_by_location_db(&pool, "Kent".to_string(), None, None, None, None, None)
        .await
        .unwrap();

    assert_eq!(pubs.len(), 1);
    assert_eq!(pubs[0].name, "Pub In Kent");
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

    let pubs = get_pubs_by_location_db(
        &pool,
        "Kent".to_string(),
        Some("Canterbury".to_string()),
        None,
        None,
        None,
        None,
    )
    .await
    .unwrap();

    assert_eq!(pubs.len(), 1);
    assert_eq!(pubs[0].name, "Canterbury Pub");
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

#[sqlx::test(migrations = "../migrations")]
async fn test_ranking_competition_sql(pool: PgPool) {
    // 1. Seed pubs
    for i in 1..=4 {
        let id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO pubs (id, name, region, town, postcode) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(id)
        .bind(format!("Pub {}", i))
        .bind("Test Region")
        .bind("Test Town")
        .bind(format!("TS1 1A{}", i))
        .execute(&pool)
        .await
        .unwrap();

        // Pub 1 & 2: 3 years (Rank 1)
        // Pub 3: 2 years (Rank 3)
        // Pub 4: 1 year (Rank 4)
        let years = if i <= 2 {
            vec![2024, 2025, 2026]
        } else if i == 3 {
            vec![2025, 2026]
        } else {
            vec![2026]
        };
        for year in years {
            sqlx::query("INSERT INTO gbg_history (pub_id, year) VALUES ($1, $2)")
                .bind(id)
                .bind(year)
                .execute(&pool)
                .await
                .unwrap();
        }
    }

    // Refresh view
    sqlx::query("REFRESH MATERIALIZED VIEW pub_stats")
        .execute(&pool)
        .await
        .unwrap();

    // Assertions for Entry Rank (Competition 1224)
    let ranks: Vec<(String, i64, i64)> = sqlx::query_as::<_, (String, i64, i64)>(
        "SELECT p.name, s.total_years, s.entries_rank FROM pubs p JOIN pub_stats s ON p.id = s.pub_id ORDER BY s.entries_rank ASC, p.name ASC"
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    assert_eq!(ranks[0].2, 1, "Pub 1 should be Rank 1");
    assert_eq!(ranks[1].2, 1, "Pub 2 should be Rank 1 (Tie)");
    assert_eq!(ranks[2].2, 3, "Pub 3 should be Rank 3 (Skip 2)");
    assert_eq!(ranks[3].2, 4, "Pub 4 should be Rank 4");
}

#[sqlx::test(migrations = "../migrations")]
async fn test_streak_ranking_competition_sql(pool: PgPool) {
    // 1. Seed pubs
    for i in 1..=4 {
        let id = Uuid::new_v4();
        sqlx::query(
            "INSERT INTO pubs (id, name, region, town, postcode) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(id)
        .bind(format!("Pub {}", i))
        .bind("Test Region")
        .bind("Test Town")
        .bind(format!("TS1 1B{}", i))
        .execute(&pool)
        .await
        .unwrap();

        // Base year is 2026
        // Pub 1 & 2: 3 year streak (2026, 2025, 2024) -> Rank 1
        // Pub 3: 2 year streak (2026, 2025) -> Rank 3
        // Pub 4: 1 year streak (2026) -> Rank 4
        let years = if i <= 2 {
            vec![2026, 2025, 2024]
        } else if i == 3 {
            vec![2026, 2025]
        } else {
            vec![2026]
        };
        for year in years {
            sqlx::query("INSERT INTO gbg_history (pub_id, year) VALUES ($1, $2)")
                .bind(id)
                .bind(year)
                .execute(&pool)
                .await
                .unwrap();
        }
    }

    // Refresh view
    sqlx::query("REFRESH MATERIALIZED VIEW pub_stats")
        .execute(&pool)
        .await
        .unwrap();

    // Assertions for Streak Rank
    let ranks: Vec<(String, i32, i64)> = sqlx::query_as::<_, (String, i32, i64)>(
        "SELECT p.name, s.current_streak, s.streak_rank FROM pubs p JOIN pub_stats s ON p.id = s.pub_id ORDER BY s.streak_rank ASC, p.name ASC"
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    assert_eq!(ranks[0].2, 1, "Pub 1 should be Rank 1");
    assert_eq!(ranks[1].2, 1, "Pub 2 should be Rank 1 (Tie)");
    assert_eq!(ranks[2].2, 3, "Pub 3 should be Rank 3 (Skip 2)");
    assert_eq!(ranks[3].2, 4, "Pub 4 should be Rank 4");
}
