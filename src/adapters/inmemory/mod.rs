//! # In-Memory Adapters

use crate::domain::entities::{Plugin, PluginId};
use crate::domain::events::PluginEvent;
use crate::ports::driven::{EventPublisherPort, PluginStoragePort};
use std::collections::HashMap;

/// In-memory plugin storage
pub struct InMemoryPluginStorage {
    plugins: HashMap<PluginId, Plugin>,
}

impl InMemoryPluginStorage {
    pub fn new() -> Self {
        Self {
            plugins: HashMap::new(),
        }
    }
}

impl Default for InMemoryPluginStorage {
    fn default() -> Self {
        Self::new()
    }
}

impl PluginStoragePort for InMemoryPluginStorage {
    fn store(&mut self, plugin: &Plugin) -> Result<(), String> {
        let id = plugin.id().clone();
        self.plugins.insert(id, plugin.clone());
        Ok(())
    }

    fn remove(&mut self, id: &PluginId) -> Result<(), String> {
        self.plugins.remove(id);
        Ok(())
    }

    fn get(&self, id: &PluginId) -> Option<Plugin> {
        self.plugins.get(id).cloned()
    }

    fn list(&self) -> Vec<Plugin> {
        self.plugins.values().cloned().collect()
    }

    fn exists(&self, id: &PluginId) -> bool {
        self.plugins.contains_key(id)
    }
}

/// In-memory event publisher
pub struct InMemoryEventPublisher {
    handlers: Vec<Box<dyn Fn(PluginEvent) + Send>>,
    events: Vec<PluginEvent>,
}

impl InMemoryEventPublisher {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
            events: Vec::new(),
        }
    }
}

impl Default for InMemoryEventPublisher {
    fn default() -> Self {
        Self::new()
    }
}

impl EventPublisherPort for InMemoryEventPublisher {
    fn publish(&mut self, event: PluginEvent) -> Result<(), String> {
        self.events.push(event.clone());
        for handler in &self.handlers {
            handler(event.clone());
        }
        Ok(())
    }

    fn subscribe(&mut self, handler: Box<dyn Fn(PluginEvent) + Send>) {
        self.handlers.push(handler);
    }
}
