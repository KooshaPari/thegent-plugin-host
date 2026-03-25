//! # Driving Ports (Primary)

/// CLI interface for plugin management
pub trait PluginCliPort {
    /// List all plugins
    fn list_plugins(&self);

    /// Install a plugin
    fn install_plugin(&self, name: &str) -> Result<(), String>;

    /// Uninstall a plugin
    fn uninstall_plugin(&self, name: &str) -> Result<(), String>;

    /// Enable a plugin
    fn enable_plugin(&self, name: &str) -> Result<(), String>;

    /// Disable a plugin
    fn disable_plugin(&self, name: &str) -> Result<(), String>;

    /// Show plugin info
    fn show_plugin(&self, name: &str) -> Result<(), String>;
}

/// HTTP API interface
pub trait PluginApiPort {
    /// Handle install request
    fn handle_install(&self, name: &str) -> Result<(), String>;

    /// Handle uninstall request
    fn handle_uninstall(&self, name: &str) -> Result<(), String>;

    /// Handle list request
    fn handle_list(&self) -> Vec<crate::domain::entities::Plugin>;
}
