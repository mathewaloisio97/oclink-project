//! gRPC server execution entrypoint for the Human Verification service.
//!
//! Handles challenges and solution validations (e.g., arrow alignment CAPTCHAs)
//! and issues signed crypto vouchers upon successful human verification.

use oclink_constants::security::DEFAULT_HV_SECRET;
use oclink_contracts::human_verification::v1::human_verification_service_server::HumanVerificationServiceServer;
use oclink_human_verification::grpc::VerificationGrpcServer;
use oclink_human_verification::{
    config, providers::arrow_alignment_provider::ArrowAlignmentProvider,
};
use oclink_human_verification_crypto::CryptoEngine;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tonic::transport::Server;
use tracing::info;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    // Use a hardcoded secret key for local development environments if none is configured.
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

    // Wire dependency tree.
    let config = Arc::new(config::VerificationConfig::from_env());
    let crypto = Arc::new(CryptoEngine::new(hv_secret.as_bytes()));
    let grpc_service = VerificationGrpcServer::new(
        crypto.clone(),
        Arc::new(ArrowAlignmentProvider::new(config, crypto)),
    );

    // Initialize and run the gRPC server connection.
    let addr: SocketAddr = "0.0.0.0:50055".parse().unwrap();
    info!("Human Verification gRPC Service listening on {}", addr);

    Server::builder()
        .add_service(HumanVerificationServiceServer::new(grpc_service))
        .serve(addr)
        .await?;

    Ok(())
}
