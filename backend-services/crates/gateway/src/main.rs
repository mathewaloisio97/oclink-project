//! Gateway API main server entry point.
//!
//! Sets up our central web server by connecting to backend services,
//! creating our shared application state, configuring interactive API
//! documentation (Swagger UI), and starting the network listener.

use axum::Router;
use oclink_constants::security::DEFAULT_HV_SECRET;
use oclink_contracts::auth::v1::auth_service_client::AuthServiceClient;
use oclink_contracts::identity::v1::identity_service_client::IdentityServiceClient;
use oclink_gateway::{dtos, handlers, routes, state::AppState};
use std::env;
use std::net::SocketAddr;
use tonic::transport::Channel;
use tracing::info;
use utoipa::{
    openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme},
    Modify, OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

/// Tells Swagger UI how to display and accept our Bearer token login security format.
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("Opaque")
                    .build(),
            ),
        );
    }
}

/// Builds our interactive API specification docs from existing code paths and payload structures.
#[derive(OpenApi)]
#[openapi(
    paths(
        handlers::identity::register,
        handlers::identity::login,
        handlers::auth::logout,
        handlers::human_verification::get_challenge,
        handlers::human_verification::verify
    ),
    components(schemas(
        dtos::RegisterPayload,
        dtos::RegisterResponse,
        dtos::LoginPayload,
        dtos::LoginResponse,
        dtos::ClientVerifyPayload
    )),
    modifiers(&SecurityAddon),
    tags((name = "OcLink Edge Gateway", description = "OcLink Unified REST API"))
)]
struct ApiDoc;

/// Starts the web server loop.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start logging system to print messages to the console terminal.
    tracing_subscriber::fmt::init();

    // Ingest configuration from the environment, falling back to local dev defaults.
    // This allows seamless integration with Docker Compose networking.
    let identity_url =
        env::var("IDENTITY_URL").unwrap_or_else(|_| "http://localhost:50051".to_string());
    let auth_url = env::var("AUTH_URL").unwrap_or_else(|_| "http://localhost:50052".to_string());
    let hv_url =
        env::var("HUMAN_VERIFICATION_URL").unwrap_or_else(|_| "http://localhost:50055".to_string());
    let server_addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());
    let hv_secret =
        env::var("HUMAN_VERIFICATION_SECRET").unwrap_or_else(|_| DEFAULT_HV_SECRET.to_string());

    // Verify the integrity of the cryptographic validation keys. Allowing fallback
    // credentials is restricted to builds explicitly compiling with the "local-dev" feature.
    if hv_secret == DEFAULT_HV_SECRET {
        if cfg!(feature = "local-dev") {
            // Safe for local builds (debug or release) explicitly authorized by the feature flag.
            tracing::warn!("===========================================================");
            tracing::warn!("SECURITY ALERT: Running with the default development secret");
            tracing::warn!("===========================================================");
        } else {
            // Hard crash on any build profile that lacks explicit local-dev authorization.
            tracing::error!(
                "FATAL: Insecure fallback secret detected without local-dev authorization!"
            );
            panic!("Insecure configuration payload blocked.");
        }
    }

    // Establish a lazy multiplexed channel to the Identity service.
    info!("Connecting to Identity Subsystem at {}...", identity_url);
    let identity_channel = Channel::from_shared(identity_url)?.connect_lazy();

    // Establish a lazy multiplexed channel to the Auth service.
    info!("Connecting to Auth Subsystem at {}...", auth_url);
    let auth_channel = Channel::from_shared(auth_url)?.connect_lazy();

    // Establish a lazy multiplexed channel to the Human Verification service.
    info!(
        "Connecting to Human Verification Subsystem at {}...",
        hv_url
    );
    let hv_channel = Channel::from_shared(hv_url)?.connect_lazy();

    // Initialize the stateless cryptography engine for validating Captcha vouchers at the edge.
    let crypto_engine = oclink_human_verification_crypto::CryptoEngine::new(hv_secret.as_bytes());

    // Package clients into a single state structure to share with web route handlers.
    let state = AppState {
        identity_client: IdentityServiceClient::new(identity_channel),
        auth_client: AuthServiceClient::new(auth_channel),
        human_verification_client: oclink_contracts::human_verification::v1::human_verification_service_client::HumanVerificationServiceClient::new(hv_channel),
        crypto_engine,
    };

    let openapi = ApiDoc::openapi();

    // Combine individual endpoint blocks and our interactive Swagger documentation pages.
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
        .merge(routes::identity::build_router(state.clone()))
        .merge(routes::auth::build_router(state.clone()))
        .merge(routes::human_verification::build_router(state.clone()));

    // Open network port listener and begin serving client traffic.
    let addr: SocketAddr = server_addr.parse()?;
    info!("Gateway REST API online at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
