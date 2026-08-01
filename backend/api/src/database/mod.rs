//! The database wrapper of the Butterfly backend application. This module provides
//! communication with the [sqlx] interface.

#[cfg(test)]
mod tests;

use sqlx::{
    Pool, Sqlite, SqlitePool,
    migrate::MigrateError,
    sqlite::{SqliteConnectOptions, SqliteJournalMode},
};
use std::{println, str::FromStr};

/// Initializes an asynchronous [Pool] to the main [Sqlite] database.
///
/// **Note.** In testing environment `#[cfg(test)]` this overwrites the Sqlite URL to be
/// in-memory to ensure that every test gets its' own context.
pub async fn init() -> Result<Pool<Sqlite>, sqlx::Error> {
    #[cfg(not(test))]
    const SQLITE_URL: &str = "sqlite://.butterfly/data.db";
    #[cfg(test)]
    const SQLITE_URL: &str = "sqlite::memory:";

    let mut options = SqliteConnectOptions::from_str(SQLITE_URL).unwrap();

    if dotenvy::var("DATABASE_CREATE_MISSING").is_ok_and(|k| k != "0" && k != "false") {
        options = options.create_if_missing(true);
    }

    if let Ok(journal_mode_str) = dotenvy::var("DATABASE_JOURNAL_MODE") {
        let mode = journal_mode_str.parse::<SqliteJournalMode>().unwrap();
        options = options.journal_mode(mode)
    }

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
