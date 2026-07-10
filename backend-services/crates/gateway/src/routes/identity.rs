//! Gateway API route registration.
//!
//! Maps incoming web browser and client requests to the correct backend
//! logic code and connects our shared application state to those handlers.

use crate::{handlers::identity, state::AppState};
use axum::{routing::post, Router};

/// Creates the main API router and registers all available web endpoints.
pub fn build_router(state: AppState) -> Router {
    Router::new()
        // Accepts registration data to create new user accounts.
        .route("/api/v1/register", post(identity::register))
        // Accepts user credentials to verify and log players in.
        .route("/api/v1/login", post(identity::login))
        // Attaches the shared database and gRPC clients to all routes above.
        .with_state(state)
}
