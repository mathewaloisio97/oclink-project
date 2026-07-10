//! Identity microservice runtime entry point.
//!
//! Handles infrastructure initialization, executes database schema migrations,
//! constructs the persistence layer, and starts the gRPC network listener.

use oclink_contracts::identity::v1::identity_service_server::IdentityServiceServer;
use oclink_identity::repository::PostgresUserRepository;
use oclink_identity::OcLinkIdentity;
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;
use std::sync::Arc;
use tonic::transport::Server;
use tracing::info;

/// Entry point for the Identity microservice.
/// Initializes telemetry, connects to PostgreSQL, runs schema migrations, and boots the gRPC server.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize standard structured log formatting via tracing.
    tracing_subscriber::fmt::init();

    // Fall back to localized docker-compose infrastructure targets if environment is unconfigured.
    let db_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
        "postgres://postgres:oclink_dev_pass@localhost:5432/oclink_identity".to_string()
    });

    // Establish a bounded connection pool to mitigate backend socket exhaustion.
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await?;

    // Guarantee schema state alignment at runtime before accepting traffic.
    info!("Applying Database Migrations...");
    sqlx::migrate!("./migrations").run(&pool).await?;
    info!("Identity DB Migrations Applied Successfully");

    // Assemble the dependency injection graph.
    let repo = Arc::new(PostgresUserRepository::new(pool));
    let service = OcLinkIdentity::new(repo);

    // Bind to all local interfaces to ensure visibility within bridge networks and containers.
    let addr: SocketAddr = "0.0.0.0:50051".parse().unwrap();
    info!("Identity gRPC Service listening on {}", addr);

    // Start the gRPC runtime listener execution loop.
    Server::builder()
        .add_service(IdentityServiceServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}
