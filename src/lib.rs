#![warn(missing_docs)]
#![doc = include_str!("../README.md")]

pub use kokoro_core as core;

/// Default export
pub mod prelude {
    #[cfg(feature = "dynamic-plugin")]
    pub use super::dynamic_plugin::*;
    #[cfg(feature = "macros")]
    pub use super::macros::*;
    pub use kokoro_core::context::*;
    pub use kokoro_core::disposable::*;
    pub use kokoro_core::event::*;
    pub use kokoro_core::schedule::*;
    pub use kokoro_core::subscriber::*;
    #[cfg(feature = "default-impl")]
    pub use kokoro_default_impl::{plugin::*, thread::*};
    #[cfg(feature = "flume-channel")]
    pub use kokoro_flume_channel::*;
}

#[cfg(feature = "default-impl")]
pub use kokoro_default_impl as default_impl;

/// The macros
#[cfg(feature = "macros")]
pub mod macros {
    pub use kokoro_macros::stable_sorted_event;
    #[cfg(feature = "dynamic-plugin")]
    pub use kokoro_macros::DynamicPlugin;
    pub use kokoro_macros::Event;
}

/// Dynamic plug-in capabilities
#[cfg(feature = "dynamic-plugin")]
pub mod dynamic_plugin {
    pub use kokoro_dynamic_plugin::*;
}

#[cfg(feature = "flume-channel")]
pub use kokoro_flume_channel as flume_channel;
