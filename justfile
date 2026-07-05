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
