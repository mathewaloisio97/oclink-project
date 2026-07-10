//! Identity domain error definitions.
//!
//! Groups all the potential errors that can happen during identity or profile
//! processing, such as database failures or duplicate account registrations.

use thiserror::Error;

/// Groups all errors that can occur during identity operations.
#[derive(Error, Debug)]
pub enum IdentityError {
    /// Sent back when someone tries to register a username that is already taken.
    #[error("user with username '{0}' already exists")]
    AlreadyExists(String),

    /// Sent back when the underlying database experiences a query or connection failure.
    #[error("database error: {0}")]
    Database(#[from] sqlx::Error),
}
