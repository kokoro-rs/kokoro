use super::scope::{Mode, Resource, Scope};
use crate::context::scope::DynamicCache;
use crate::disposable::{dispose, Disposable, DisposableCache, DisposableHandle};
use crate::schedule::{Schedule, SubscriberHandle};
use crate::subscriber::Subscriber;
use parking_lot::Mutex;
use rayon::prelude::*;
use std::ops::Deref;
use std::sync::Arc;

/// The heart of Kokoro
pub struct Context<R: Resource + ?Sized, M: Mode + 'static> {
    scope: Arc<Scope<R, M>>,
    global: Arc<M>,
    global_cache: Arc<DynamicCache>,
    disposals: Arc<Mutex<Vec<DisposableCache>>>,
}

impl<R: Resource + ?Sized + 'static, M: Mode> Context<R, M> {
    /// Create a new Context
    #[inline(always)]
    pub fn create(scope: Arc<Scope<R, M>>, global: Arc<M>) -> Self {
        Self {
            scope,
            global,
            global_cache: Arc::new(DynamicCache::new()),
            disposals: Arc::new(Mutex::new(Vec::new())),
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
            disposals: Arc::clone(&self.disposals),
        }
    }
    /// Gets the schedule of the node in the current scope.
    ///
    /// Note: It is not a get-schedule that includes the parent node
    #[must_use]
    #[inline(always)]
    pub fn schedule(&self) -> Arc<Schedule<R, M>> {
        self.scope().schedule()
    }
    /// Get global
    #[inline(always)]
    pub fn global(&self) -> &M {
        &self.global
    }
    /// Get the global cache
    #[inline(always)]
    pub fn cache(&self) -> &DynamicCache {
        &self.global_cache
    }
    #[inline(always)]
    pub fn add_disposable<D: Disposable + Send + Sync + 'static>(&self, disposable: D) {
        let cache = DisposableCache::warp(disposable);
        self.disposals.lock().push(cache);
    }
    /// Register a subscriber for the main channel
    #[inline(always)]
    pub fn subscribe<Sub, const N: usize, Q, E>(
        &self,
        sub: Sub,
    ) -> DisposableHandle<SubscriberHandle<R, M>>
    where
        Sub: Subscriber<N, Q, E, R, M> + 'static + Send + Sync,
        Q: 'static + Sync + Send,
        E: 'static + Send + Sync,
    {
        DisposableHandle::new(self.schedule().insert(sub))
    }
    /// Get the dynamic reference to the resource
    #[inline(always)]
    pub fn dynref(&self) -> &Context<dyn Resource, M> {
        unsafe { &*(self as *const Context<R, M> as *const Context<dyn Resource, M>) }
    }
}

impl<R: Resource + ?Sized + 'static, M: Mode> Disposable for Context<R, M> {
    unsafe fn dispose(&mut self) {
        while let Some(handle) = self.disposals.lock().pop() {
            dispose(handle)
        }
    }
}

impl<R: Resource + 'static, M: Mode> Deref for Context<R, M> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        self.scope.as_ref().resource.as_ref()
    }
}
