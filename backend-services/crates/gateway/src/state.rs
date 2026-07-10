//! Shared application state for the API Gateway.
//!
//! Stores downstream clients and network connections in a thread-safe,
//! immutable struct shared across all request handlers.

use oclink_contracts::auth::v1::auth_service_client::AuthServiceClient;
use oclink_contracts::identity::v1::identity_service_client::IdentityServiceClient;
use tonic::transport::Channel;

/// Shared application state containing cloned gRPC service clients.
///
/// Tonic channels are internally multiplexed and connection-pooled,
/// making this struct cheap to clone across concurrent gateway request handlers.
#[derive(Clone)]
pub struct AppState {
    /// Client for the downstream Identity microservice.
    pub identity_client: IdentityServiceClient<Channel>,

    /// Client for the downstream Authentication microservice.
    pub auth_client: AuthServiceClient<Channel>,
}
