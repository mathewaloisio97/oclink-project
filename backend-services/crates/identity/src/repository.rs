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

//! Database abstraction layer for user persistence.
//!
//! Defines the core decoupling data-access traits and provides PostgreSQL
//! implementation featuring automated transient-fault retries.

use crate::errors::IdentityError;
use crate::models::User;
use async_trait::async_trait;
use sqlx::PgPool;

/// Data access layer interface for user identity management.
/// Uses `async_trait` to maintain compatibility with dynamic dispatch.
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Persists a new user record. Returns `IdentityError::AlreadyExists`
    /// if the username is taken.
    async fn create(&self, username: &str, password_hash: &str) -> Result<User, IdentityError>;

    /// Looks up a user by their exact username. Returns `Ok(None)` if no match is found.
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, IdentityError>;
}

/// PostgreSQL implementation of the user repository using SQLx.
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

/// Automatically retries a database operation up to 3 times with exponential backoff
/// if it encounters a transient infrastructure failure (e.g., I/O errors or pool exhaustion).
/// Non-transient errors (like constraint violations) fail immediately.
macro_rules! with_retry {
    ($op:expr) => {{
        let mut retries = 3;
        let mut backoff = std::time::Duration::from_millis(100);
        loop {
            match $op.await {
                Ok(res) => break Ok(res),
                Err(e) => {
                    // Check if the error is connectivity/pool related rather than syntax or schema constraints.
                    let is_transient = match &e {
                        sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed => true,
                        sqlx::Error::Io(_) => true,
                        _ => false,
                    };

                    if is_transient && retries > 0 {
                        tracing::warn!("Database transient failure, retrying in {:?}...", backoff);
                        retries -= 1;
                        tokio::time::sleep(backoff).await;
                        backoff *= 2;
                        continue;
                    }
                    break Err(e);
                }
            }
        }
    }};
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    // Sensitive data (password_hash) is explicitly skipped to protect log security.
    #[tracing::instrument(skip(self, password_hash))]
    async fn create(&self, username: &str, password_hash: &str) -> Result<User, IdentityError> {
        // Generate a time-sorted UUIDv7 to prevent Postgres B-Tree index fragmentation.
        let new_user_id = uuid::Uuid::now_v7();

        let user = with_retry!(sqlx::query_as!(
            User,
            r#"
            INSERT INTO users (id, username, password_hash)
            VALUES ($1, $2, $3)
            RETURNING id, username, password_hash
            "#,
            new_user_id,
            username,
            password_hash
        )
        .fetch_one(&self.pool))
        .map_err(|e| {
            if let Some(db_err) = e.as_database_error() {
                // 23505 is the PostgreSQL code for a unique constraint violation,
                // return a typed error.
                if db_err.code().as_deref() == Some("23505") {
                    return IdentityError::AlreadyExists(username.to_string());
                }
            }
            IdentityError::Database(e)
        })?;

        Ok(user)
    }

    #[tracing::instrument(skip(self))]
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, IdentityError> {
        let user = with_retry!(sqlx::query_as!(
            User,
            r#"
            SELECT id, username, password_hash
            FROM users
            WHERE username = $1
            "#,
            username
        )
        .fetch_optional(&self.pool))
        .map_err(IdentityError::Database)?;

        Ok(user)
    }
}
