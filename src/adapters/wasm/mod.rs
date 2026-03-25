//! # WASM Adapter
//!
//! WebAssembly plugin loader.
//!
//! ## Architecture
//!
//! This adapter implements the `PluginLoaderPort` using WASM runtime.

use crate::domain::entities::Plugin;
use crate::ports::driven::PluginLoaderPort;
use std::path::Path;

/// WASM plugin loader
pub struct WasmPluginLoader;

impl WasmPluginLoader {
    pub fn new() -> Self {
        Self
    }
}

impl Default for WasmPluginLoader {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginLoaderPort for WasmPluginLoader {
    fn load(&self, path: &str) -> Result<Plugin, String> {
        let path = Path::new(path);
        if !path.exists() {
            return Err(format!("WASM plugin path does not exist: {}", path.display()));
        }

        let name = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown");

        Ok(Plugin::new(
            name.to_string(),
            semver::Version::parse("0.1.0").map_err(|e| e.to_string())?,
            "wasm".to_string(),
        ))
    }
}
