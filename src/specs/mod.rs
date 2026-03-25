//! # Specifications (SpecDD)

/// Plugin Host Specification
///
/// ## Invariants
/// - Plugin names must be unique
/// - Plugin names must be alphanumeric with hyphens/underscores
/// - Plugins must have valid semantic versions
///
/// ## Operations
/// - install(name, version) -> Result<PluginId>
/// - uninstall(name) -> Result<()>
/// - enable(name) -> Result<()>
/// - disable(name) -> Result<()>
/// - list() -> Vec<Plugin>
///
/// ## Properties
/// - P1: install(A, V) succeeds implies get(A) == Some(Plugin{name=A, version=V})
/// - P2: uninstall(A) succeeds implies get(A) == None
/// - P3: Plugins are loaded in priority order
/// - P4: Disabled plugins are not executed
pub struct PluginHostSpec {}
