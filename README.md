# Zero-Trust Orchestration Fabric

A high-performance orchestration system for autonomous software engineering. Built natively as a highly concurrent Rust monorepo, this fabric ingests Product Requirement Documents (PRDs) and manages a swarm of specialized AI agents (Architects, Coders, Security, QA) to autonomously build, verify, and deploy production-ready systems.

> [!NOTE]
> **Project Status / Current Scaffold**
> - **`services/api-gateway`**: Functional ingress with health checks. WebSocket deliberation broadcasting is WIP.
> - **`crates/orchestrator`**: Skeletal implementation of the DAG state machine.
> - **`crates/agents`**: Scaffolded with `LlmProvider` trait. Requires a local `llama-server` backend.
> - **`crates/ghost-mesh`**: Core passport issuance and crypto-first verification enabled.

## Environment Requirements
- **Rust Toolchain**: ≥1.85 (Edition 2024)
- **Local Inference**: A running `llama.cpp` HTTP server (e.g., `llama-server --model ... --port 8080`) is required for the `agents` crate to function.

## Core Philosophical Pillars

### 1. Reasoning-on-the-Loop
We invert the typical AI coding dynamic. Instead of agents blindly writing code and humans reviewing syntax, this framework operates under **Reasoning-on-the-Loop**. 
Humans review the *decisions* the Lead Agent makes at critical crossroads. If a major architectural divergence or ambiguity arises in the PRD, the swarm pauses execution, broadcasts the deliberation state over the API Gateway via WebSockets, and awaits manual traversal validation before continuing.

### 2. Local AI Privacy (`llama.cpp`)
To guarantee source material and ideation privacy, the agent swarms do not send code via Commercial APIs (like OpenAI). Instead, the system implements a direct `LlmProvider` integration targeting distributed pools of `llama.cpp` servers exposing lightweight local inference HTTP boundaries. 

### 3. Ghost-Mesh & Agent Passports
Every autonomous agent requires a digital "passport" to interface with legacy backend systems, databases, or cloud infrastructure.
**Ghost-Mesh** is our proprietary Zero-Trust Engine. Avoiding bulky, bloated enterprise standards, we use high-speed binary serialization (`bincode`) and elliptic curve cryptography (`ed25519-dalek`) to issue tamper-proof TTL-based identities natively inside memory. This ensures absolute trust, preventing privilege escalation between the discrete roles of the swarm.

## High-Level Architecture (Cargo Workspace)

1. **`crates/domain`**: Foundational domain primitives, data models, and custom `CoreError` mappings.
2. **`crates/ghost-mesh`**: The Zero-Trust Public Key Infrastructure executing passport issuance and crypto-first verification.
3. **`crates/orchestrator`**: The task distribution and Directed Acyclic Graph (DAG) state machine.
4. **`crates/agents`**: Specialized agent interface layers tied to the `llama-server` backend via async `LlmProvider` wrappers.
5. **`services/api-gateway`**: The user-facing ingress layer (`Axum` server).

## Quickstart

### 1. Verification
Verify the monorepo logic correctly links and parses shared dependencies from the workspace root:

```bash
cargo check --workspace
```

Execute the Ghost-Mesh security test suite:

```bash
cargo test --workspace
```

### 2. Running Locally
Start the API Gateway:

```bash
# Optional: BIND_ADDR="127.0.0.1:4000"
cargo run -p api-gateway
```

In another terminal, verify the service:

```bash
curl http://localhost:3000/health
# Expected: "Gateway is running!"
```