use crate::{any::StableAny, context::Context};
use std::{
    marker::{PhantomData, Tuple},
    sync::Arc,
};
pub trait Avail<T: StableAny>: Send + Sync {
    fn run(&mut self, ctx: &Context<T>);
}

pub trait Params<T: StableAny>: Tuple {
    fn get(ctx: &Context<T>) -> Self;
}

impl<T: StableAny> Params<T> for () {
    fn get(_ctx: &Context<T>) -> Self {
        ()
    }
}

impl<T: StableAny> Params<T> for (Arc<T>,) {
    fn get(ctx: &Context<T>) -> Self {
        (Arc::clone(&ctx.scope),)
    }
}

pub struct Availed<T: StableAny, Param, Func>
where
    Param: Params<T>,
    Func: FnMut<Param, Output = ()>,
{
    function: Func,
    _p: PhantomData<Param>,
    _t: PhantomData<T>,
}

impl<Param, Func, T: StableAny> Availed<T, Param, Func>
where
    Param: Params<T>,
    Func: FnMut<Param, Output = ()>,
{
    fn new(func: Func) -> Self {
        Self {
            function: func,
            _p: PhantomData,
            _t: PhantomData,
        }
    }
}

unsafe impl<Param, Func, T: StableAny> Send for Availed<T, Param, Func>
where
    Param: Params<T>,
    Func: FnMut<Param, Output = ()>,
{
}
unsafe impl<Param, Func, T: StableAny> Sync for Availed<T, Param, Func>
where
    Param: Params<T>,
    Func: FnMut<Param, Output = ()>,
{
}

impl<Param, Func, T: StableAny> Avail<T> for Availed<T, Param, Func>
where
    Param: Params<T>,
    Func: FnMut<Param, Output = ()>,
{
    fn run(&mut self, ctx: &Context<T>) {
        self.function.call_mut(Param::get(ctx))
    }
}

impl<Param, Func, T: StableAny> From<Func> for Availed<T, Param, Func>
where
    Param: Params<T>,
    Func: FnMut<Param, Output = ()>,
{
    fn from(value: Func) -> Self {
        Self::new(value)
    }
}
