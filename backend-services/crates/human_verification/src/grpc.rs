//! gRPC network interface implementation for processing human verification requests.

use crate::providers::{arrow_alignment_provider::ArrowAlignmentProvider, VerificationProvider};
use oclink_contracts::human_verification::v1::human_verification_service_server::HumanVerificationService;
use oclink_contracts::human_verification::v1::{
    GetChallengeRequest, GetChallengeResponse, VerifyRequest, VerifyResponse,
};
use oclink_human_verification_crypto::CryptoEngine;
use std::sync::Arc;
use tonic::{Request, Response, Status};

/// Implements the proto-generated gRPC service using shared verification engines.
pub struct VerificationGrpcServer {
    pub crypto: Arc<CryptoEngine>,
    pub arrow_provider: Arc<ArrowAlignmentProvider>,
}

impl VerificationGrpcServer {
    /// Creates a new gRPC service instance with injected dependencies.
    pub fn new(crypto: Arc<CryptoEngine>, arrow_provider: Arc<ArrowAlignmentProvider>) -> Self {
        Self {
            crypto,
            arrow_provider,
        }
    }
}

#[tonic::async_trait]
impl HumanVerificationService for VerificationGrpcServer {
    /// Dispatches challenge requests to the configured provider strategy.
    async fn get_challenge(
        &self,
        req: Request<GetChallengeRequest>,
    ) -> Result<Response<GetChallengeResponse>, Status> {
        let inner = req.into_inner();

        let payload_json = match inner.provider_id.as_str() {
            "arrow_alignment" => self
                .arrow_provider
                .generate_challenge(&inner.edition_id)?
                .to_string(),
            _ => return Err(Status::invalid_argument("Provider disabled or unknown")),
        };

        Ok(Response::new(GetChallengeResponse {
            challenge_payload_json: payload_json,
        }))
    }

    /// Validates the challenge answer and returns a short-lived signed voucher if successful.
    async fn verify(
        &self,
        req: Request<VerifyRequest>,
    ) -> Result<Response<VerifyResponse>, Status> {
        let inner = req.into_inner();

        // Ensure standard JSON structure is present.
        let parsed_payload: serde_json::Value = serde_json::from_str(&inner.payload_json)
            .map_err(|_| Status::invalid_argument("Malformed JSON payload"))?;

        // Route payload processing to the target provider.
        let provider: Arc<dyn VerificationProvider> = match inner.provider_id.as_str() {
            "arrow_alignment" => self.arrow_provider.clone(),
            _ => return Err(Status::invalid_argument("Provider unknown")),
        };

        match provider.verify(&parsed_payload).await {
            Ok(true) => {
                // Issue a 5-minute transient voucher.
                let voucher = self.crypto.generate_signed_voucher(300).unwrap();
                Ok(Response::new(VerifyResponse {
                    success: true,
                    voucher,
                }))
            }
            Ok(false) => Ok(Response::new(VerifyResponse {
                success: false,
                voucher: "".to_string(),
            })),
            Err(e) => Err(Status::internal(e.to_string())),
        }
    }
}
