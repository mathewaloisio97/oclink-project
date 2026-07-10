//! Data Transfer Objects (DTOs) for the Edge Gateway.
//!
//! This module defines the public-facing JSON schemas and validation contracts
//! for incoming HTTP requests and outgoing HTTP responses. These types are
//! decoupled from internal gRPC schemas to allow safe API versioning.

use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Request payload containing credentials for user registration.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct RegisterPayload {
    #[schema(example = "example_username")]
    pub username: String,
    #[schema(example = "example_password")]
    pub password: String,
}

/// Response payload containing the newly registered user's Identity UUID.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct RegisterResponse {
    #[schema(example = "0191701c-da51-749a-a4fc-e96579ebc043")]
    pub user_id: String,
}

/// Request payload containing credentials for user authentication.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginPayload {
    #[schema(example = "dev_user")]
    pub username: String,
    #[schema(example = "example_password")]
    pub password: String,
}

/// Response payload containing the generated session credentials.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginResponse {
    #[schema(example = "opaque_session_token_xyz")]
    pub token: String,
}
