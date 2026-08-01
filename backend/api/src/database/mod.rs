//! The database wrapper of the Butterfly backend application. This module provides
//! communication with the [sqlx] interface.

use sqlx::{Pool, Sqlite, SqlitePool, migrate::MigrateError, sqlite::SqliteConnectOptions};
use std::{println, str::FromStr};

/// Initializes an asynchronous [Pool] to [Sqlite]
pub async fn init() -> Result<Pool<Sqlite>, sqlx::Error> {
    let options = SqliteConnectOptions::from_str("sqlite://.butterfly/data.db")
        .unwrap()
        .create_if_missing(true);
    SqlitePool::connect_with(options).await
}

/// Run migrations on the main database.
pub async fn migrate(pool: &Pool<Sqlite>) -> Result<(), MigrateError> {
    sqlx::migrate!("../migrations").run(pool).await?;

    // TODO only print migrations which have been "currently" migrated by writing a custom migrator
    for migration in sqlx::migrate!("../migrations").iter() {
        println!("Migrate {} `{}`", migration.version, migration.description);
    }

    Ok(())
}
