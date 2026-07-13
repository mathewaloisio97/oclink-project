//! Verification Provider interfaces and routing.

pub mod arrow_alignment_provider;

use async_trait::async_trait;
use serde_json::Value;

/// Defines the contract for all human verification mechanisms.
#[async_trait]
pub trait VerificationProvider: Send + Sync {
    /// Generates a stateless challenge payload required by the client.
    fn generate_challenge(&self, edition_id: &str) -> Result<Value, tonic::Status>;

    /// Mathematically or externally verifies the client's puzzle submission.
    async fn verify(
        &self,
        payload: &Value,
    ) -> Result<bool, Box<dyn std::error::Error + Send + Sync>>;
}
