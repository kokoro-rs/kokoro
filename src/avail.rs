use crate::{any::IStableAny, context::Context};
use std::{
    marker::{PhantomData, Tuple},
    sync::Arc,
};
pub trait Avail<T: IStableAny>: Send + Sync {
    fn run(&mut self, ctx: &Context<T>);
}

pub trait Params<T: IStableAny>: Tuple {
    fn get(ctx: &Context<T>) -> Self;
}

impl<T: IStableAny> Params<T> for () {
    fn get(_ctx: &Context<T>) -> Self {
        ()
    }
}

impl<T: IStableAny> Params<T> for (Arc<T>,) {
    fn get(ctx: &Context<T>) -> Self {
        (Arc::clone(&ctx.scope),)
    }
}

pub struct Availed<T: IStableAny, Param, Func>
where
    Param: Params<T>,
    Func: FnMut<Param, Output = ()>,
{
    function: Func,
    _p: PhantomData<Param>,
    _t: PhantomData<T>,
}

impl<Param, Func, T: IStableAny> Availed<T, Param, Func>
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

unsafe impl<Param, Func, T: IStableAny> Send for Availed<T, Param, Func>
where
    Param: Params<T>,
    Func: FnMut<Param, Output = ()>,
{
}
unsafe impl<Param, Func, T: IStableAny> Sync for Availed<T, Param, Func>
where
    Param: Params<T>,
    Func: FnMut<Param, Output = ()>,
{
}

impl<Param, Func, T: IStableAny> Avail<T> for Availed<T, Param, Func>
where
    Param: Params<T>,
    Func: FnMut<Param, Output = ()>,
{
    fn run(&mut self, ctx: &Context<T>) {
        self.function.call_mut(Param::get(ctx))
    }
}

impl<Param, Func, T: IStableAny> From<Func> for Availed<T, Param, Func>
where
    Param: Params<T>,
    Func: FnMut<Param, Output = ()>,
{
    fn from(value: Func) -> Self {
        Self::new(value)
    }
}
