//! Data Transfer Objects (DTOs) for identity management workflows.
//!
//! Defines the public-facing JSON schemas and validation contracts for user
//! registration and authentication endpoints.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Request payload containing credentials for user registration.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct RegisterPayload {
    /// The requested unique identifier for the new account.
    #[schema(example = "example_username")]
    pub username: String,

    /// The plain text password secret to be securely hashed down-funnel.
    #[schema(example = "example_password")]
    pub password: String,
}

impl RegisterPayload {
    /// Ensures that neither the username nor the password strings are blank.
    pub fn is_valid(&self) -> bool {
        !self.username.trim().is_empty() && !self.password.trim().is_empty()
    }
}

/// Response payload containing the newly registered user's unique identity mapping.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct RegisterResponse {
    /// The canonical Identity UUIDv7 identifier assigned to the user.
    #[schema(example = "0191701c-da51-749a-a4fc-e96579ebc043")]
    pub user_id: String,
}

/// Request payload containing credentials for user authentication.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginPayload {
    /// The unique account identifier attempting authentication.
    #[schema(example = "dev_user")]
    pub username: String,

    /// The plain text password challenge to match against stored credentials.
    #[schema(example = "example_password")]
    pub password: String,
}

impl LoginPayload {
    /// Ensures that neither the username nor the password credentials are blank.
    pub fn is_valid(&self) -> bool {
        !self.username.trim().is_empty() && !self.password.trim().is_empty()
    }
}

/// Response payload containing the generated session credentials.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    #[schema(example = "opaque_session_token_xyz")]
    pub token: String,
}
