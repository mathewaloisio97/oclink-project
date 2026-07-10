//! User database storage and lookup.
//!
//! Handles saving new users to our database and fetching them later.
//! It includes automated logic to safely retry operations if the database
//! experiences a temporary connection issue.

use crate::errors::IdentityError;
use crate::models::User;
use async_trait::async_trait;
use sqlx::PgPool;

/// Defines the shared functions needed to manage user accounts in storage.
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Saves a new user record. Returns an error if the username is already taken.
    async fn create(&self, username: &str, password_hash: &str) -> Result<User, IdentityError>;

    /// Looks up a user by their exact username. Returns none if no match is found.
    async fn get_by_username(&self, username: &str) -> Result<Option<User>, IdentityError>;
}

/// PostgreSQL implementation of the user repository using SQLx.
pub struct PostgresUserRepository {
    pool: PgPool,
}

impl PostgresUserRepository {
    /// Creates a new repository instance backed by a live database connection pool.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

/// Automatically retries a database query up to 3 times with a short pause
/// if it hits a temporary network drop or pool timeout. It fails immediately
/// for logic errors like invalid queries or duplicate data.
macro_rules! with_retry {
    ($op:expr) => {{
        let mut retries = 3;
        let mut backoff = std::time::Duration::from_millis(100);
        loop {
            match $op.await {
                Ok(res) => break Ok(res),
                Err(e) => {
                    // Only retry on network, I/O, or connection pool timeouts.
                    let is_transient = match &e {
                        sqlx::Error::PoolTimedOut | sqlx::Error::PoolClosed => true,
                        sqlx::Error::Io(_) => true,
                        _ => false,
                    };

                    if is_transient && retries > 0 {
                        tracing::warn!(
                            "Database hit a temporary issue, retrying in {:?}...",
                            backoff
                        );
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
    // The password hash is skipped here so it never accidentally leaks into system logs.
    #[tracing::instrument(skip(self, password_hash))]
    async fn create(&self, username: &str, password_hash: &str) -> Result<User, IdentityError> {
        // Generate a time-sorted UUIDv7 to keep database search indexes fast and sequential.
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
                // Code 23505 means a unique constraint failed (username already exists).
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
