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

use std::env;
use std::path::PathBuf;

/// Build script to compile gRPC Protobuf contracts into Rust types using Tonic.
///
/// This script runs automatically before the crate compiles. It reads the shared
/// enterprise API contracts, injects Serde attributes for downstream caching/logging,
/// and generates both client and server stubs.
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Resolve absolute paths based on the workspace manifest root
    // to prevent path drift if this crate is moved.
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let contracts_root = manifest_dir.join("../../../contracts");

    // Define target proto files relative to the contracts root.
    let proto_files = &[
        contracts_root.join("auth/v1/auth.proto"),
        contracts_root.join("identity/v1/identity.proto"),
    ];

    // Configure and execute the Tonic build pipeline
    tonic_prost_build::configure()
        .build_server(true)
        .build_client(true)
        // CRITICAL: We inject Serde derives globally (".") across all generated structs.
        // This allows our edge gateways to transparently serialize gRPC payloads into
        // JSON for internal HTTP telemetry and Redis caching layers.
        .type_attribute(".", "#[derive(serde::Serialize, serde::Deserialize)]")
        .compile_protos(proto_files, &[contracts_root])?;

    // Optimization: Tell Cargo to rerun this script ONLY if the .proto files change.
    // Without this, Cargo might unnecessarily recompile this script on every build.
    for proto in proto_files {
        println!("cargo:rerun-if-changed={}", proto.display());
    }

    Ok(())
}
