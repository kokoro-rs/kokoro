use crate::{any::StableAny, context::Context};
use std::{
    marker::{PhantomData, Tuple},
    sync::Arc,
};
pub trait Avail<T: StableAny + ?Sized, Ps>: Send + Sync {
    fn run(&mut self, ctx: &Context<T, Ps>, ps: Arc<Ps>);
}

pub trait Params<T: StableAny, Ps>: Tuple {
    fn get(ctx: &Context<T, Ps>, ps: Arc<Ps>) -> Self;
}

impl<T: StableAny, Ps> Params<T, Ps> for () {
    fn get(_ctx: &Context<T, Ps>, _ps: Arc<Ps>) -> Self {
        ()
    }
}

impl<T: StableAny, Ps> Params<T, Ps> for (Context<T, Ps>,) {
    fn get(ctx: &Context<T, Ps>, _ps: Arc<Ps>) -> Self {
        (ctx.clone(),)
    }
}
impl<T: StableAny, Ps> Params<T, Ps> for (Context<T, Ps>, Arc<Ps>) {
    fn get(ctx: &Context<T, Ps>, ps: Arc<Ps>) -> Self {
        (ctx.clone(), ps)
    }
}

impl<T: StableAny, Ps> Params<T, Ps> for (Arc<Ps>,) {
    fn get(_ctx: &Context<T, Ps>, ps: Arc<Ps>) -> Self {
        (ps,)
    }
}

pub struct Availed<T: StableAny, Param, Func, Ps>
where
    Param: Params<T, Ps>,
    Func: FnMut<Param, Output = ()>,
{
    function: Func,
    _p: PhantomData<Param>,
    _t: PhantomData<T>,
    _ps: PhantomData<Ps>,
}

impl<Param, Func, T: StableAny, Ps> Availed<T, Param, Func, Ps>
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

unsafe impl<Param, Func, T: StableAny, Ps> Send for Availed<T, Param, Func, Ps>
where
    Param: Params<T, Ps>,
    Func: FnMut<Param, Output = ()>,
{
}
unsafe impl<Param, Func, T: StableAny, Ps> Sync for Availed<T, Param, Func, Ps>
where
    Param: Params<T, Ps>,
    Func: FnMut<Param, Output = ()>,
{
}

impl<Param, Func, T: StableAny, Ps> Avail<T, Ps> for Availed<T, Param, Func, Ps>
where
    Param: Params<T, Ps>,
    Func: FnMut<Param, Output = ()>,
{
    fn run(&mut self, ctx: &Context<T, Ps>, ps: Arc<Ps>) {
        self.function.call_mut(Param::get(ctx, ps))
    }
}

impl<Param, Func, T: StableAny, Ps> From<Func> for Availed<T, Param, Func, Ps>
where
    Param: Params<T, Ps>,
    Func: FnMut<Param, Output = ()>,
{
    fn from(value: Func) -> Self {
        Self::new(value)
    }
}
