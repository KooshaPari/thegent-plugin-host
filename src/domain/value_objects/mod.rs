//! # Value Objects
//!
//! Immutable objects defined by their attributes.

use std::path::PathBuf;

/// Plugin name (validated)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PluginName(String);

impl PluginName {
    pub fn new(name: impl Into<String>) -> Result<Self, &'static str> {
        let name = name.into();
        if name.is_empty() {
            return Err("Plugin name cannot be empty");
        }
        if !name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
            return Err("Plugin name must be alphanumeric with hyphens/underscores");
        }
        Ok(Self(name))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Plugin path
#[derive(Debug, Clone)]
pub struct PluginPath(PathBuf);

impl PluginPath {
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self(path.into())
    }

    pub fn as_path(&self) -> &PathBuf {
        &self.0
    }
}

/// Load priority
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Priority(u8);

impl Priority {
    pub const LOW: Priority = Priority(0);
    pub const NORMAL: Priority = Priority(128);
    pub const HIGH: Priority = Priority(200);
    pub const CRITICAL: Priority = Priority(255);

    pub fn new(value: u8) -> Self {
        Self(value)
    }

    pub fn value(&self) -> u8 {
        self.0
    }
}

impl Default for Priority {
    fn default() -> Self {
        Self::NORMAL
    }
}
