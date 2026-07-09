// GNU AFFERO GENERAL PUBLIC LICENSE
// Version 3, 19 November 2007
//
// Copyright (C) 2026 Mathew Aloisio
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published
// by the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

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
