//! HTTP routing layer for the human verification subsystem.
//!
//! Provides the public Axum handler entry points to request new bot protection
//! challenges and verify corresponding client telemetry payloads.

use crate::{
    dtos::{ChallengeQuery, ClientVerifyPayload},
    error::handle_grpc_error,
    state::AppState,
};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use oclink_contracts::human_verification::v1::{GetChallengeRequest, VerifyRequest};
use serde_json::json;

/// Fetches a new challenge payload from the backend verification subsystem.
///
/// Maps incoming HTTP GET query attributes into a gRPC request, signaling the
/// designated provider to assemble an isolated, unique frontend widget configuration.
#[utoipa::path(
    get,
    path = "/api/v1/captcha/request",
    params(ChallengeQuery),
    responses(
        (status = 200, description = "Challenge payload generated")
    )
)]
#[tracing::instrument(skip(state))]
pub async fn get_challenge(
    State(mut state): State<AppState>,
    Query(query): Query<ChallengeQuery>,
) -> impl IntoResponse {
    let req = tonic::Request::new(GetChallengeRequest {
        provider_id: query.provider_id,
        edition_id: query.edition_id.unwrap_or_default(),
    });

    match state.human_verification_client.get_challenge(req).await {
        Ok(res) => {
            let inner = res.into_inner();
            let challenge_json: serde_json::Value =
                serde_json::from_str(&inner.challenge_payload_json).unwrap_or_else(|_| json!({}));
            (StatusCode::OK, Json(challenge_json)).into_response()
        }
        Err(err) => handle_grpc_error(err),
    }
}

/// Submits telemetry inputs to determine if the client is human.
///
/// Serializes provider-specific client behavior properties into raw JSON, routing
/// the payload down to the verification coordinator to evaluate heuristics and issue
/// an authenticated downstream routing pass token upon success.
#[utoipa::path(
    post,
    path = "/api/v1/captcha/verify",
    request_body = ClientVerifyPayload,
    responses(
        (status = 200, description = "Verification passed, voucher issued"),
        (status = 401, description = "Verification failed (Bot detected)")
    )
)]
#[tracing::instrument(skip(state, payload))]
pub async fn verify(
    State(mut state): State<AppState>,
    Json(payload): Json<ClientVerifyPayload>,
) -> impl IntoResponse {
    let payload_json_str = serde_json::to_string(&payload.payload).unwrap_or_default();

    let req = tonic::Request::new(VerifyRequest {
        provider_id: payload.provider_id,
        payload_json: payload_json_str,
    });

    match state.human_verification_client.verify(req).await {
        Ok(res) => {
            let inner = res.into_inner();
            if inner.success {
                (
                    StatusCode::OK,
                    Json(json!({ "captcha_voucher": inner.voucher })),
                )
                    .into_response()
            } else {
                (
                    StatusCode::UNAUTHORIZED,
                    Json(json!({ "error": "Human verification failed. Automated behavior detected." })),
                ).into_response()
            }
        }
        Err(err) => handle_grpc_error(err),
    }
}
