use super::Context;
use crate::event::Event;
use crate::schedule::Schedule;
use dashmap::mapref::entry::Entry;
use dashmap::DashMap;
use rayon::prelude::*;
use std::any::Any;
use std::hash::Hash;
use std::mem::MaybeUninit;
use std::ops::Deref;
use std::sync::Arc;

/// Dynamic storage content
pub struct DynamicCache(DashMap<&'static str, Arc<dyn Any>>);
impl DynamicCache {
    /// Create a new dynamic cache
    pub fn new() -> Self {
        DynamicCache(DashMap::new())
    }
    /// Inserting a value
    pub fn insert(&self, key: &'static str, value: Arc<dyn Any>) -> Option<Arc<dyn Any>> {
        self.0.insert(key, value)
    }
    /// Getting a value
    pub fn get<T: 'static>(&self, key: &'static str) -> Option<Arc<T>> {
        if let Some(v) = self.0.get(key) {
            Some(Arc::clone(unsafe {
                &*(v.deref() as *const Arc<dyn Any> as *const Arc<T>)
            }))
        } else {
            None
        }
    }
    /// If present, the value is returned; otherwise, a default value is set
    ///
    /// Like or_insert_with
    pub fn default<T: 'static>(
        &self,
        key: &'static str,
        constructor: impl FnOnce() -> Arc<T>,
    ) -> Arc<T> {
        match self.0.entry(key) {
            Entry::Occupied(v) => {
                Arc::clone(unsafe { &*(v.get() as *const Arc<dyn Any> as *const Arc<T>) })
            }
            Entry::Vacant(v) => {
                let arc = constructor();
                v.insert(Arc::clone(&arc) as Arc<dyn Any>);
                arc
            }
        }
    }
}
impl Deref for DynamicCache {
    type Target = DashMap<&'static str, Arc<dyn Any>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// For tags that can be cached, impl Send and Sync will do this auto-impl
pub trait LocalCache: Send + Sync {}
/// Used to cache the scope of the context
pub struct Scope<T: LocalCache + ?Sized> {
    schedule: Arc<Schedule<T>>,
    subscopes: DashMap<ScopeId, Arc<dyn Triggerable + Send + Sync>>,
    /// Cached content
    pub cache: Arc<T>,
    /// Dynamic storage content
    dyn_cache: DynamicCache,
    ctx: MaybeUninit<Context<T>>,
}
impl<T: LocalCache + ?Sized> Drop for Scope<T> {
    fn drop(&mut self) {
        let _ = std::mem::replace(&mut self.ctx, MaybeUninit::uninit());
    }
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
        self.schedule()
            .trigger(e, unsafe { self.ctx.assume_init_ref() })
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
    /// Fetching dyn cache
    #[inline(always)]
    pub fn dyn_cache(&self) -> &DynamicCache {
        &self.dyn_cache
    }
    /// If you already have a context, use the create function to create a range
    #[inline(always)]
    pub fn create<N: LocalCache + 'static>(cache: Arc<T>, ctx: &Context<N>) -> Arc<Self> {
        let s = Arc::new(Self {
            schedule: Arc::new(Schedule::<T>::new()),
            subscopes: DashMap::new(),
            cache,
            dyn_cache: DynamicCache::new(),
            ctx: MaybeUninit::<Context<T>>::uninit(),
        });
        unsafe {
            let ctx_ptr = &s.ctx as *const MaybeUninit<Context<T>>;
            let _ = std::ptr::read(ctx_ptr);
            #[allow(invalid_reference_casting)]
            let ctx_mut = &mut *(ctx_ptr as *mut MaybeUninit<Context<T>>);
            ctx_mut.write(ctx.with(Arc::downgrade(&s)));
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
            subscopes: DashMap::new(),
            cache,
            dyn_cache: DynamicCache::new(),
            ctx: MaybeUninit::<Context<T>>::uninit(),
        });
        let ctx = f(Arc::clone(&s));
        unsafe {
            let ctx_ptr = &s.ctx as *const MaybeUninit<Context<T>>;
            let _ = std::ptr::read(ctx_ptr);
            #[allow(invalid_reference_casting)]
            let ctx_mut = &mut *(ctx_ptr as *mut MaybeUninit<Context<T>>);
            ctx_mut.write(ctx.with(Arc::downgrade(&s)));
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
    pub fn subscopes(&self) -> &DashMap<ScopeId, Arc<dyn Triggerable + Send + Sync>> {
        &self.subscopes
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
