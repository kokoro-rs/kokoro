use crate::{any::KAny, context::Context};
use std::{
    marker::{PhantomData, Tuple},
    sync::Arc,
};
pub trait Avail<T: KAny + ?Sized, Ps>: Send + Sync {
    fn run(&mut self, ctx: &Context<T, Ps>, ps: Arc<Ps>);
}

pub trait Params<T: KAny, Ps>: Tuple {
    fn get(ctx: &Context<T, Ps>, ps: Arc<Ps>) -> Self;
}

impl<T: KAny, Ps> Params<T, Ps> for () {
    fn get(_ctx: &Context<T, Ps>, _ps: Arc<Ps>) -> Self {
        ()
    }
}

impl<T: KAny, Ps> Params<T, Ps> for (Context<T, Ps>,) {
    fn get(ctx: &Context<T, Ps>, _ps: Arc<Ps>) -> Self {
        (ctx.clone(),)
    }
}
impl<T: KAny, Ps> Params<T, Ps> for (Context<T, Ps>, Arc<Ps>) {
    fn get(ctx: &Context<T, Ps>, ps: Arc<Ps>) -> Self {
        (ctx.clone(), ps)
    }
}

impl<T: KAny, Ps> Params<T, Ps> for (Arc<Ps>,) {
    fn get(_ctx: &Context<T, Ps>, ps: Arc<Ps>) -> Self {
        (ps,)
    }
}

pub struct Availed<T: KAny, Param, Func, Ps>
where
    Param: Params<T, Ps>,
    Func: FnMut<Param, Output = ()>,
{
    function: Func,
    _p: PhantomData<Param>,
    _t: PhantomData<T>,
    _ps: PhantomData<Ps>,
}

impl<Param, Func, T: KAny, Ps> Availed<T, Param, Func, Ps>
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

unsafe impl<Param, Func, T: KAny, Ps> Send for Availed<T, Param, Func, Ps>
where
    Param: Params<T, Ps>,
    Func: FnMut<Param, Output = ()>,
{
}
unsafe impl<Param, Func, T: KAny, Ps> Sync for Availed<T, Param, Func, Ps>
where
    Param: Params<T, Ps>,
    Func: FnMut<Param, Output = ()>,
{
}

impl<Param, Func, T: KAny, Ps> Avail<T, Ps> for Availed<T, Param, Func, Ps>
where
    Param: Params<T, Ps>,
    Func: FnMut<Param, Output = ()>,
{
    fn run(&mut self, ctx: &Context<T, Ps>, ps: Arc<Ps>) {
        self.function.call_mut(Param::get(ctx, ps))
    }
}

impl<Param, Func, T: KAny, Ps> From<Func> for Availed<T, Param, Func, Ps>
where
    Param: Params<T, Ps>,
    Func: FnMut<Param, Output = ()>,
{
    fn from(value: Func) -> Self {
        Self::new(value)
    }
}
