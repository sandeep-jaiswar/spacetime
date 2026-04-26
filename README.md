# Zero-Trust Orchestration Fabric

A high-performance orchestration system for autonomous software engineering. Built natively as a highly concurrent Rust monorepo, this fabric ingests Product Requirement Documents (PRDs) and manages a swarm of specialized AI agents (Architects, Coders, Security, QA) to autonomously build, verify, and deploy production-ready systems.

Designed for efficiency and scale (intended for 1M+ users), this acts as the "Operating System" for autonomous code generation and validation.

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

1. **`crates/domain`**: Foundational domain primitives, data models, enumerations, and custom Result/Error mappings utilized system-wide.
2. **`crates/ghost-mesh`**: The Zero-Trust Public Key Infrastructure executing passport issuance, signature generation, and verification at microsecond latency.
3. **`crates/orchestrator`**: The task distribution and Directed Acyclic Graph (DAG) state machine.
4. **`crates/agents`**: Specialized agent interface layers (Architect, Coder, etc.) tied to the `llama-server` backend via async `LlmProvider` wrappers.
5. **`services/api-gateway`**: The user-facing ingress layer (`Axum` server). Manages incoming PRDs and outbound real-time streams of the agent's decision deliberation process.

## Quickstart

Verify the monorepo logic correctly links and parses shared dependencies from the workspace root:

```bash
cargo check --workspace
```

Execute the Ghost-Mesh security test suite against forged or expired passports:

```bash
cargo test --workspace
```