//! # Domain Entities
//!
//! Core business objects with identity.

use std::fmt;

use chrono::{DateTime, Utc};
use semver::Version;
use uuid::Uuid;

/// Plugin identifier
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PluginId(String);

impl PluginId {
    pub fn new(name: &str) -> Self {
        Self(name.to_lowercase().replace('-', "_"))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for PluginId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Plugin metadata and identity
#[derive(Debug, Clone)]
pub struct Plugin {
    /// Unique identifier
    pub id: Uuid,
    /// Plugin name (unique within host)
    pub name: String,
    /// Semantic version
    pub version: Version,
    /// Plugin description
    pub description: String,
    /// Plugin author
    pub author: String,
    /// Entry point for the plugin
    pub entry_point: String,
    /// Current state
    pub state: PluginState,
    /// When loaded
    pub loaded_at: Option<DateTime<Utc>>,
    /// Plugin capabilities
    pub capabilities: Vec<Capability>,
}

impl Plugin {
    pub fn id(&self) -> PluginId {
        PluginId::new(&self.name)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn new(name: impl Into<String>, version: Version, author: impl Into<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name: name.into(),
            version,
            description: String::new(),
            author: author.into(),
            entry_point: String::new(),
            state: PluginState::Discovered,
            loaded_at: None,
            capabilities: Vec::new(),
        }
    }

    pub fn load(&mut self) {
        self.state = PluginState::Loaded;
        self.loaded_at = Some(Utc::now());
    }

    pub fn unload(&mut self) {
        self.state = PluginState::Unloaded;
        self.loaded_at = None;
    }

    pub fn enable(&mut self) {
        if matches!(self.state, PluginState::Loaded) {
            self.state = PluginState::Enabled;
        }
    }

    pub fn disable(&mut self) {
        if matches!(self.state, PluginState::Enabled) {
            self.state = PluginState::Loaded;
        }
    }
}

/// Plugin lifecycle state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PluginState {
    /// Plugin discovered but not loaded
    Discovered,
    /// Plugin loaded into memory
    Loaded,
    /// Plugin loaded and enabled
    Enabled,
    /// Plugin temporarily disabled
    Unloaded,
}

/// Plugin capability definition
#[derive(Debug, Clone)]
pub struct Capability {
    /// Capability name
    pub name: String,
    /// Version requirement
    pub version_req: String,
}

/// Plugin manifest (from plugin.toml)
#[derive(Debug, Clone)]
pub struct PluginManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub entry: String,
    pub capabilities: Vec<String>,
    pub dependencies: Vec<PluginDependency>,
}

/// Plugin dependency
#[derive(Debug, Clone)]
pub struct PluginDependency {
    pub name: String,
    pub version_req: String,
}

/// Plugin error types
#[derive(Debug, Clone, thiserror::Error)]
pub enum PluginError {
    #[error("Plugin not found: {0}")]
    NotFound(String),

    #[error("Plugin already loaded: {0}")]
    AlreadyLoaded(String),

    #[error("Plugin not loaded: {0}")]
    NotLoaded(String),

    #[error("Version mismatch: expected {expected}, got {actual}")]
    VersionMismatch { expected: String, actual: String },

    #[error("Dependency not satisfied: {0}")]
    DependencyNotSatisfied(String),

    #[error("Load error: {0}")]
    LoadError(String),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_plugin_lifecycle() {
        let mut plugin = Plugin::new("test-plugin", Version::new(1, 0, 0), "test");
        assert!(matches!(plugin.state, PluginState::Discovered));

        plugin.load();
        assert!(matches!(plugin.state, PluginState::Loaded));
        assert!(plugin.loaded_at.is_some());

        plugin.enable();
        assert!(matches!(plugin.state, PluginState::Enabled));

        plugin.disable();
        assert!(matches!(plugin.state, PluginState::Loaded));

        plugin.unload();
        assert!(matches!(plugin.state, PluginState::Unloaded));
        assert!(plugin.loaded_at.is_none());
    }

    #[test]
    fn test_plugin_creation() {
        let plugin = Plugin::new("my-plugin", Version::new(2, 1, 0), "author");
        assert_eq!(plugin.name, "my-plugin");
        assert_eq!(plugin.author, "author");
    }
}
