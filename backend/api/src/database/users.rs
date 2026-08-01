//! The users of the Butterfly backend application.
//!
//! This module defines the [Message] struct.

use butterfly_macros::collection;
use serde::{Deserialize, Serialize};

/// The user struct represents stored user data for a single Butterfly user.
#[derive(Deserialize, Serialize)]
#[collection(table = "users")]
#[allow(dead_code)]
pub struct User {
    pub username: String,
}
