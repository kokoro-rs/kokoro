use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::{Arc, Mutex, Weak};

use dashmap::DashMap;
use rand::rngs::mock::StepRng;
use rand::Rng;
use rayon::prelude::*;

use crate::any::StableAny;
use crate::avail::{Avail, Availed, Params};

pub struct Avails<T: ?Sized>(DashMap<u128, Box<dyn Avail<T>>>, Mutex<StepRng>);
pub struct AvailHandle<T: StableAny, Param: Params<T>, Func: FnMut<Param, Output = ()>>(
    pub u128,
    PhantomData<T>,
    PhantomData<Param>,
    PhantomData<Func>,
);
impl<T> From<DashMap<u128, Box<dyn Avail<T>>>> for Avails<T> {
    fn from(value: DashMap<u128, Box<dyn Avail<T>>>) -> Self {
        Self(value, StepRng::new(0, 1).into())
    }
}
impl<T: StableAny, Param: Params<T>, Func: FnMut<Param, Output = ()>>
    From<AvailHandle<T, Param, Func>> for u128
{
    fn from(value: AvailHandle<T, Param, Func>) -> Self {
        value.0
    }
}

impl<T: 'static + Send + Sync + StableAny> Avails<T> {
    pub fn add<Param, Func, A: Into<Availed<T, Param, Func>>>(
        &self,
        avail: A,
    ) -> AvailHandle<T, Param, Func>
    where
        Param: Params<T> + 'static,
        Func: FnMut<Param, Output = ()> + 'static,
    {
        let id = &avail as *const _ as u128;
        let id = id | &self.1.lock().unwrap().gen();
        self.0.insert(id, Box::new(avail.into()));
        AvailHandle(id, PhantomData, PhantomData, PhantomData)
    }
    pub fn get<'a, Param, Func>(
        &'a self,
        id: &'a AvailHandle<T, Param, Func>,
    ) -> Option<dashmap::mapref::one::RefMut<'a, u128, Box<(dyn Avail<T> + 'a)>>>
    where
        Param: Params<T> + 'static,
        Func: FnMut<Param, Output = ()> + 'static,
    {
        unsafe { std::mem::transmute(self.0.get_mut(&id.0)) }
    }
    pub fn remove<Param, Func>(
        &self,
        id: &AvailHandle<T, Param, Func>,
    ) -> Option<Box<dyn Avail<T> + 'static>>
    where
        Param: Params<T> + 'static,
        Func: FnMut<Param, Output = ()> + 'static,
    {
        self.0.remove(&id.0).map(|a| a.1)
    }
    pub fn run_all(&self, ctx: &Context<T>) {
        self.0
            .par_iter_mut()
            .for_each(|mut avail| (*avail).run(ctx));
    }
}

pub struct Children(DashMap<u64, Arc<Context<dyn StableAny>>>, Mutex<StepRng>);
impl Children {
    pub fn new() -> Self {
        Self(DashMap::new(), StepRng::new(0, 1).into())
    }
    pub fn add<T: StableAny>(&self, child: Arc<Context<T>>) -> ChildHandle<T> {
        let id = self.1.lock().unwrap().gen();
        self.0.insert(id, unsafe { std::mem::transmute(child) });
        ChildHandle(id, PhantomData)
    }
    pub fn get<T: 'static + StableAny>(
        &self,
        id: &ChildHandle<T>,
    ) -> Option<dashmap::mapref::one::RefMut<'static, u64, Arc<Context<T>>>> {
        unsafe { std::mem::transmute(self.0.get_mut(&id.0)) }
    }
}
pub struct ChildHandle<T: 'static>(pub u64, PhantomData<T>);
impl<T> From<ChildHandle<T>> for u64 {
    fn from(value: ChildHandle<T>) -> Self {
        value.0
    }
}

pub struct Context<T: StableAny + 'static + ?Sized> {
    pub avails: Avails<T>,
    pub scope: Arc<T>,
    pub parent: Weak<Context<dyn StableAny>>,
    pub children: Children,
}

impl<T: StableAny + 'static> Context<T> {
    pub fn new(scope: T) -> Arc<Self> {
        let s = Self {
            avails: DashMap::new().into(),
            scope: Arc::new(scope),
            parent: Weak::new(),
            children: Children::new(),
        };
        Arc::new(s)
    }
    pub fn new_with_avails(scope: T, avails: Avails<T>) -> Arc<Self> {
        let s = Self {
            avails,
            scope: Arc::new(scope),
            parent: Weak::new(),
            children: Children::new(),
        };
        Arc::new(s)
    }
}
pub trait ContextExt<T: 'static> {
    fn with<N: StableAny>(&self, scope: N) -> ChildHandle<N>;
}
impl<T: 'static + StableAny> ContextExt<T> for Arc<Context<T>> {
    fn with<N: StableAny>(&self, scope: N) -> ChildHandle<N> {
        let s: Context<N> = Context {
            avails: DashMap::new().into(),
            scope: Arc::new(scope),
            parent: Arc::downgrade(unsafe { std::mem::transmute(self) }),
            children: Children::new(),
        };
        self.children.add(Arc::new(s))
    }
}

impl<T: StableAny> Deref for Context<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.scope.as_ref()
    }
}
