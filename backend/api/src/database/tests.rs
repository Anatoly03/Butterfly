//! Database tests.

use super::{init, migrate};

/// This is a pair of two tests, the other is called [migrations]. This test verifies
/// that a newly initialized database does not have any migrations.
#[tokio::test]
pub async fn no_migrations() {
    let pool = init().await.unwrap();
    let result: Result<(i64,), sqlx::Error> =
        sqlx::query_as("SELECT COUNT(*) FROM _sqlx_migrations")
            .fetch_one(&pool)
            .await;

    // verify "_sqlx_migrations" does not exist
    assert!(result.is_err(), "`_sqlx_migrations` table should not exist");
}

/// This is a pair of two tests, the other is called [no_migrations]. This test verifies
/// that after running migrations, there is no error and the total migration count is
/// greater 1. (`create_tickets` migration exists)
#[tokio::test]
pub async fn migrations() {
    let pool = init().await.unwrap();
    migrate(&pool).await.unwrap();
    let result: Result<(i64,), sqlx::Error> =
        sqlx::query_as("SELECT COUNT(*) FROM _sqlx_migrations")
            .fetch_one(&pool)
            .await;

    // verify "_sqlx_migrations" exists
    assert!(result.is_ok(), "`_sqlx_migrations` table should be created");
    assert!(result.unwrap().0 > 0, "there should be >= 1 migrations");
}
