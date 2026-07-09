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
