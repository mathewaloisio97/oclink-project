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
//! Provides the primary API routing layer, including request validation,
//! authentication handlers, and auto-generated OpenAPI documentation.

use axum::http::StatusCode;
use axum::{routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tracing::info;
use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

/// Request payload containing credentials for user authentication.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct AuthPayload {
    /// The unique account username.
    #[schema(example = "dev_user")]
    pub username: String,
    /// The plain-text account password.
    #[schema(example = "example_password")]
    pub password: String,
}

/// Response payload containing the generated session credentials.
#[derive(Serialize, Deserialize, ToSchema)]
pub struct AuthResponse {
    /// Cryptographically signed JWT token for authorizing subsequent requests.
    #[schema(example = "eyJhbGciOiJIUzI1NiIsInR5c...")]
    pub token: String,
}

/// Authenticate a user and return an access token.
#[utoipa::path(
    post,
    path = "/api/v1/login",
    request_body = AuthPayload,
    responses(
        (status = 200, description = "Successfully authenticated", body = AuthResponse),
        (status = 401, description = "Invalid credentials")
    )
)]
async fn login(Json(payload): Json<AuthPayload>) -> Result<Json<AuthResponse>, StatusCode> {
    // TODO: Implement actual password verification logic.
    // Early return an unauthorized error if an empty password is provided.
    if payload.password.is_empty() {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Generate a mock token bound to the provided username.
    Ok(Json(AuthResponse {
        token: format!("mock_token_for_{}", payload.username),
    }))
}

/// OpenAPI documentation schema compiler.
#[derive(OpenApi)]
#[openapi(
    paths(login),
    tags((name = "OcLink Edge Gateway", description = "OcLink Unified API"))
)]
struct ApiDoc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let openapi = ApiDoc::openapi();

    // Bind API endpoints and inject the OpenAPI/Swagger UI router.
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
        .route("/api/v1/login", post(login));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    info!("Gateway online. Swagger UI available at http://localhost:3000/swagger-ui");

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
