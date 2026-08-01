//! The users of the Butterfly backend application.
//!
//! This module defines the [Message] struct.

use butterfly_macros::collection;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// The user struct represents stored user data for a single Butterfly user.
#[derive(Deserialize, Serialize)]
#[collection(table = "users")]
#[allow(dead_code)]
pub struct User {
    /// The univesally unique ID of the user.
    #[primary]
    pub id: Uuid,

    /// The displayed username of the user.
    pub username: String,
}

#[cfg(test)]
mod tests {
    use std::assert_eq;

    use super::User;
    // use crate::database;
    // use uuid::Uuid;

    #[test]
    fn users_table_name() {
        assert_eq!(User::TABLE_NAME, "users");
    }

    /*#[tokio::test]
    async fn saving_users_works() {
        let pool: sqlx::Pool<sqlx::Sqlite> = database::init().await.unwrap();
        database::migrate(&pool).await.unwrap();

        let user1 = User {
            id: Uuid::now_v7(),
            username: "User1".to_string(),
        };
        user1.save(&pool).await;

        todo!("test")
    }*/
}
