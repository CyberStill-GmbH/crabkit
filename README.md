<div align="center">

<img src="https://raw.githubusercontent.com/rust-lang/rust-artwork/master/logo/rust-logo-512x512.png" width="96" alt="Rust Logo" />

# CRABKIT

### Offensive Security Framework built in Rust

**High-performance reconnaissance, endpoint discovery, and tactical fuzzing engine for authorized security testing.**

<br />

<img src="https://img.shields.io/badge/Rust-1.70%2B-000000?style=for-the-badge&logo=rust&logoColor=white" alt="Rust 1.70+" />
<img src="https://img.shields.io/badge/Runtime-Tokio-111827?style=for-the-badge&logo=rust&logoColor=white" alt="Tokio Runtime" />
<img src="https://img.shields.io/badge/HTTP-Reqwest-1F2937?style=for-the-badge&logo=rust&logoColor=white" alt="Reqwest" />
<img src="https://img.shields.io/badge/Security-Authorized%20Use-7F1D1D?style=for-the-badge" alt="Authorized Use Only" />

<br />
<br />

<sub>
Engineered for controlled security assessments where throughput, memory safety, and concurrency discipline matter.
</sub>

</div>

---

## Overview

**Crabkit** is a Rust-based offensive security framework focused on high-speed reconnaissance and controlled endpoint discovery.  
It is designed for environments where predictable performance, memory safety, and precise concurrency control are critical.

Unlike traditional tools that rely on heavier runtime models, Crabkit uses an asynchronous task-based execution model to maximize network throughput while keeping system resource usage predictable.

The first MVP focuses on:

- Asset enumeration.
- Endpoint discovery.
- High-concurrency HTTP/S probing.
- Response filtering by status code and body length.
- Large-scale wordlist streaming with low memory overhead.

> Crabkit is intended strictly for educational research, internal security testing, and authorized penetration testing.

---

## Technical Philosophy

Crabkit is built around a simple engineering principle:

> **Do more work with less overhead, without sacrificing control.**

In high-load reconnaissance workflows, unpredictable runtime pauses and excessive allocations can become bottlenecks. Crabkit reduces those risks through Rust’s ownership model and a carefully controlled asynchronous architecture.

### Core Principles

| Principle | Description |
| :--- | :--- |
| **Memory Safety** | Rust’s ownership and borrowing rules reduce entire classes of memory-related bugs. |
| **Predictable Performance** | No garbage collector, fewer runtime surprises, and tighter control over allocations. |
| **Async-first Execution** | Tokio-powered task scheduling for high-throughput network operations. |
| **Backpressure Control** | Semaphore-based throttling to avoid overwhelming the operating system network stack. |
| **Typed Error Handling** | Explicit error propagation using Rust’s type system instead of silent failures. |
| **Streaming IO** | Wordlists are processed incrementally to support large files without loading them fully into memory. |

---

## System Architecture

Crabkit follows a modular architecture so each tactical capability can evolve independently.

```text
crabkit/
├── core/
│   └── engine.rs        # Request orchestration and concurrency control
├── net/
│   └── client.rs        # HTTP/S transport abstraction
├── io/
│   └── streamer.rs      # High-performance wordlist reader
├── logic/
│   └── mutator.rs       # Payload generation and mutation logic
├── output/
│   └── reporter.rs      # Structured output and result formatting
└── main.rs              # CLI entrypoint
```

### Module Responsibilities

| Module | Responsibility | Key Technology |
| :--- | :--- | :--- |
| `Core::Engine` | Coordinates tasks, concurrency limits, retries, and execution flow. | Tokio Runtime |
| `Net::Client` | Provides HTTP/S request abstraction and transport configuration. | Reqwest |
| `IO::Streamer` | Reads large wordlists efficiently with minimal RAM usage. | BufReader |
| `Logic::Mutator` | Generates candidate paths, payload variants, and controlled mutations. | Rust std / Bitwise Ops |
| `Output::Reporter` | Normalizes results for terminal and machine-readable formats. | JSON / Stdout |

---

## MVP Scope

The current MVP is intentionally focused and performance-oriented.

### Included

- Massive parallel execution using lightweight asynchronous tasks.
- Configurable concurrency limit.
- HTTP/S endpoint probing.
- Automatic timeout handling.
- Retry strategy for unstable responses.
- Response filtering by:
  - HTTP status code.
  - Response body length.
  - Latency threshold.
- Large wordlist support through streaming reads.
- CLI-based execution for fast operational use.

### Not Included Yet

- Distributed scanning.
- Authentication-aware crawling.
- Browser-based rendering.
- Exploit execution.
- Persistence mechanisms.
- Automated exploitation chains.

This keeps the first release clean, auditable, and focused on reconnaissance primitives.

---

## Installation

### Requirements

- Rust Stable `1.70+`
- Cargo
- Linux, macOS, or Windows with a supported Rust toolchain

### Clone Repository

```bash
git clone https://github.com/tu-usuario/crabkit.git
cd crabkit
```

### Build Development Version

```bash
cargo build
```

### Build Optimized Release

```bash
cargo build --release
```

The optimized binary will be generated at:

```bash
./target/release/crabkit
```

---

## Usage

Basic execution:

```bash
./crabkit \
  --target <URL> \
  --wordlist <PATH> \
  --concurrency <INT>
```

Example using a controlled internal target:

```bash
./crabkit \
  --target https://api.internal.example \
  --wordlist ./wordlists/endpoints.txt \
  --concurrency 100
```

### Recommended Operational Flags

```bash
./crabkit \
  --target https://api.internal.example \
  --wordlist ./wordlists/endpoints.txt \
  --concurrency 80 \
  --timeout 5 \
  --retries 2 \
  --filter-status 200,204,301,302,403
```

---

## Output Model

Crabkit is designed to produce both human-readable and pipeline-friendly output.

### Terminal Output

```text
[200] /api/v1/users              42ms     1842 bytes
[403] /admin                    51ms      721 bytes
[301] /dashboard                38ms      128 bytes
```

### Planned JSON Output

```json
{
  "target": "https://api.internal.example",
  "path": "/api/v1/users",
  "status": 200,
  "latency_ms": 42,
  "content_length": 1842
}
```

---

## Performance Strategy

Crabkit is engineered around controlled pressure, not blind aggression.

### Concurrency Control

The engine uses semaphore-based throttling to ensure that concurrency remains bounded and predictable.

```text
Wordlist Stream
      │
      ▼
Task Scheduler ──► Semaphore ──► HTTP Client Pool ──► Response Filter
      │                                                   │
      └──────────────────── Reporter ◄────────────────────┘
```

### Why Rust

Rust is a strong fit for this type of tool because it provides:

- Low-level performance without sacrificing safety.
- No garbage collector pauses.
- Excellent async ecosystem through Tokio.
- Strong compile-time guarantees.
- Efficient binaries suitable for operational environments.

---

## Roadmap

### Phase 01 — Core Engine

Status: **Completed**

- Asynchronous execution engine.
- Semaphore-based concurrency management.
- Streaming wordlist reader.
- Basic HTTP/S probing.
- Status and length-based filtering.

### Phase 02 — Tactical Fuzzing Layer

Status: **In Development**

- Controlled payload mutation.
- Bit-level mutation primitives.
- Latency anomaly detection.
- Structured JSON export.
- Improved retry and timeout policies.

### Phase 03 — Cloud-Aware Reconnaissance

Status: **Planned**

- AWS S3 bucket enumeration for authorized assessments.
- Lambda proxy support for controlled distributed testing.
- Result aggregation layer.
- Configuration profiles for different assessment types.

### Phase 04 — Detection and Reporting

Status: **Planned**

- HTML report generation.
- Baseline comparison.
- Noise reduction heuristics.
- Integration-ready output for security pipelines.

---

## Security and Ethics

Crabkit must only be used in environments where explicit authorization has been granted.

Acceptable use cases include:

- Internal security assessments.
- Authorized penetration testing.
- Bug bounty programs within defined scope.
- Educational labs and controlled training environments.
- Defensive validation of exposed services.

Unacceptable use includes:

- Scanning systems without permission.
- Attempting to bypass access controls.
- Targeting third-party infrastructure outside an approved scope.
- Using the tool for unauthorized exploitation or disruption.

The author assumes no responsibility for misuse of this software.

---

## Engineering Standards

Crabkit aims to follow production-grade Rust engineering practices:

- Clear module boundaries.
- Explicit error handling.
- Minimal global state.
- Conservative defaults.
- Configurable runtime behavior.
- Auditable code paths.
- Deterministic CLI behavior.
- CI-ready build pipeline.

Recommended quality checks:

```bash
cargo fmt
cargo clippy -- -D warnings
cargo test
```

---

## Suggested Repository Badges

Once the repository is public, consider adding:

```markdown
![Build](https://img.shields.io/github/actions/workflow/status/tu-usuario/crabkit/rust.yml?style=for-the-badge)
![License](https://img.shields.io/github/license/tu-usuario/crabkit?style=for-the-badge)
![Stars](https://img.shields.io/github/stars/tu-usuario/crabkit?style=for-the-badge)
```

---

## License

This project is released for educational and authorized security research purposes.

- MIT

---

<div align="center">

### CRABKIT

**Fast. Controlled. Memory-safe. Built for authorized security operations.**

<sub>Rust-powered reconnaissance with senior-grade engineering discipline.</sub>

</div>
