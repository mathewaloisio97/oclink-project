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

impl RegisterPayload {
    pub fn is_valid(&self) -> bool {
        !self.username.trim().is_empty() && !self.password.trim().is_empty()
    }
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

impl LoginPayload {
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

/// Query parameters requested by the HTTP API layout to fetch a challenge.
#[derive(serde::Deserialize, utoipa::IntoParams, Debug)]
pub struct ChallengeQuery {
    /// The unique identifier of the target bot protection provider.
    pub provider_id: String,
    /// An optional specific iteration or layout variant of the challenge.
    pub edition_id: Option<String>,
}

/// The inbound verification submission body transmitted by the frontend client.
#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct ClientVerifyPayload {
    /// The unique identifier of the bot protection provider handling the check.
    #[schema(example = "arrow_alignment")]
    pub provider_id: String,
    /// The raw, provider-specific telemetry and token data token string.
    pub payload: serde_json::Value,
}
