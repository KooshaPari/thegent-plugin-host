//! Integration tests

use thegent_plugin_host::adapters::inmemory::{InMemoryPluginStorage, InMemoryEventPublisher};
use thegent_plugin_host::application::use_cases::PluginUseCase;

#[test]
fn test_install_plugin() {
    let storage = InMemoryPluginStorage::new();
    let events = InMemoryEventPublisher::new();
    let mut use_case = PluginUseCase::new(storage, events);

    let result = use_case.install_plugin("test-plugin".to_string(), Some("1.0.0".to_string()));
    assert!(result.is_ok());
}

#[test]
fn test_uninstall_plugin() {
    let storage = InMemoryPluginStorage::new();
    let events = InMemoryEventPublisher::new();
    let mut use_case = PluginUseCase::new(storage, events);

    use_case.install_plugin("test-plugin".to_string(), None).unwrap();
    let result = use_case.uninstall_plugin("test-plugin");
    assert!(result.is_ok());
}

#[test]
fn test_list_plugins() {
    let storage = InMemoryPluginStorage::new();
    let events = InMemoryEventPublisher::new();
    let use_case = PluginUseCase::new(storage, events);

    let plugins = use_case.list_plugins(false);
    assert!(plugins.is_empty());
}
