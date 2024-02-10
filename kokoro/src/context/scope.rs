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

pub trait LocalCache: Send + Sync {}
pub struct Scope<T: LocalCache + ?Sized> {
    schedule: Arc<Schedule<T>>,
    subscopes: Arc<DashMap<ScopeId, Arc<dyn Triggerable + Send + Sync>>>,
    pub cache: Arc<T>,
    ctx: Option<Context<T>>,
    pub scope_id_gen: Mutex<ScopeIdGen<StepRng>>,
}
pub trait Triggerable {
    fn trigger(&self, e: Arc<dyn Event + Send + Sync>);
    fn trigger_recursive(&self, e: Arc<dyn Event + Send + Sync>);
}
impl LocalCache for dyn Triggerable + Send + Sync {}

unsafe impl<T: LocalCache + ?Sized> Send for Scope<T> {}
unsafe impl<T: LocalCache + ?Sized> Sync for Scope<T> {}
impl<T: LocalCache + ?Sized + 'static> Triggerable for Scope<T> {
    fn trigger(&self, e: Arc<dyn Event + Send + Sync>) {
        if let Some(ctx) = &self.ctx {
            self.schedule().trigger(e, ctx)
        } else {
            panic!("Where did your Context go?")
        }
    }
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
    pub fn cache(&self) -> Arc<T> {
        Arc::clone(&self.cache)
    }
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
    pub fn schedule(&self) -> Arc<Schedule<T>> {
        Arc::clone(&self.schedule)
    }
    pub fn subscopes(&self) -> Arc<DashMap<ScopeId, Arc<dyn Triggerable + Send + Sync>>> {
        Arc::clone(&self.subscopes)
    }
}

pub struct ScopeId {
    name: &'static str,
    pre_id: u64,
}
impl PartialEq for ScopeId {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.pre_id == other.pre_id && self.name == self.name
    }
}
impl Eq for ScopeId {}
impl ScopeId {
    #[inline]
    pub fn new(name: &'static str, id: u64) -> Self {
        Self { name, pre_id: id }
    }
}
impl Hash for ScopeId {
    #[inline]
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.pre_id);
        state.write(self.name.as_bytes())
    }
}
impl Clone for ScopeId {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            name: self.name,
            pre_id: self.pre_id,
        }
    }
}

pub struct ScopeIdGen<R: Rng> {
    rand: R,
}
impl<R: Rng> ScopeIdGen<R> {
    pub fn next(&mut self, name: &'static str) -> ScopeId {
        let num = self.rand.next_u64();
        ScopeId::new(name, num)
    }
}
impl<R: Rng> ScopeIdGen<R> {
    fn new(rand: R) -> Self {
        Self { rand }
    }
}
