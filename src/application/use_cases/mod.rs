//! # Use Cases

use crate::domain::entities::{Plugin, PluginId};
use crate::domain::events::PluginEvent;
use crate::ports::driven::{EventPublisherPort, PluginStoragePort};

/// Plugin management use case
pub struct PluginUseCase<S: PluginStoragePort, E: EventPublisherPort> {
    storage: S,
    events: E,
}

impl<S: PluginStoragePort, E: EventPublisherPort> PluginUseCase<S, E> {
    pub fn new(storage: S, events: E) -> Self {
        Self { storage, events }
    }

    pub fn install_plugin(&mut self, name: String, version: Option<String>) -> Result<PluginId, String> {
        let id = PluginId::new(&name);
        let ver = version.unwrap_or_else(|| "0.1.0".to_string());
        let plugin = Plugin::new(
            name.clone(),
            semver::Version::parse(&ver).map_err(|e| e.to_string())?,
            "system".to_string(),
        );
        self.storage.store(&plugin)?;
        self.events.publish(PluginEvent::Loaded {
            name: name.clone(),
            version: plugin.version().to_string(),
            timestamp: std::time::SystemTime::now(),
        })?;
        Ok(id)
    }

    pub fn uninstall_plugin(&mut self, name: &str) -> Result<(), String> {
        let id = PluginId::new(name);
        if !self.storage.exists(&id) {
            return Err(format!("Plugin {} not found", name));
        }
        self.storage.remove(&id)?;
        self.events.publish(PluginEvent::Unloaded {
            name: name.to_string(),
            reason: "Uninstalled".to_string(),
            timestamp: std::time::SystemTime::now(),
        })?;
        Ok(())
    }

    pub fn list_plugins(&self, _include_disabled: bool) -> Vec<Plugin> {
        self.storage.list()
    }

    pub fn get_plugin(&self, name: &str) -> Option<Plugin> {
        let id = PluginId::new(name);
        self.storage.get(&id)
    }
}
