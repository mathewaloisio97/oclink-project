# OcLink — Project Blueprint & System Architecture

This repository contains the source code for **OcLink**, a distributed system consisting of a decoupled **Rust backend cluster** and **C# APIs**.

The project utilizes a centralized Protobuf pipeline to handle contract generation. This keeps the C# clients and Rust backend microservices perfectly in sync to prevent API breaking changes.

---

## Repository Architecture

```text
oclink-project/
├── .github/workflows/      # CI/CD pipelines enforcing offline validation and testing
├── backend-services/       # Rust microservice workspace (Identity, Gateway, Auth, etc.)
├── contracts/              # Central source of truth for all network schemas (.proto)
├── docker/                 # Infrastructure provisioning scripts (e.g., Postgres initialization)
├── dot-net-apis/           # Managed API client layer strictly targeting .NET Standard 2.0
├── website/                # React/Vite Developer Player Identity Token (PIT) Portal
├── .editorconfig           # Enforces unified cross-platform formatting rules
├── .env.example            # Master runtime configuration template
├── docker-compose.yml      # Local development infrastructure definition
└── justfile                # Root task runner for repository-wide pipeline automation
```

## Infrastructure & Development Philosophy

### CI/CD Parity & Offline Compilation
To guarantee parity between local development machines and GitHub Actions, the Rust backend enforces **SQLx Offline Mode** via `.cargo/config.toml`. Running a standard compilation (`cargo check` or `cargo build`) requires zero running infrastructure. Database schemas are validated against committed `.sqlx` cache directories, ensuring rapid, deterministic builds.

### Passwordless Local Development
The local PostgreSQL infrastructure utilizes `POSTGRES_HOST_AUTH_METHOD=trust`. This explicitly bypasses password authentication for localized Docker connections. This architectural decision permanently eliminates dummy credentials (e.g., `oclink_dev_pass`) from the repository's `.env` and `docker-compose.yml` files, preventing false-positive alerts from automated enterprise secret scanners (e.g., GitGuardian, GitHub Advanced Security).

### Secure-by-Default Configuration
To prevent the accidental deployment of unsecure development keys into production environments, the cryptographic engine enforces a strict fail-safe mechanism. The default execution path actively monitors for fallback development secrets at boot. If detected, the application will intentionally panic and crash. To utilize localized fallback keys, developers must explicitly authorize the execution by compiling with the `local-dev` feature flag, guaranteeing that production releases remain secure by design.

---

## Developer Setup

The project utilizes `just` as the cross-platform command runner. This abstracts complex build requirements and ensures consistent execution across macOS, Linux, and Windows environments.

### Prerequisites

Ensure the following core runtimes are installed on your host machine:
1. **Python 3.14+**
2. **Rust & Cargo** (Installed via [rustup.rs](https://rustup.rs/))
3. **Docker Desktop** (or an equivalent container daemon)
4. **.NET 8 SDK**
5. **Node.js 24+**
6. **Protobuf Compiler (`protoc`)** *(Install via `brew install protobuf`, `apt install protobuf-compiler`, or `winget install Google.Protobuf`)*

### Step 1: Install CLI Tooling
Execute the following commands to install the required polyglot build utilities:

```bash
# Install 'just' and 'sqlx-cli' via Cargo
cargo install just
cargo install sqlx-cli --no-default-features --features rustls,postgres

# Install pnpm for deterministic frontend dependency management
npm install -g pnpm
```

### Step 2: Initialize the Environment
Copy the example environment configuration to establish your local routing variables.

```bash
cp .env.example .env
```

### Step 3: Database Preparation & Caching
Whenever a database schema is modified, or when initializing the repository for the first time, you must prepare the offline caches.

```bash
just db-prepare
```
*This command automates the entire infrastructure lifecycle: it boots the local PostgreSQL container, provisions the isolated logical databases, runs all SQL migrations, updates the `.sqlx` cache directories for the Rust compiler, and cleanly halts the container.*

### Step 4: Execute the Monorepo Validation Pipeline
To compile the Protobuf contracts into C#, build the .NET assemblies, compile the Rust backend offline, and build the React frontend, run the continuous integration simulation:

```bash
# Standard production-ready validation
just ci

# Localized validation permitting development secrets
just ci local-dev
```

---

## Running the Services Locally

To boot the infrastructure and run the microservices for localized testing:

1. **Start Infrastructure:**
   ```bash
   just db-up
   ```
   *(Boots PostgreSQL, RabbitMQ, and Mailpit in the background).*

2. **Start the Edge Gateway:**
   ```bash
   cd backend-services
   cargo run --bin oclink_gateway --features local-dev
   ```
   * **Swagger OpenAPI Docs:** `http://localhost:3000/swagger-ui`

3. **Start the Backend Microservices (e.g., Human Verification):**
   ```bash
   cd backend-services
   cargo run --bin oclink_human_verification --features local-dev
   ```

4. **Start the Web Portal:**
   ```bash
   cd website
   pnpm dev
   ```
   * **React Web Portal:** `http://localhost:5173`
   * **Mailpit (Emails):** `http://localhost:8025`

---

## Project & Contact Information

* **Author:** Mathew Aloisio
* **Project Purpose:** A core identity and authentication platform providing user registration, email verification, human validation checks, and secure token issuance. The system manages session security via standard access tokens alongside specialized Player Identity Tokens (PIT). Architecturally, the project demonstrates cross-runtime decoupling between a Rust backend and .NET Standard 2.0 clients using automated contract workflows.

### Links
* **Portfolio:** [mathewaloisio.com](https://mathewaloisio.com)
* **LinkedIn:** [linkedin.com/in/mathew-aloisio-594025404](https://www.linkedin.com/in/mathew-aloisio-594025404/)
* **Email:** [mathew.aloisio97@gmail.com](mailto:mathew.aloisio97@gmail.com)
