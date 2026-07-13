//! Gateway API route registration.
//!
//! Maps incoming web browser and client requests to the correct backend
//! logic code and connects our shared application state to those handlers.

use crate::{handlers::identity, middleware::captcha::captcha_middleware, state::AppState};
use axum::{middleware, routing::post, Router};

/// Creates the main API router and registers all available web endpoints.
pub fn build_router(state: AppState) -> Router {
    // Human-verification protected router.
    let _captcha_protected_register = Router::new()
        .route("/api/v1/register", post(identity::register))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            captcha_middleware,
        ));

    // Unverified router.
    Router::new()
        // Accepts user credentials to verify and log players in.
        .route("/api/v1/login", post(identity::login))
        // Attaches the shared database and gRPC clients to all routes above.
        .with_state(state)
}
