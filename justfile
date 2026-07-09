# GNU AFFERO GENERAL PUBLIC LICENSE
# Version 3, 19 November 2007
#
# Copyright (C) 2026 Mathew Aloisio
#
# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as published
# by the Free Software Foundation, either version 3 of the License, or
# (at your option) any later version.
#
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
# GNU Affero General Public License for more details.
#
# You should have received a copy of the GNU Affero General Public License
# along with this program. If not, see <https://www.gnu.org/licenses/>.

# Configure the execution shell safely for Windows environments.
set windows-shell := ["cmd.exe", "/c"]

default:
    @just --list

# Run the full validation pipeline locally.
ci: check-contracts check-rust check-frontend

# Compile Protobuf schemas and build the .NET assembly.
check-contracts:
    cd contracts && just build

# Format, lint, and build the Rust backend.
check-rust:
    cd backend-services && cargo fmt -- --check
    cd backend-services && cargo clippy --all-targets --all-features -- -D warnings
    cd backend-services && cargo build --release

# Install dependencies and build the React portal.
check-frontend:
    cd website && pnpm install --frozen-lockfile
    cd website && pnpm build

# --- Developer Database & SQLx Utilities ---

# Spins up the database and ensures all schemas are migrated.
db-up:
    docker compose up -d postgres
    @echo Waiting for Postgres to boot...
    timeout 3 >nul 2>&1 || ping -n 4 127.0.0.1 >nul
    @echo Running Identity Migrations...
    cd backend-services/crates/identity && cargo sqlx migrate run

# Updates the .sqlx offline caches for all microservices, then stops the DB.
db-prepare: db-up
    @echo Preparing offline cache for Identity...
    cd backend-services/crates/identity && cargo sqlx prepare
    @echo Stopping Postgres...
    docker compose stop postgres
    @echo SUCCESS: Offline caches updated. You can now commit the .sqlx folders!

# Destroys the database volume entirely (Useful if initialization scripts change).
db-clean:
    docker compose down -v
