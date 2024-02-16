use crate::{
    context::{
        scope::Scope,
        Context,
    },
    event::Event,
};
use std::sync::Arc;
use crate::context::scope::Mode;

/// Event Send Sync
pub type SSE = dyn Event + Send + Sync;

/// The root's cache
pub struct Root {}


impl Default for Root {
    #[inline(always)]
    fn default() -> Self {
        Root {}
    }
}

impl<M: Mode> Context<Root, M> {
    /// Create a root context
    #[inline(always)]
    pub fn new(global: Arc<M>) -> Self {
        let scope = Scope::create(Box::new(Root::default()));
        Context::create(Arc::new(scope), global)
    }
}





