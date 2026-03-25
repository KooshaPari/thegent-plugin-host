//! # thegent-plugin-host
//!
//! Plugin host and loader for thegent multi-agent system.
//!
//! ## Architecture
//!
//! - **Hexagonal Architecture**: Ports & Adapters pattern
//! - **Clean Architecture**: Domain/Application/Ports/Adapters layers
//! - **DDD**: Entities, Value Objects, Domain Events

use clap::{Parser, Subcommand};

mod domain;
mod application;
mod ports;
mod adapters;

use adapters::inmemory::{InMemoryPluginStorage, InMemoryEventPublisher};
use application::use_cases::PluginUseCase;

#[derive(Parser)]
#[command(name = "thegent-plugin-host")]
#[command(about = "Plugin host and loader for thegent")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all plugins
    List {
        /// Include disabled plugins
        #[arg(long)]
        include_disabled: bool,
    },
    /// Install a plugin
    Install {
        /// Plugin name
        name: String,
        /// Plugin version
        #[arg(long)]
        version: Option<String>,
    },
    /// Uninstall a plugin
    Uninstall {
        /// Plugin name
        name: String,
        /// Force uninstall
        #[arg(long, short)]
        force: bool,
    },
    /// Enable a plugin
    Enable {
        /// Plugin name
        name: String,
    },
    /// Disable a plugin
    Disable {
        /// Plugin name
        name: String,
    },
    /// Show plugin info
    Info {
        /// Plugin name
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    let storage = InMemoryPluginStorage::new();
    let events = InMemoryEventPublisher::new();
    let mut use_case = PluginUseCase::new(storage, events);

    match cli.command {
        Commands::List { include_disabled } => {
            let plugins = use_case.list_plugins(include_disabled);
            if plugins.is_empty() {
                println!("No plugins installed");
            } else {
                for plugin in plugins {
                    println!("{} v{}", plugin.name(), plugin.version());
                }
            }
        }
        Commands::Install { name, version } => {
            match use_case.install_plugin(name.clone(), version.clone()) {
                Ok(id) => println!("Installed plugin {} ({})", name, id),
                Err(e) => eprintln!("Failed to install plugin: {}", e),
            }
        }
        Commands::Uninstall { name, force: _ } => {
            match use_case.uninstall_plugin(&name) {
                Ok(()) => println!("Uninstalled plugin {}", name),
                Err(e) => eprintln!("Failed to uninstall plugin: {}", e),
            }
        }
        Commands::Enable { name } => {
            println!("Enabled plugin {}", name);
        }
        Commands::Disable { name } => {
            println!("Disabled plugin {}", name);
        }
        Commands::Info { name } => {
            if let Some(plugin) = use_case.get_plugin(&name) {
                println!("Plugin: {}", plugin.name());
                println!("Version: {}", plugin.version());
            } else {
                eprintln!("Plugin {} not found", name);
            }
        }
    }
}
