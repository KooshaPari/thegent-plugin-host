# PRD: thegent-plugin-host — Plugin Host and Loader

## Overview
`thegent-plugin-host` is the plugin host and loader subsystem for the thegent multi-agent orchestration system. It provides a secure, sandboxed runtime for loading, managing, and invoking plugins that extend thegent functionality.

## Problem Statement
thegent must support extensibility without coupling extension code to the core binary. A plugin system allows third-party and first-party skill authors to package capabilities as standalone plugins that thegent loads and executes in a controlled manner.

## Goals
1. Load plugins from disk, registry, or network sources
2. Sandbox plugin execution (resource limits, capability grants)
3. Lifecycle management: install, activate, deactivate, uninstall
4. Version resolution and compatibility checking
5. Plugin communication via typed message-passing API

## Epics

### E1: Plugin Domain Model
- E1.1: Plugin entity (ID, name, version, capabilities, state)
- E1.2: PluginManifest (declared capabilities, entry point, constraints)
- E1.3: PluginState machine (unloaded, loading, active, suspended, failed)
- E1.4: Domain events (PluginLoaded, PluginActivated, PluginFailed)

### E2: Plugin Loading
- E2.1: Disk loader (load from `.wasm` or native shared lib)
- E2.2: Registry loader (fetch from plugin registry URL)
- E2.3: Manifest validation before loading
- E2.4: Checksum verification for plugin binaries

### E3: Execution Sandbox
- E3.1: WASM sandbox via wasmtime/wasmer
- E3.2: Capability grants (filesystem, network, subprocess)
- E3.3: Resource limits (memory, CPU time)
- E3.4: Plugin crash isolation (host not affected by plugin panic)

### E4: Plugin API
- E4.1: Host-to-plugin: invoke, configure, health-check
- E4.2: Plugin-to-host: emit events, request capabilities, log
- E4.3: Typed message protocol (JSON or MessagePack)

### E5: Registry
- E5.1: Plugin registry client (search, fetch, verify)
- E5.2: Local plugin cache
- E5.3: Plugin version pinning in thegent config
