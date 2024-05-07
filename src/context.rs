use std::marker::PhantomData;
use std::mem;
use std::ops::Deref;
use std::sync::{Arc, Mutex, Weak};

use dashmap::DashMap;
use rand::rngs::mock::StepRng;
use rand::Rng;
use rayon::prelude::*;

use crate::any::{IStableAny, StableAny};
use crate::avail::{Avail, Availed, Params};

pub struct Avails<T: ?Sized, Ps>(DashMap<u128, Box<dyn Avail<T, Ps>>>, Mutex<StepRng>);
pub struct AvailHandle<T: IStableAny, Param: Params<T, Ps>, Func: FnMut<Param, Output = ()>, Ps>(
    pub u128,
    PhantomData<T>,
    PhantomData<Param>,
    PhantomData<Func>,
    PhantomData<Ps>,
);
impl<T, Ps> From<DashMap<u128, Box<dyn Avail<T, Ps>>>> for Avails<T, Ps> {
    fn from(value: DashMap<u128, Box<dyn Avail<T, Ps>>>) -> Self {
        Self(value, StepRng::new(0, 1).into())
    }
}
impl<T: IStableAny, Param: Params<T, Ps>, Func: FnMut<Param, Output = ()>, Ps>
    From<AvailHandle<T, Param, Func, Ps>> for u128
{
    fn from(value: AvailHandle<T, Param, Func, Ps>) -> Self {
        value.0
    }
}
impl<Ps> Avails<dyn IStableAny, Ps> {
    pub unsafe fn upcast<T: IStableAny + ?Sized>(self) -> Avails<T, Ps> {
        mem::transmute::<Avails<dyn IStableAny, Ps>, Avails<T, Ps>>(self)
    }
    pub unsafe fn upcast_ref<T: IStableAny + ?Sized>(&self) -> &Avails<T, Ps> {
        mem::transmute::<&Avails<dyn IStableAny, Ps>, &Avails<T, Ps>>(self)
    }
}
impl<T: 'static + Send + Sync + IStableAny + ?Sized, Ps: Send + Sync> Avails<T, Ps> {
    /// #### 创建一个可用性函数容器
    ///
    /// 其本质是一个`DashMap`对象，用于存储可用性对象。
    pub fn new() -> Self {
        Self(DashMap::new(), StepRng::new(0, 1).into())
    }
    /// 执行所有可用性函数
    pub fn run_all(&self, ctx: &Context<T, Ps>, ps: Arc<Ps>) {
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

impl<T: 'static + Send + Sync + IStableAny, Ps: 'static> Avails<T, Ps> {
    /// #### 添加一个可用性函数
    ///
    /// 这个函数可以将一个可用性对象添加到容器中，
    /// 该对象会在容器的`run_all`方法被调用时被执行。
    ///
    /// 这个函数返回一个`AvailHandle`，
    /// 可以用于删除该可用性函数。
    ///
    /// #### 参数
    /// * `avail` - 一个可用性对象，该对象必须实现`Availed` trait
    ///
    /// #### 用例
    /// ```rust
    /// let ctx = Contenxt::new(());
    /// let handle = ctx.avails.add(avail_fn);
    /// ctx.avails.run_all(&ctx); // 运行所有可用性函数
    /// ctx.avails.remove(&handle); // 删除可用性函数
    /// fn avail_fn() {
    ///     println!("hello world");    
    /// }
    /// ```
    pub fn add<Param, Func, A: Into<Availed<T, Param, Func, Ps>>>(
        &self,
        avail: A,
    ) -> AvailHandle<T, Param, Func, Ps>
    where
        Param: Params<T, Ps> + 'static,
        Func: FnMut<Param, Output = ()> + 'static,
    {
        let id = &avail as *const _ as u128;
        let id = id | &self.1.lock().unwrap().gen();
        self.0.insert(id, Box::new(avail.into()));
        AvailHandle(id, PhantomData, PhantomData, PhantomData, PhantomData)
    }
    /// #### 从容器中删除一个可用性函数
    ///
    /// 这个函数可以根据`AvailHandle`来删除一个可用性对象。
    ///
    /// 将会返回被删除的可用性对象，
    /// 如果删除失败（没有找到对应的可用性对象）则返回`None`。
    ///
    /// #### 参数
    /// * `id` - 一个`AvailHandle`对象，
    ///   该对象必须是容器中的一个可用性对象的唯一标识。
    ///
    /// #### 用例
    /// ```rust
    /// let ctx = Contenxt::new(());
    /// let handle = ctx.avails.add(avail_fn);
    /// ctx.avails.remove(&handle); // 删除可用性函数
    /// fn avail_fn() {
    ///     println!("hello world");    
    /// }
    /// ```
    pub fn remove<Param, Func>(
        &self,
        id: &AvailHandle<T, Param, Func, Ps>,
    ) -> Option<Box<dyn Avail<T, Ps> + 'static>>
    where
        Param: Params<T, Ps> + 'static,
        Func: FnMut<Param, Output = ()> + 'static,
    {
        self.0.remove(&id.0).map(|a| a.1)
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

pub struct Context<T: IStableAny + 'static + ?Sized, Ps> {
    raw: Arc<RawContext<Ps>>,
    self_id: Option<u64>,
    call_from: Option<u128>,
    _marker: PhantomData<T>,
}
impl<T: IStableAny + 'static + ?Sized, Ps> Clone for Context<T, Ps> {
    fn clone(&self) -> Self {
        Self {
            raw: self.raw.clone(),
            self_id: self.self_id,
            call_from: self.call_from,
            _marker: PhantomData,
        }
    }
}
impl<T: IStableAny + 'static + ?Sized, Ps: Send + Sync> FnOnce<(Ps,)> for Context<T, Ps> {
    type Output = ();

    extern "rust-call" fn call_once(self, args: (Ps,)) -> Self::Output {
        self.recursive_avail(Arc::new(args.0));
    }
}
impl<T: IStableAny + 'static + ?Sized, Ps: Send + Sync> FnMut<(Ps,)> for Context<T, Ps> {
    extern "rust-call" fn call_mut(&mut self, args: (Ps,)) -> Self::Output {
        self.recursive_avail(Arc::new(args.0));
    }
}
impl<T: IStableAny + 'static + ?Sized, Ps: Send + Sync> Fn<(Ps,)> for Context<T, Ps> {
    extern "rust-call" fn call(&self, args: (Ps,)) -> Self::Output {
        self.recursive_avail(Arc::new(args.0));
    }
}
impl<T: IStableAny + 'static + ?Sized, Ps: Send + Sync> FnOnce<(Arc<Ps>,)> for Context<T, Ps> {
    type Output = ();

    extern "rust-call" fn call_once(self, args: (Arc<Ps>,)) -> Self::Output {
        self.recursive_avail(args.0);
    }
}
impl<T: IStableAny + 'static + ?Sized, Ps: Send + Sync> FnMut<(Arc<Ps>,)> for Context<T, Ps> {
    extern "rust-call" fn call_mut(&mut self, args: (Arc<Ps>,)) -> Self::Output {
        self.recursive_avail(args.0);
    }
}
impl<T: IStableAny + 'static + ?Sized, Ps: Send + Sync> Fn<(Arc<Ps>,)> for Context<T, Ps> {
    extern "rust-call" fn call(&self, args: (Arc<Ps>,)) -> Self::Output {
        self.recursive_avail(args.0);
    }
}
pub struct RawContext<Ps> {
    pub scope: Arc<dyn IStableAny>,
    pub children: Children<Ps>,
    pub parent: Weak<RawContext<Ps>>,
    pub avails: Avails<dyn IStableAny, Ps>,
}
impl<Ps: Send + Sync> RawContext<Ps> {
    pub fn new<T: IStableAny + 'static>(scope: T) -> Self {
        Self {
            scope: Arc::new(scope),
            children: Children::new(),
            parent: Weak::new(),
            avails: Avails::new(),
        }
    }
}
pub trait RawContextExt<Ps> {
    unsafe fn downcast_unchecked<T: IStableAny>(
        &self,
        self_id: Option<u64>,
        call_from: Option<u128>,
    ) -> Context<T, Ps>;
    fn with<T: IStableAny + 'static>(&self, scope: T) -> (Arc<RawContext<Ps>>, u64);
}
impl<Ps: Send + Sync> RawContextExt<Ps> for Arc<RawContext<Ps>> {
    unsafe fn downcast_unchecked<T: IStableAny>(
        &self,
        self_id: Option<u64>,
        call_from: Option<u128>,
    ) -> Context<T, Ps> {
        Context {
            raw: Arc::clone(self),
            self_id,
            call_from,
            _marker: PhantomData,
        }
    }
    fn with<T: IStableAny + 'static>(&self, scope: T) -> (Arc<RawContext<Ps>>, u64) {
        let raw = RawContext {
            scope: Arc::new(scope),
            children: Children::new(),
            parent: Arc::downgrade(self),
            avails: Avails::new(),
        };
        let araw = Arc::new(raw);
        let id = self.children.add(araw.clone());
        (araw, id)
    }
}

impl<T: IStableAny + 'static, Ps: Send + Sync> Context<T, Ps> {
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
impl<T: IStableAny + 'static + ?Sized, Ps: Send + Sync> Context<T, Ps> {
    pub fn recursive_avail(&self, ps: Arc<Ps>) {
        self.raw
            .avails
            .run_all(unsafe { self.upcast_ref() }, ps.clone());
        self.raw.children.0.par_iter().for_each(|child_raw| {
            let ctx: Context<dyn IStableAny, Ps> = Context {
                raw: child_raw.clone(),
                self_id: Some(*child_raw.key()),
                call_from: None,
                _marker: PhantomData,
            };
            ctx.recursive_avail(ps.clone());
        });
    }
    pub unsafe fn upcast_ref(&self) -> &Context<dyn IStableAny, Ps> {
        unsafe { mem::transmute(self) }
    }
    pub fn with<N: IStableAny>(&self, scope: N) -> ChildHandle<N> {
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
}

impl<T: IStableAny, Ps: Send + Sync> Deref for Context<T, Ps> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.scope()
    }
}
