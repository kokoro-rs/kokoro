use std::marker::PhantomData;
use std::mem;
use std::ops::Deref;
use std::sync::{Arc, Mutex, OnceLock, Weak};

use dashmap::DashMap;
use rand::rngs::mock::StepRng;
use rand::Rng;
use rayon::prelude::*;

use crate::any::KAny;
use crate::avail::*;

impl<T: 'static + Send + Sync + KAny + ?Sized, Ps: Send + Sync + Clone> Avails<T, Ps> {
    /// 执行所有可用性函数
    pub fn run_all(&self, ctx: &Context<T, Ps>, ps: Ps) {
        self.0.par_iter_mut().for_each(|mut avail| {
            let from_id = *avail.key();
            if let Some(id) = ctx.call_from {
                if from_id == id {
                    return;
                }
            }
            (*avail).run(
                &Context {
                    raw: ctx.raw.clone(),
                    self_id: ctx.self_id,
                    call_from: Some(from_id),
                    _marker: PhantomData,
                },
                ps.clone(),
            );
        });
    }
}
pub struct Children<Ps>(DashMap<u64, Arc<RawContext<Ps>>>, Mutex<StepRng>);
impl<Ps> Children<Ps> {
    pub fn new() -> Self {
        Self(DashMap::new(), StepRng::new(0, 1).into())
    }
    pub fn add(&self, child: Arc<RawContext<Ps>>) -> u64 {
        let id = self.1.lock().unwrap().gen();
        self.0.insert(id, child);
        id
    }
    pub fn get(&self, id: &u64) -> Option<Arc<RawContext<Ps>>> {
        self.0.get(id).map(|a| Arc::clone(&a))
    }
    pub fn remove(&self, id: &u64) -> Option<Arc<RawContext<Ps>>> {
        self.0.remove(id).map(|a| a.1)
    }
}
pub struct ChildHandle<T: 'static>(pub u64, PhantomData<T>);
impl<T> From<ChildHandle<T>> for u64 {
    fn from(value: ChildHandle<T>) -> Self {
        value.0
    }
}
impl<T> From<u64> for ChildHandle<T> {
    fn from(value: u64) -> Self {
        Self(value, PhantomData)
    }
}

pub struct Context<T: KAny + 'static + ?Sized, Ps: Clone> {
    raw: Arc<RawContext<Ps>>,
    self_id: Option<u64>,
    call_from: Option<u128>,
    _marker: PhantomData<T>,
}
impl<T: KAny + 'static + ?Sized, Ps: Clone> Clone for Context<T, Ps> {
    fn clone(&self) -> Self {
        Self {
            raw: self.raw.clone(),
            self_id: self.self_id,
            call_from: self.call_from,
            _marker: PhantomData,
        }
    }
}
impl<T: KAny + 'static + ?Sized, Ps: Send + Sync + Clone> FnOnce<(Ps,)> for Context<T, Ps> {
    type Output = ();

    extern "rust-call" fn call_once(self, args: (Ps,)) -> Self::Output {
        self.recursive_avail(args.0);
    }
}
impl<T: KAny + 'static + ?Sized, Ps: Send + Sync + Clone> FnMut<(Ps,)> for Context<T, Ps> {
    extern "rust-call" fn call_mut(&mut self, args: (Ps,)) -> Self::Output {
        self.recursive_avail(args.0);
    }
}
impl<T: KAny + 'static + ?Sized, Ps: Send + Sync + Clone> Fn<(Ps,)> for Context<T, Ps> {
    extern "rust-call" fn call(&self, args: (Ps,)) -> Self::Output {
        self.recursive_avail(args.0);
    }
}
pub struct RawContext<Ps> {
    pub scope: Arc<dyn KAny>,
    pub children: Children<Ps>,
    pub parent: Weak<RawContext<Ps>>,
    pub avails: Avails<dyn KAny, Ps>,
    pub _effects: Box<[OnceLock<Box<dyn KAny>>]>,
}
impl<Ps: Send + Sync + Clone> RawContext<Ps> {
    pub fn new<T: KAny + 'static, S: Into<Arc<T>>>(scope: S) -> Self {
        Self {
            scope: scope.into(),
            children: Children::new(),
            parent: Weak::new(),
            avails: Avails::new(),
            _effects: Box::new([]),
        }
    }
}
pub trait RawContextExt<Ps: Clone> {
    unsafe fn downcast_unchecked<T: KAny + ?Sized>(
        self,
        self_id: Option<u64>,
        call_from: Option<u128>,
    ) -> Context<T, Ps>;
    fn with<T: KAny + 'static>(&self, scope: T) -> (Arc<RawContext<Ps>>, u64);
}
impl<Ps: Send + Sync + Clone> RawContextExt<Ps> for Arc<RawContext<Ps>> {
    unsafe fn downcast_unchecked<T: KAny + ?Sized>(
        self,
        self_id: Option<u64>,
        call_from: Option<u128>,
    ) -> Context<T, Ps> {
        Context {
            raw: Arc::clone(&self),
            self_id,
            call_from,
            _marker: PhantomData,
        }
    }
    fn with<T: KAny + 'static>(&self, scope: T) -> (Arc<RawContext<Ps>>, u64) {
        let raw = RawContext {
            scope: Arc::new(scope),
            children: Children::new(),
            parent: Arc::downgrade(self),
            avails: Avails::new(),
            _effects: Box::new([]),
        };
        let araw = Arc::new(raw);
        let id = self.children.add(araw.clone());
        (araw, id)
    }
}

impl<T: KAny + 'static, Ps: Send + Sync + Clone> Context<T, Ps> {
    pub fn new(scope: T) -> Self {
        Self {
            raw: Arc::new(RawContext::new(scope)),
            self_id: None,
            call_from: None,
            _marker: PhantomData,
        }
    }
    pub fn new_with_avails(scope: T, avails: Avails<T, Ps>) -> Context<T, Ps> {
        Context {
            raw: Arc::new(RawContext {
                scope: Arc::new(scope),
                children: Children::new(),
                parent: Weak::new(),
                avails: unsafe { mem::transmute(avails) },
                _effects: Box::new([]),
            }),
            self_id: None,
            call_from: None,
            _marker: PhantomData,
        }
    }
    pub fn scope(&self) -> &T {
        &self.raw.scope.as_ref().downcast_ref_unchecked()
    }
}
impl<T: KAny + 'static + ?Sized, Ps: Send + Sync + Clone> Context<T, Ps> {
    pub fn recursive_avail(&self, ps: Ps) {
        self.raw
            .avails
            .run_all(unsafe { self.upcast_ref() }, ps.clone());
        self.raw.children.0.par_iter().for_each(|child_raw| {
            let ctx: Context<dyn KAny, Ps> = Context {
                raw: child_raw.clone(),
                self_id: Some(*child_raw.key()),
                call_from: None,
                _marker: PhantomData,
            };
            ctx.recursive_avail(ps.clone());
        });
    }
    pub unsafe fn upcast_ref(&self) -> &Context<dyn KAny, Ps> {
        unsafe { mem::transmute(self) }
    }
    pub fn with<N: KAny>(&self, scope: N) -> ChildHandle<N> {
        let (_, id) = self.raw.with(scope);
        ChildHandle::from(id)
    }
    pub fn avails(&self) -> &Avails<T, Ps> {
        unsafe { self.raw.avails.upcast_ref() }
    }
    pub fn children_raw(&self) -> &Children<Ps> {
        &self.raw.children
    }
    pub fn get_child<N: Send + Sync>(&self, handel: &ChildHandle<N>) -> Option<Context<N, Ps>> {
        self.raw.children.get(&handel.0).map(|raw| Context {
            raw,
            self_id: Some(handel.0),
            call_from: None,
            _marker: PhantomData,
        })
    }
    pub fn into_raw(self) -> Arc<RawContext<Ps>> {
        self.raw
    }
    pub fn raw(&self) -> Arc<RawContext<Ps>> {
        self.raw.clone()
    }
    pub fn raw_ref(&self) -> &Arc<RawContext<Ps>> {
        &self.raw
    }
}

impl<T: KAny, Ps: Send + Sync + Clone> Deref for Context<T, Ps> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.scope()
    }
}
