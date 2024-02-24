use super::Context;
use crate::event::Event;
use crate::schedule::Schedule;
use dashmap::mapref::entry::Entry;
use dashmap::DashMap;
use rayon::prelude::*;
use std::any::Any;
use std::hash::Hash;
use std::ops::Deref;
use std::sync::Arc;

/// Dynamic storage content
pub struct DynamicCache(DashMap<&'static str, Arc<dyn Any>>);

unsafe impl Send for DynamicCache {}

unsafe impl Sync for DynamicCache {}

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
/// `Resource` is a trait that represents a resource that can be shared across threads safely.
pub trait Resource: Send + Sync {}

impl<T: Send + Sync> Resource for T {}
/// `Mode` is a trait that anything that can be used as a mode should implement
pub trait Mode: Send + Sync {}

impl<T: Send + Sync> Mode for T {}

/// Used to cache the scope of the context
pub struct Scope<R: Resource + ?Sized, M: Mode + 'static> {
    schedule: Arc<Schedule<R, M>>,
    subscopes: DashMap<ScopeId, Box<dyn Triggerable<M> + Send + Sync>>,
    /// Cached content
    pub resource: Arc<R>,
    /// Dynamic storage content
    cache: DynamicCache,
}

/// Can be triggered
pub trait Triggerable<M: Mode> {
    /// All the subscribers triggered the current scope
    fn trigger(&self, e: Arc<dyn Event + Send + Sync>, ctx: &Context<dyn Resource, M>);
    /// Recursively triggers all subscribers of the current scope and descendant scope
    fn trigger_recursive(
        &self,
        e: Arc<dyn Event + Send + Sync>,
        ctx: &Context<dyn Resource, M>,
    );
}

impl<M: Mode> Resource for dyn Triggerable<M> + Send + Sync {}

unsafe impl<T: Resource + ?Sized, M: Mode> Send for Scope<T, M> {}

unsafe impl<T: Resource + ?Sized, M: Mode> Sync for Scope<T, M> {}

impl<R: Resource + ?Sized + 'static, M: Mode> Triggerable<M> for Arc<Scope<R, M>> {
    #[inline(always)]
    fn trigger(&self, e: Arc<dyn Event + Send + Sync>, ctx: &Context<dyn Resource, M>) {
        self.schedule().trigger(e, &ctx.with(Arc::clone(&self)));
    }
    #[inline(always)]
    fn trigger_recursive(
        &self,
        e: Arc<dyn Event + Send + Sync>,
        ctx: &Context<dyn Resource, M>,
    ) {
        let ps = self.subscopes();
        ps.par_iter().for_each(|p| {
            let e = Arc::clone(&e);
            p.trigger_recursive(e, ctx);
        });
        self.trigger(e, ctx);
    }
}

impl<R: Resource + ?Sized + 'static, M: Mode> Scope<R, M> {
    /// Fetching cache
    #[inline(always)]
    pub fn resource(&self) -> &R {
        self.resource.as_ref()
    }
    /// Fetching dyn cache
    #[inline(always)]
    pub fn cache(&self) -> &DynamicCache {
        &self.cache
    }
    /// Create a Scope
    #[inline(always)]
    pub fn create(resource: Arc<R>) -> Self {
        Self {
            schedule: Arc::new(Schedule::<R, M>::new()),
            subscopes: DashMap::new(),
            resource,
            cache: DynamicCache::new(),
        }
    }
    /// Gets the schedule for the current scope
    #[inline(always)]
    pub fn schedule(&self) -> Arc<Schedule<R, M>> {
        Arc::clone(&self.schedule)
    }
    /// Get the scope of children
    #[inline(always)]
    pub fn subscopes(&self) -> &DashMap<ScopeId, Box<dyn Triggerable<M> + Send + Sync>> {
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
