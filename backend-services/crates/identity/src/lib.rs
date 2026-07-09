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

//! User identity and core authentication backend engine.
//!
//! Encapsulates user database access operations, provides cryptographic
//! password verification via Argon2id, and exposes the implementation
//! structures for downstream gRPC service mounting.

pub mod errors;
pub mod models;
pub mod repository;

use crate::repository::UserRepository;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use oclink_contracts::identity::v1::identity_service_server::IdentityService;
use oclink_contracts::identity::v1::{
    CreateUserRequest, CreateUserResponse, VerifyCredentialsRequest, VerifyCredentialsResponse,
};
use std::sync::Arc;
use tonic::{Request, Response, Status};
use tracing::{error, info, instrument};

/// gRPC service implementation handling user authentication and registration workflows.
pub struct OcLinkIdentity {
    /// Thread-safe backing repository using dynamic dispatch.
    repo: Arc<dyn UserRepository>,
}

impl OcLinkIdentity {
    pub fn new(repo: Arc<dyn UserRepository>) -> Self {
        Self { repo }
    }
}

#[tonic::async_trait]
impl IdentityService for OcLinkIdentity {
    /// Validates raw input arguments, hashes the password using Argon2id,
    /// and registers the new user within the persistence layer.
    #[instrument(skip(self, req))]
    async fn create_user(
        &self,
        req: Request<CreateUserRequest>,
    ) -> Result<Response<CreateUserResponse>, Status> {
        let inner = req.into_inner();
        let username = inner.username;
        let password = inner.password;

        if username.trim().is_empty() || password.trim().is_empty() {
            return Err(Status::invalid_argument(
                "Username and password are required",
            ));
        }

        info!(username = %username, "Identity: Initiating user provisioning");

        // Hash password securely before persistent storage
        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| {
                error!("Critical Cryptography Fault: {}", e);
                Status::internal("Internal cryptography fault")
            })?
            .to_string();

        let internal_user =
            self.repo
                .create(&username, &password_hash)
                .await
                .map_err(|e| match e {
                    crate::errors::IdentityError::AlreadyExists(_) => {
                        Status::already_exists("Username in use")
                    }
                    _ => {
                        error!("Database Fault: {:?}", e);
                        Status::internal("Internal database fault")
                    }
                })?;

        Ok(Response::new(CreateUserResponse {
            user_id: internal_user.id.to_string(),
        }))
    }

    /// Pulls a user profile by username and evaluates the provided cleartext password
    /// against the stored Argon2 hash representation.
    #[instrument(skip(self, req))]
    async fn verify_credentials(
        &self,
        req: Request<VerifyCredentialsRequest>,
    ) -> Result<Response<VerifyCredentialsResponse>, Status> {
        let inner = req.into_inner();

        info!(username = %inner.username, "Identity: Executing credential verification sequence");

        let user_opt = self
            .repo
            .get_by_username(&inner.username)
            .await
            .map_err(|e| {
                error!("Database fault during credential lookup: {}", e);
                Status::internal("Internal database fault")
            })?;

        // Constant-time execution path protection when a record is matched
        if let Some(user) = user_opt {
            let parsed_hash = PasswordHash::new(&user.password_hash).map_err(|_| {
                error!("Data Integrity Fault: Corrupt hash payload");
                Status::internal("Data integrity fault")
            })?;

            if Argon2::default()
                .verify_password(inner.password.as_bytes(), &parsed_hash)
                .is_ok()
            {
                return Ok(Response::new(VerifyCredentialsResponse {
                    user_id: user.id.to_string(),
                    valid: true,
                }));
            }
        }

        // Returns a safe false flag instead of throwing a gRPC NotFound error
        // to prevent potential account enumeration attacks.
        Ok(Response::new(VerifyCredentialsResponse {
            user_id: "".to_string(),
            valid: false,
        }))
    }
}
