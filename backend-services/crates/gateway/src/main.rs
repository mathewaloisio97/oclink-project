// src/main.rs

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

//! OcLink Edge Gateway service entry point.
//!
//! Provides the primary stateless API routing layer. This service bridges external
//! HTTP/REST clients to internal gRPC microservices, enforcing edge validations,
//! rate limiting, and generating OpenAPI documentation via Utoipa.

use axum::{http::StatusCode, response::IntoResponse, routing::post, Json, Router};
use oclink_contracts::identity::v1::identity_service_client::IdentityServiceClient;
use serde_json::json;
use std::env;
use std::net::SocketAddr;
use tonic::transport::Channel;
use tracing::{error, info};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod dtos;
mod handlers;

// ============================================================================
// Application State & Helpers
// ============================================================================

/// Maintains multiplexed, resilient gRPC channels to downstream microservices.
#[derive(Clone)]
pub struct AppState {
    pub identity_client: IdentityServiceClient<Channel>,
}

/// Translates internal gRPC Status codes into semantic HTTP responses with JSON bodies.
pub fn handle_grpc_error(err: tonic::Status) -> axum::response::Response {
    let (status, message) = match err.code() {
        tonic::Code::AlreadyExists => (StatusCode::CONFLICT, err.message().to_string()),
        tonic::Code::InvalidArgument => (StatusCode::BAD_REQUEST, err.message().to_string()),
        _ => {
            error!("Upstream gRPC fault: {:?}", err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            )
        }
    };

    (status, Json(json!({ "error": message }))).into_response()
}

// ============================================================================
// OpenAPI Configuration & Bootstrapper
// ============================================================================

/// OpenAPI documentation schema compiler.
#[derive(OpenApi)]
#[openapi(
    paths(handlers::identity::register, handlers::identity::login),
    components(schemas(
        dtos::RegisterPayload,
        dtos::RegisterResponse,
        dtos::LoginPayload,
        dtos::LoginResponse
    )),
    tags((name = "OcLink Edge Gateway", description = "OcLink Unified REST API"))
)]
struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Ingest configuration from the environment, falling back to local dev defaults.
    // This allows seamless integration with Docker Compose networking.
    let identity_url =
        env::var("IDENTITY_URL").unwrap_or_else(|_| "http://localhost:50051".to_string());
    let server_addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());

    info!("Connecting to Identity Subsystem at {}...", identity_url);

    // Establish a resilient, multiplexed HTTP/2 channel to the Identity service.
    let channel = Channel::from_shared(identity_url)?.connect_lazy();

    let state = AppState {
        identity_client: IdentityServiceClient::new(channel),
    };

    let openapi = ApiDoc::openapi();

    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
        .route("/api/v1/register", post(handlers::register))
        .route("/api/v1/login", post(handlers::login))
        .with_state(state);

    let addr: SocketAddr = server_addr.parse()?;
    info!("Gateway REST API online at http://{}", addr);
    info!(
        "Swagger UI interactive documentation available at http://{}/swagger-ui",
        addr
    );

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
