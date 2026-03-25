//! # thegent-plugin-host
//!
//! Plugin host and loader for thegent agent system.
//!
//! ## Architecture
//!
//! This crate follows **Hexagonal Architecture** (Ports & Adapters) with **Clean Architecture** layers.
//!
//! ## xDD Methodologies Applied
//!
//! - **TDD**: Tests written first
//! - **DDD**: Bounded contexts for plugin management
//! - **SOLID**: Single responsibility per module
//! - **CQRS**: Separate command and query interfaces
//! - **EDA**: Domain events for state changes
//! - **SpecDD**: Formal specifications in `specs/` module

pub mod domain;
pub mod application;
pub mod ports;
pub mod adapters;

// Re-exports for convenience
pub use domain::entities::*;
pub use domain::value_objects::*;
pub use domain::events::*;
pub use application::commands::*;
pub use application::queries::*;
pub use application::use_cases::*;
