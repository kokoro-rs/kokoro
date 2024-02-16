#![warn(missing_docs)]
//! Some useful implementations
//!
//! Like Context::default, Context::spwan

/// The implementation of the thread spawn for the Context
pub mod thread;
/// Implement pluggable for Context
pub mod plugin;
/// The implementation of the service for the Context
pub mod service;

