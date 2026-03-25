//! # Dynamic Loading Adapter

use crate::domain::entities::Plugin;
use crate::ports::driven::PluginLoaderPort;
use std::path::Path;

/// Dynamic library plugin loader
pub struct DynamicPluginLoader;

impl DynamicPluginLoader {
    pub fn new() -> Self {
        Self
    }
}

impl Default for DynamicPluginLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginLoaderPort for DynamicPluginLoader {
    fn load(&self, path: &str) -> Result<Plugin, String> {
        let path = Path::new(path);
        if !path.exists() {
            return Err(format!("Plugin path does not exist: {}", path.display()));
        }

        // In production, this would use libloading crate
        // For now, create a placeholder plugin
        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        Ok(Plugin::new(
                    name.to_string(),
                    semver::Version::parse("0.1.0").map_err(|e| e.to_string())?,
                    "dynamic".to_string(),
                ))
    }
}
