# Functional Requirements: thegent-plugin-host

## FR-PLH-001: Plugin Manifest
FR-PLH-001a: Every plugin SHALL provide a `plugin.toml` manifest declaring: id, name, version, entry_point, capabilities[], min_host_version.
FR-PLH-001b: The host SHALL reject plugins with malformed or missing manifests.
FR-PLH-001c: Capability declarations SHALL be validated against a known capability registry.

## FR-PLH-002: Plugin Loading
FR-PLH-002a: The host SHALL support loading plugins from local filesystem paths.
FR-PLH-002b: The host SHALL support loading plugins from a registry URL with version pinning.
FR-PLH-002c: Before loading, the host SHALL verify the plugin binary checksum against the manifest.
FR-PLH-002d: Plugin loading SHALL be async and non-blocking to the host event loop.

## FR-PLH-003: Plugin State Machine
FR-PLH-003a: Plugins SHALL transition through states: Unloaded -> Loading -> Active -> Suspended | Failed.
FR-PLH-003b: The host SHALL emit a domain event on each state transition.
FR-PLH-003c: A Failed plugin SHALL be quarantined and not automatically reloaded without explicit operator action.

## FR-PLH-004: WASM Sandbox
FR-PLH-004a: Plugins compiled to WebAssembly SHALL execute inside a wasmtime-based sandbox.
FR-PLH-004b: The sandbox SHALL enforce memory limits specified in the plugin manifest.
FR-PLH-004c: Filesystem, network, and subprocess access SHALL require explicit capability grants in the manifest AND host-side approval.
FR-PLH-004d: A plugin panic/trap SHALL not crash the host process.

## FR-PLH-005: Plugin Invocation
FR-PLH-005a: The host SHALL expose `invoke(pluginId, method, payload) -> Result<Response, PluginError>`.
FR-PLH-005b: Invocation SHALL be typed: payload and response use JSON or MessagePack encoding.
FR-PLH-005c: Invocations SHALL time out after a configurable duration (default: 30 seconds).

## FR-PLH-006: Plugin Events
FR-PLH-006a: Plugins SHALL be able to emit named events to the host via the plugin ABI.
FR-PLH-006b: The host SHALL route plugin events to registered host-side event handlers.
FR-PLH-006c: Unhandled plugin events SHALL be logged and discarded (not cause errors).

## FR-PLH-007: Plugin Registry
FR-PLH-007a: The host SHALL provide a registry client for searching and fetching plugins by name/version.
FR-PLH-007b: Fetched plugins SHALL be cached in a local plugin store directory.
FR-PLH-007c: thegent config SHALL support pinning plugin versions: `plugin_name = "1.2.3"`.
