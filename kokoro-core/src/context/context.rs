use super::scope::{Mode, Resource, Scope};
use crate::context::scope::DynamicCache;
use crate::disposable::DisposableHandle;
use crate::schedule::{Schedule, WithNoneList, AROBS};
use crate::subscriber::Subscriber;
use std::ops::Deref;
use std::sync::Arc;

/// The heart of Kokoro
pub struct Context<T: Resource + ?Sized, M: Mode + 'static> {
    scope: Arc<Scope<T, M>>,
    global: Arc<M>,
    global_cache: Arc<DynamicCache>,
}

impl<R: Resource + ?Sized + 'static, M: Mode> Context<R, M> {
    /// Create a new Context
    #[inline(always)]
    pub fn create(scope: Arc<Scope<R, M>>, global: Arc<M>) -> Self {
        Self {
            scope,
            global,
            global_cache: Arc::new(DynamicCache::new()),
        }
    }
    /// Get the scope of the Context
    #[inline(always)]
    pub fn scope(&self) -> Arc<Scope<R, M>> {
        Arc::clone(&self.scope)
    }
    /// Place the current context in a new scope
    #[inline(always)]
    pub fn with<N: Resource + ?Sized>(&self, scope: Arc<Scope<N, M>>) -> Context<N, M> {
        Context {
            scope,
            global: self.global.clone(),
            global_cache: Arc::clone(&self.global_cache),
        }
    }
    /// Gets the schedule of the node in the current scope.
    ///
    /// Note: It is not a get-schedule that includes the parent node
    #[inline(always)]
    pub fn schedule(&self) -> Arc<Schedule<R, M>> {
        self.scope().schedule()
    }
    #[inline(always)]
    pub fn global(&self) -> &M {
        &self.global
    }
    /// Register a subscriber for the main channel
    #[inline]
    pub fn subscribe<Sub, Et>(&self, sub: Sub) -> DisposableHandle<WithNoneList<AROBS<R, M>, R, M>>
        where
            Sub: Subscriber<Et, R, M> + 'static + Send + Sync,
            Et: 'static + Sync + Send,
    {
        DisposableHandle::new(self.schedule().insert(sub))
    }
}

impl<R: Resource + 'static, M: Mode> Deref for Context<R, M> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.scope.as_ref().resource.as_ref()
    }
}
