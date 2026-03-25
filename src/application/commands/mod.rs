//! # Commands

/// Install plugin command
#[derive(Debug, Clone)]
pub struct InstallPluginCommand {
    pub name: String,
    pub version: Option<String>,
    pub path: Option<String>,
}

/// Uninstall plugin command
#[derive(Debug, Clone)]
pub struct UninstallPluginCommand {
    pub name: String,
    pub force: bool,
}

/// Enable plugin command
#[derive(Debug, Clone)]
pub struct EnablePluginCommand {
    pub name: String,
}

/// Disable plugin command
#[derive(Debug, Clone)]
pub struct DisablePluginCommand {
    pub name: String,
}
