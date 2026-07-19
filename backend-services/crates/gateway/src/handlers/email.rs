//! HTTP gateway routing handlers for user email resource management.
//!
//! Exposes the REST API surface to manage active email configurations, initiate
//! lifecycle state changes protected by captcha telemetry validation, and submit
//! challenge code verification attempts.

use crate::{
    dtos::{
        EmailStateResponse, SetEmailPayload, SetEmailResponse, VerifyEmailPayload,
        VerifyEmailResponse,
    },
    error::handle_grpc_error,
    middleware::UserId,
    state::AppState,
};
use axum::{
    extract::{Extension, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use oclink_contracts::email::v1::{GetEmailRequest, SetEmailRequest, VerifyEmailRequest};
use serde_json::json;

#[utoipa::path(
    get,
    path = "/api/v1/email",
    security(("bearer_auth" = [])),
    responses(
        (status = 200, description = "Current email state returned", body = EmailStateResponse),
        (status = 401, description = "Unauthorized - Missing or invalid token")
    )
)]
#[tracing::instrument(skip(state))]
pub async fn get_email(
    State(mut state): State<AppState>,
    Extension(user_id): Extension<UserId>,
) -> impl IntoResponse {
    let req = tonic::Request::new(GetEmailRequest { user_id: user_id.0 });

    // Forward configuration request to the underlying gRPC microservice.
    match state.email_client.get_email(req).await {
        Ok(res) => {
            let inner = res.into_inner();
            (
                StatusCode::OK,
                Json(EmailStateResponse {
                    current_email: inner.current_email,
                    is_verified: inner.is_verified,
                    pending_new_email: inner.pending_new_email,
                    verification_type: inner.verification_type,
                }),
            )
                .into_response()
        }
        Err(err) => handle_grpc_error(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/email",
    security(("bearer_auth" = [])),
    request_body = SetEmailPayload,
    params(
        ("x-captcha-voucher" = String, Header, description = "Cryptographic proof-of-humanity voucher")
    ),
    responses(
        (status = 200, description = "Email transition initiated", body = SetEmailResponse),
        (status = 400, description = "Invalid payload (e.g., malformed email)"),
        (status = 401, description = "Unauthorized - Missing or invalid token"),
        (status = 403, description = "Missing or invalid captcha voucher")
    )
)]
#[tracing::instrument(skip(state, payload))]
pub async fn set_email(
    State(mut state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Json(payload): Json<SetEmailPayload>,
) -> impl IntoResponse {
    if !payload.is_valid() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Invalid email format"})),
        )
            .into_response();
    }

    let req = tonic::Request::new(SetEmailRequest {
        user_id: user_id.0,
        new_email: payload.email,
    });

    // Request an email mapping overwrite or stage a multi-step verification change.
    match state.email_client.set_email(req).await {
        Ok(res) => (
            StatusCode::OK,
            Json(SetEmailResponse {
                status: res.into_inner().status,
            }),
        )
            .into_response(),
        Err(err) => handle_grpc_error(err),
    }
}

#[utoipa::path(
    post,
    path = "/api/v1/email/verify",
    security(("bearer_auth" = [])),
    request_body = VerifyEmailPayload,
    responses(
        (status = 200, description = "Verification attempt evaluated", body = VerifyEmailResponse),
        (status = 400, description = "Invalid code payload or expired code"),
        (status = 401, description = "Unauthorized - Missing or invalid token")
    )
)]
#[tracing::instrument(skip(state, payload))]
pub async fn verify_email(
    State(mut state): State<AppState>,
    Extension(user_id): Extension<UserId>,
    Json(payload): Json<VerifyEmailPayload>,
) -> impl IntoResponse {
    if !payload.is_valid() {
        return (
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Verification code cannot be empty"})),
        )
            .into_response();
    }

    let req = tonic::Request::new(VerifyEmailRequest {
        user_id: user_id.0,
        code: payload.code,
    });

    // Evaluate the matching challenge token against the current state machine criteria.
    match state.email_client.verify_email(req).await {
        Ok(res) => {
            let inner = res.into_inner();
            if inner.success {
                (StatusCode::OK, Json(VerifyEmailResponse { success: true })).into_response()
            } else {
                (
                    StatusCode::BAD_REQUEST,
                    Json(
                        json!({"error": "Invalid or expired verification code", "success": false}),
                    ),
                )
                    .into_response()
            }
        }
        Err(err) => handle_grpc_error(err),
    }
}
