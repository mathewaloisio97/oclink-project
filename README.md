# OcLink — Project Blueprint & System Architecture

This repository contains the source code for **OcLink**, a distributed system consisting of a **Rust backend cluster** and decoupled **C# APIs**. 

Network contracts are managed in a centralized Protocol Buffers (Protobuf) automation pipeline to prevent data drift between the client and backend runtimes.

---

## Repository Architecture

```text
oclink-project/
¦
+-- contracts/                        # Central source of truth for all network schemas
¦   +-- auth/v1/auth.proto            # Versioned domain contracts
¦   +-- build_contracts.py            # Python compilation driver
¦   +-- justfile                      # Task runner for C# generation
¦
+-- dot-net-apis/                     # Managed API client layer (.NET Standard 2.0)
¦   +-- OcLink.API.Core/
¦       +-- OcLink.API.Core.csproj    # Targets .NET Standard 2.0
¦       +-- Generated/                # Git-ignored compilation target for C# Protobufs
¦
+-- backend-services/                 # Rust Microservices
¦   +-- Cargo.toml                    # Master workspace
¦   +-- crates/gateway                # Axum API Gateway with Utoipa (OpenAPI)
¦   +-- Dockerfile                    # Multi-stage Docker build
¦
+-- website/                          # React/Vite Developer PIT Portal
```

## Developer Setup

We use `just` as our cross-platform command runner to handle builds consistently across macOS, Linux, and Windows without relying on fragmented shell scripts.

### Prerequisites

Ensure you have the core runtimes installed before proceeding:
1. **Python 3.14+** 
2. **Rust & Cargo** (Installed via [rustup.rs](https://rustup.rs/))
3. **Docker Desktop** (or equivalent container daemon)
4. **.NET 8 SDK**
5. **Node.js 20+**
6. **Protobuf Compiler (`protoc`)** *(Install via `brew install protobuf`, `apt install protobuf-compiler`, or `winget install Google.Protobuf` depending on your OS)*

### Step 1: Install & Verify CLI Tools
Open your terminal and run the following commands to install the required polyglot build tools:

```bash
# Install 'just' using Cargo (Cross-platform)
cargo install just

# Install pnpm for strict frontend dependency management
npm install -g pnpm

# Verify all installations
just --version
dotnet --version
cargo --version
pnpm --version
protoc --version
```

### Step 2: Compile Contracts to C#
Compile the `.proto` schemas into C# classes and build the `.NET Standard 2.0` assembly:
```bash
cd contracts
just build
```

### Step 3: Environment & Infrastructure Setup
Before starting the backend, establish your local environment variables and boot the infrastructure:
```bash
cd ..
# Copy the example environment file to your local machine
cp .env.example .env

# Start PostgreSQL, RabbitMQ, Mailpit, and the Rust microservices
docker compose up -d --build
```
* **Swagger OpenAPI Docs:** `http://localhost:3000/swagger-ui`
* **Mailpit (Emails):** `http://localhost:8025`

### Step 4: Start the Web Portal
Open a new terminal window, navigate to the `website` directory, and start the Vite development server:
```bash
cd website
pnpm install
pnpm dev
```
* **React Web Portal:** `http://localhost:5173`

---

## Architecture & Project Info

* **Lead Software Engineer:** Mathew Aloisio
* **Project Purpose:** Portfolio demonstration of low-latency, zero-allocation network state machines, cross-runtime decoupling (.NET Standard 2.0/Rust), and automated contract workflows.

### Links & Contact
* **Portfolio:** [mathewaloisio.com](https://mathewaloisio.com)
* **LinkedIn:** [linkedin.com/in/mathew-aloisio-594025404](https://www.linkedin.com/in/mathew-aloisio-594025404/)
* **Email:** [mathew.aloisio97@gmail.com](mailto:mathew.aloisio97@gmail.com)
