//! # Domain Events

use std::time::SystemTime;

/// Plugin lifecycle events
#[derive(Debug, Clone)]
pub enum PluginEvent {
    /// Plugin was loaded
    Loaded {
        name: String,
        version: String,
        timestamp: SystemTime,
    },
    /// Plugin was unloaded
    Unloaded {
        name: String,
        reason: String,
        timestamp: SystemTime,
    },
    /// Plugin failed to load
    LoadFailed {
        name: String,
        error: String,
        timestamp: SystemTime,
    },
    /// Plugin was enabled
    Enabled { name: String, timestamp: SystemTime },
    /// Plugin was disabled
    Disabled { name: String, timestamp: SystemTime },
}

impl PluginEvent {
    pub fn timestamp(&self) -> SystemTime {
        match self {
            Self::Loaded { timestamp, .. } => *timestamp,
            Self::Unloaded { timestamp, .. } => *timestamp,
            Self::LoadFailed { timestamp, .. } => *timestamp,
            Self::Enabled { timestamp, .. } => *timestamp,
            Self::Disabled { timestamp, .. } => *timestamp,
        }
    }
}
