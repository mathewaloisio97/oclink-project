//! Gateway API main server entry point.
//!
//! Sets up our central web server by connecting to backend services,
//! creating our shared application state, configuring interactive API
//! documentation (Swagger UI), and starting the network listener.

use axum::Router;
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
        handlers::auth::logout
    ),
    components(schemas(
        dtos::RegisterPayload,
        dtos::RegisterResponse,
        dtos::LoginPayload,
        dtos::LoginResponse
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
    let server_addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());

    // Establish a lazy multiplexed channel to the Identity service.
    info!("Connecting to Identity Subsystem at {}...", identity_url);
    let identity_channel = Channel::from_shared(identity_url)?.connect_lazy();

    // Establish a lazy multiplexed channel to the Auth service.
    info!("Connecting to Auth Subsystem at {}...", auth_url);
    let auth_channel = Channel::from_shared(auth_url)?.connect_lazy();

    // Package clients into a single state structure to share with web route handlers.
    let state = AppState {
        identity_client: IdentityServiceClient::new(identity_channel),
        auth_client: AuthServiceClient::new(auth_channel),
    };

    let openapi = ApiDoc::openapi();

    // Combine individual endpoint blocks and our interactive Swagger documentation pages.
    let app = Router::new()
        .merge(SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", openapi))
        .merge(routes::identity::build_router(state.clone()))
        .merge(routes::auth::build_router(state.clone()));

    // Open network port listener and begin serving client traffic.
    let addr: SocketAddr = server_addr.parse()?;
    info!("Gateway REST API online at http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
