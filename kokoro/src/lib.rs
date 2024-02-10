#![cfg_attr(feature = "nightly", feature(unboxed_closures))]
#![cfg_attr(feature = "nightly", feature(fn_traits))]
// #![warn(missing_docs)]
#![doc = include_str!("../README.md")]
/// The heart of Kokoro is the Context
pub mod context;
/// Some useful implementations
pub mod default_implement;
/// Abstract what can be disposed
pub mod disposable;
/// Traits of the event to be publish
pub mod event;
/// A schedule that each node will have
pub mod schedule;
/// Subscribers, which are executed when a message is received
pub mod subscriber;
pub use flume::unbounded as mpsc;
/// Default export
pub mod prelude {
    pub use super::context::*;
    pub use super::default_implement::{base::*, thread::*};
    pub use super::disposable::*;
    pub use super::event::*;
    pub use super::schedule::*;
    pub use super::subscriber::*;
    pub use kokoro_macros::*;
}
pub use kokoro_macros::*;
