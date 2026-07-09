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

//! Identity Domain Route Handlers.
//!
//! This module implements the public HTTP endpoints for user onboarding and
//! authentication. It handles the mapping between inbound REST JSON payloads
//! and the internal gRPC contracts required by the Identity microservice.

use crate::dtos::{LoginPayload, LoginResponse, RegisterPayload, RegisterResponse};
use crate::{handle_grpc_error, AppState};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use oclink_contracts::identity::v1::{CreateUserRequest, VerifyCredentialsRequest};
use serde_json::json;

/// Register a new user account.
///
/// Forwards the credentials to the Identity subsystem to generate
/// a new UUIDv7 and securely hash the password.
#[utoipa::path(
    post,
    path = "/api/v1/register",
    request_body = RegisterPayload,
    responses(
        (status = 201, description = "User registered successfully", body = RegisterResponse),
        (status = 409, description = "Username already in use"),
        (status = 400, description = "Invalid payload (e.g., missing fields)")
    )
)]
#[tracing::instrument(skip(state, payload))]
pub async fn register(
    State(mut state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> impl IntoResponse {
    // Map the RegisterPayload DTO to the internal Protobuf
    // CreateUserRequest contract.
    let req = tonic::Request::new(CreateUserRequest {
        username: payload.username,
        password: payload.password,
    });

    match state.identity_client.create_user(req).await {
        Ok(res) => (
            StatusCode::CREATED,
            Json(RegisterResponse {
                user_id: res.into_inner().user_id,
            }),
        )
            .into_response(),
        Err(err) => handle_grpc_error(err),
    }
}

/// Authenticate a user and provision a session.
///
/// Verifies the provided plaintext password against the Identity subsystem's
/// stored Argon2id hash. Upon success, provisions a new stateful session.
#[utoipa::path(
    post,
    path = "/api/v1/login",
    request_body = LoginPayload,
    responses(
        (status = 200, description = "Successfully authenticated", body = LoginResponse),
        (status = 401, description = "Invalid credentials")
    )
)]
#[tracing::instrument(skip(state, payload))]
pub async fn login(
    State(mut state): State<AppState>,
    Json(payload): Json<LoginPayload>,
) -> impl IntoResponse {
    // Verify credentials against the Identity Subsystem.
    let verify_req = tonic::Request::new(VerifyCredentialsRequest {
        username: payload.username.clone(),
        password: payload.password,
    });

    let verify_res = match state.identity_client.verify_credentials(verify_req).await {
        Ok(res) => res.into_inner(),
        Err(err) => return handle_grpc_error(err),
    };

    // Reject if the Identity service denies the password.
    if !verify_res.valid {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({ "error": "Invalid username or password" })),
        )
            .into_response();
    }

    // TODO: Wire up to the Auth Service to mint a real opaque session token.
    // For now, we return a mock token so downstream clients can test the integration slice.
    let mock_session_token = format!("mock_token_for_{}", payload.username);

    (
        StatusCode::OK,
        Json(LoginResponse {
            token: mock_session_token,
        }),
    )
        .into_response()
}
