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

//! Shared API contract definitions for the OcLink ecosystem.
//!
//! Consolidates all auto-generated Protobuf modules and gRPC client/server
//! stubs into a unified assembly to eliminate cross-domain circular dependencies.

pub mod auth {
    pub mod v1 {
        tonic::include_proto!("auth.v1");
    }
}

pub mod identity {
    pub mod v1 {
        tonic::include_proto!("identity.v1");
    }
}
