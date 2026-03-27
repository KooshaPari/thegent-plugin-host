# ADR: thegent-plugin-host

## ADR-001: WASM as Primary Plugin Runtime

**Status**: Accepted

**Context**: Plugin execution sandboxing options: (a) OS processes with IPC, (b) WASM runtime, (c) native shared libraries (.so/.dylib), (d) language-specific VMs.

**Decision**: WebAssembly via wasmtime as the primary plugin runtime.

**Rationale**: WASM provides memory isolation, capability-based security, and cross-platform portability. Native shared libs (.so) can crash the host process on panic. OS processes have higher overhead and IPC complexity. WASM is now production-grade for server-side use.

**Consequences**: Plugin authors must target WASM (most languages can compile to WASM). Native performance plugins may need a native adapter layer.

---

## ADR-002: Capability-Based Security Model

**Status**: Accepted

**Context**: Plugins need controlled access to host resources (filesystem, network). Two approaches: (a) full access (unsafe), (b) capability-based grants.

**Decision**: Capability-based security. Plugins declare required capabilities in manifest; host operator approves at install time.

**Rationale**: Principle of least privilege. Users can inspect exactly what a plugin requests before installing. Similar to mobile app permission models which have proven user familiarity.

**Consequences**: Plugin manifest must be accurate about capability requirements. Host must maintain a capability approval store per plugin.

---

## ADR-003: Hexagonal Architecture for Plugin Domain

**Status**: Accepted

**Context**: Plugin host has clear port boundaries: plugin loading, plugin invocation, event emission, registry access.

**Decision**: Domain model in `domain/`, use cases in `application/`, ports as traits in `ports/`, adapters (wasmtime, registry HTTP client) in `adapters/`.

**Rationale**: Hexagonal architecture isolates the core plugin lifecycle logic from concrete WASM runtime details. Future migration to a different sandbox (e.g., WASIX) only requires replacing the adapter.

**Consequences**: More boilerplate for simple operations. Justified by the security-critical nature of the plugin host — testability and isolation are paramount.
