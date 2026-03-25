//! # Queries

/// List plugins query
#[derive(Debug, Clone)]
pub struct ListPluginsQuery {
    pub include_disabled: bool,
}

/// Get plugin query
#[derive(Debug, Clone)]
pub struct GetPluginQuery {
    pub name: String,
}

/// Search plugins query
#[derive(Debug, Clone)]
pub struct SearchPluginsQuery {
    pub pattern: String,
}
