//! Identity Domain Route Handlers.
//!
//! This module implements the public HTTP endpoints for user onboarding and
//! authentication. It handles the mapping between inbound REST JSON payloads
//! and the internal gRPC contracts required by the Identity microservice.

use crate::dtos::{LoginPayload, LoginResponse, RegisterPayload, RegisterResponse};
use crate::{error::handle_grpc_error, state::AppState};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use oclink_contracts::auth::v1::CreateTokenRequest;
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

    // Request a secure session token from the Auth Subsystem.
    let auth_req = tonic::Request::new(CreateTokenRequest {
        user_id: verify_res.user_id,
    });

    let auth_res = match state.auth_client.create_token(auth_req).await {
        Ok(res) => res.into_inner(),
        Err(err) => return handle_grpc_error(err),
    };

    // Return the fully provisioned stateful token to the client.
    (
        StatusCode::OK,
        Json(LoginResponse {
            token: auth_res.token,
        }),
    )
        .into_response()
}
