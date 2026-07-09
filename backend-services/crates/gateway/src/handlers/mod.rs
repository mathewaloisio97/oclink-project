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

//! Route Handlers for the Edge Gateway API.
//!
//! This module aggregates and re-exports all endpoint handlers for the gateway routing
//! layer. Handlers are split into dedicated domain submodules (e.g., identity) to maintain
//! clean isolation as the public REST API scales.

pub mod identity;

// Re-export them so main.rs can call `handlers::register` directly.
pub use identity::{login, register};
