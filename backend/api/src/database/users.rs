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
