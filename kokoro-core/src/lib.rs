#![cfg_attr(feature = "nightly", feature(unboxed_closures))]
#![cfg_attr(feature = "nightly", feature(fn_traits))]
#![warn(missing_docs)]
#![warn(clippy::inline_always)]
#![doc = include_str!("../README.md")]
/// The heart of Kokoro is the Context
pub mod context;
/// Some useful implementations
/// Abstract what can be disposed
pub mod disposable;
/// Traits of the event to be publish
pub mod event;
/// A schedule that each node will have
pub mod schedule;
/// Subscribers, which are executed when a message is received
pub mod subscriber;
