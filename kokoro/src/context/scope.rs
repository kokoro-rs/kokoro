use super::Context;
use crate::event::Event;
use crate::schedule::Schedule;
use dashmap::DashMap;
use parking_lot::Mutex;
use rand::rngs::mock::StepRng;
use rand::Rng;
use rayon::prelude::*;
use std::hash::Hash;
use std::ptr;
use std::sync::Arc;

/// For tags that can be cached, impl Send and Sync will do this auto-impl
pub trait LocalCache: Send + Sync {}
/// Used to cache the scope of the context
pub struct Scope<T: LocalCache + ?Sized> {
    schedule: Arc<Schedule<T>>,
    subscopes: Arc<DashMap<ScopeId, Arc<dyn Triggerable + Send + Sync>>>,
    /// Cached content
    pub cache: Arc<T>,
    ctx: Option<Context<T>>,
    /// Used to generate consecutive Scopeids that do not repeat
    pub scope_id_gen: Mutex<ScopeIdGen<StepRng>>,
}
/// Can be triggered
pub trait Triggerable {
    /// All the subscribers triggered the current scope
    fn trigger(&self, e: Arc<dyn Event + Send + Sync>);
    /// Recursively triggers all subscribers of the current scope and descendant scope
    fn trigger_recursive(&self, e: Arc<dyn Event + Send + Sync>);
}
impl LocalCache for dyn Triggerable + Send + Sync {}

unsafe impl<T: LocalCache + ?Sized> Send for Scope<T> {}
unsafe impl<T: LocalCache + ?Sized> Sync for Scope<T> {}
impl<T: LocalCache + ?Sized + 'static> Triggerable for Scope<T> {
    #[inline(always)]
    fn trigger(&self, e: Arc<dyn Event + Send + Sync>) {
        if let Some(ctx) = &self.ctx {
            self.schedule().trigger(e, ctx)
        } else {
            panic!("Where did your Context go?")
        }
    }
    #[inline(always)]
    fn trigger_recursive(&self, e: Arc<dyn Event + Send + Sync>) {
        let ps = self.subscopes();
        ps.par_iter().for_each(|p| {
            let e = Arc::clone(&e);
            p.trigger_recursive(e);
        });
        self.trigger(e);
    }
}
impl<T: LocalCache + ?Sized + 'static> Scope<T> {
    /// Fetching cache
    #[inline(always)]
    pub fn cache(&self) -> Arc<T> {
        Arc::clone(&self.cache)
    }
    /// If you already have a context, use the create function to create a range
    #[inline(always)]
    pub fn create<N: LocalCache + 'static>(cache: Arc<T>, ctx: &Context<N>) -> Arc<Self> {
        let s = Arc::new(Self {
            schedule: Arc::new(Schedule::<T>::new()),
            subscopes: Arc::new(DashMap::new()),
            cache,
            ctx: None,
            scope_id_gen: Mutex::new(ScopeIdGen::new(StepRng::new(0, 1))),
        });
        unsafe {
            let ctx_ptr = &s.ctx as *const Option<Context<T>>;
            #[allow(invalid_reference_casting)]
            let ctx_ptr = &mut *(ctx_ptr as *mut Option<Context<T>>);
            ptr::write(ctx_ptr, Some(ctx.with(Arc::clone(&s))));
        }
        s
    }
    /// If you don't have a context, use the builder function to create the context and scope
    #[inline(always)]
    pub fn build<F: FnOnce(Arc<Self>) -> Context<T>>(
        cache: Arc<T>,
        f: F,
    ) -> (Arc<Self>, Context<T>) {
        let s = Arc::new(Self {
            schedule: Arc::new(Schedule::<T>::new()),
            subscopes: Arc::new(DashMap::new()),
            cache,
            ctx: None,
            scope_id_gen: Mutex::new(ScopeIdGen::new(StepRng::new(0, 1))),
        });
        let ctx = f(Arc::clone(&s));
        unsafe {
            let ctx_ptr = &s.ctx as *const Option<Context<T>>;
            #[allow(invalid_reference_casting)]
            let ctx_ptr = &mut *(ctx_ptr as *mut Option<Context<T>>);
            ptr::write(ctx_ptr, Some(ctx.with(Arc::clone(&s))));
        }
        (s, ctx)
    }
    /// Gets the schedule for the current scope
    #[inline(always)]
    pub fn schedule(&self) -> Arc<Schedule<T>> {
        Arc::clone(&self.schedule)
    }
    /// Get the scope of children
    #[inline(always)]
    pub fn subscopes(&self) -> Arc<DashMap<ScopeId, Arc<dyn Triggerable + Send + Sync>>> {
        Arc::clone(&self.subscopes)
    }
}
/// Used to mark the scope of the identifier
pub struct ScopeId {
    name: &'static str,
    pre_id: u64,
}
impl PartialEq for ScopeId {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        self.pre_id == other.pre_id && self.name == self.name
    }
}
impl Eq for ScopeId {}
impl ScopeId {
    /// Create a new identifier
    #[inline(always)]
    pub fn new(name: &'static str, id: u64) -> Self {
        Self { name, pre_id: id }
    }
}
impl Hash for ScopeId {
    #[inline(always)]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.pre_id);
        state.write(self.name.as_bytes())
    }
}
impl Clone for ScopeId {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            name: self.name,
            pre_id: self.pre_id,
        }
    }
}
/// Used to generate consecutive Scopeids that do not repeat
pub struct ScopeIdGen<R: Rng> {
    rand: R,
}
impl<R: Rng> ScopeIdGen<R> {
    #[inline(always)]
    /// Iterate to get a new identifier
    pub fn next(&mut self, name: &'static str) -> ScopeId {
        let num = self.rand.next_u64();
        ScopeId::new(name, num)
    }
}
impl<R: Rng> ScopeIdGen<R> {
    #[inline(always)]
    fn new(rand: R) -> Self {
        Self { rand }
    }
}
