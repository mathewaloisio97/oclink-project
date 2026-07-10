//! Identity Domain Storage Models.
//!
//! This module defines the internal domain models and entity structures that map
//! directly to persistent storage tables in the identity database layer. These
//! structures remain strictly decoupled from both external gateway HTTP contracts
//! and gRPC wire schemas.

use uuid::Uuid;

/// Represents a persistently stored Identity entity.
#[derive(Debug, Clone)]
pub struct User {
    /// The globally unique identifier for the user (UUIDv7).
    pub id: Uuid,
    /// The username for login operations.
    pub username: String,
    /// The Argon2id representation of the user's password.
    pub password_hash: String,
}
