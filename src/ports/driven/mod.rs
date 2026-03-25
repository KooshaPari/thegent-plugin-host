//! # Driven Ports (Secondary)

use crate::domain::entities::{Plugin, PluginId};
use crate::domain::events::PluginEvent;

/// Port for plugin storage
pub trait PluginStoragePort {
    /// Store a plugin
    fn store(&mut self, plugin: &Plugin) -> Result<(), String>;

    /// Remove a plugin
    fn remove(&mut self, id: &PluginId) -> Result<(), String>;

    /// Get a plugin by ID
    fn get(&self, id: &PluginId) -> Option<Plugin>;

    /// List all plugins
    fn list(&self) -> Vec<Plugin>;

    /// Check if plugin exists
    fn exists(&self, id: &PluginId) -> bool;
}

/// Port for plugin loading
pub trait PluginLoaderPort {
    /// Load a plugin from path
    fn load(&self, path: &str) -> Result<Plugin, String>;
}

/// Port for event publishing
pub trait EventPublisherPort {
    /// Publish an event
    fn publish(&mut self, event: PluginEvent) -> Result<(), String>;

    /// Subscribe to events
    fn subscribe(&mut self, handler: Box<dyn Fn(PluginEvent) + Send>);
}
