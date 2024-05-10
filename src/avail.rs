use dashmap::DashMap;
use rand::rngs::mock::StepRng;
use rand::Rng;
use std::mem;

use crate::{any::KAny, context::Context};
use std::{
    marker::{PhantomData, Tuple},
    sync::Mutex,
};
pub trait Avail<T: KAny + ?Sized, Ps: Clone>: Send + Sync {
    fn run(&mut self, ctx: &Context<T, Ps>, ps: Ps);
}

pub trait Params<T: KAny, Ps: Clone>: Tuple {
    fn get(ctx: &Context<T, Ps>, ps: Ps) -> Self;
}

impl<T: KAny, Ps: Clone> Params<T, Ps> for () {
    fn get(_ctx: &Context<T, Ps>, _ps: Ps) -> Self {
        ()
    }
}

impl<T: KAny, Ps: Clone> Params<T, Ps> for (Context<T, Ps>,) {
    fn get(ctx: &Context<T, Ps>, _ps: Ps) -> Self {
        (ctx.clone(),)
    }
}
impl<T: KAny, Ps: Clone> Params<T, Ps> for (Context<T, Ps>, Ps) {
    fn get(ctx: &Context<T, Ps>, ps: Ps) -> Self {
        (ctx.clone(), ps)
    }
}

impl<T: KAny, Ps: Clone> Params<T, Ps> for (Ps,) {
    fn get(_ctx: &Context<T, Ps>, ps: Ps) -> Self {
        (ps,)
    }
}

pub struct Availed<T: KAny, Param, Func, Ps: Clone>
where
    Param: Params<T, Ps>,
    Func: FnMut<Param, Output = ()>,
{
    function: Func,
    _p: PhantomData<Param>,
    _t: PhantomData<T>,
    _ps: PhantomData<Ps>,
}

impl<Param, Func, T: KAny, Ps: Clone> Availed<T, Param, Func, Ps>
where
    Param: Params<T, Ps>,
    Func: FnMut<Param, Output = ()>,
{
    fn new(func: Func) -> Self {
        Self {
            function: func,
            _p: PhantomData,
            _t: PhantomData,
            _ps: PhantomData,
        }
    }
}

unsafe impl<Param, Func, T: KAny, Ps: Clone> Send for Availed<T, Param, Func, Ps>
where
    Param: Params<T, Ps>,
    Func: FnMut<Param, Output = ()>,
{
}
unsafe impl<Param, Func, T: KAny, Ps: Clone> Sync for Availed<T, Param, Func, Ps>
where
    Param: Params<T, Ps>,
    Func: FnMut<Param, Output = ()>,
{
}

impl<Param, Func, T: KAny, Ps: Clone> Avail<T, Ps> for Availed<T, Param, Func, Ps>
where
    Param: Params<T, Ps>,
    Func: FnMut<Param, Output = ()>,
{
    fn run(&mut self, ctx: &Context<T, Ps>, ps: Ps) {
        self.function.call_mut(Param::get(ctx, ps))
    }
}

impl<Param, Func, T: KAny, Ps: Clone> From<Func> for Availed<T, Param, Func, Ps>
where
    Param: Params<T, Ps>,
    Func: FnMut<Param, Output = ()>,
{
    fn from(value: Func) -> Self {
        Self::new(value)
    }
}

// ===Avails===

pub struct Avails<T: ?Sized, Ps>(pub DashMap<u128, Box<dyn Avail<T, Ps>>>, Mutex<StepRng>);
pub struct AvailHandle<T: KAny, Param: Params<T, Ps>, Func: FnMut<Param, Output = ()>, Ps: Clone>(
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
impl<T: KAny, Param: Params<T, Ps>, Func: FnMut<Param, Output = ()>, Ps: Clone>
    From<AvailHandle<T, Param, Func, Ps>> for u128
{
    fn from(value: AvailHandle<T, Param, Func, Ps>) -> Self {
        value.0
    }
}
impl<Ps> Avails<dyn KAny, Ps> {
    pub unsafe fn upcast<T: KAny + ?Sized>(self) -> Avails<T, Ps> {
        mem::transmute::<Avails<dyn KAny, Ps>, Avails<T, Ps>>(self)
    }
    pub unsafe fn upcast_ref<T: KAny + ?Sized>(&self) -> &Avails<T, Ps> {
        mem::transmute::<&Avails<dyn KAny, Ps>, &Avails<T, Ps>>(self)
    }
}
impl<T: 'static + Send + Sync + KAny + ?Sized, Ps: Send + Sync + Clone> Avails<T, Ps> {
    /// #### 创建一个可用性函数容器
    ///
    /// 其本质是一个`DashMap`对象，用于存储可用性对象。
    pub fn new() -> Self {
        Self(DashMap::new(), StepRng::new(0, 1).into())
    }
}
impl<T: 'static + Send + Sync + KAny, Ps: Clone + 'static> Avails<T, Ps> {
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
