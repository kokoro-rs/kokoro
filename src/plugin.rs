use crate::{any::*, context::*};
use anyhow::{anyhow, Ok, Result};
pub trait Pluggable<Ps>: Send + Sync + Sized {
    fn plug<P: Plugin<Ps>>(&self, p: P) -> Result<ChildHandle<P>>;
    fn unplug<P: Plugin<Ps>>(&self, handle: ChildHandle<P>) -> Result<Context<P, Ps>>;
}
impl<T: IStableAny + 'static + ?Sized, Ps: Send + Sync> Pluggable<Ps> for Context<T, Ps> {
    fn plug<P: Plugin<Ps>>(&self, p: P) -> Result<ChildHandle<P>> {
        let child_handle = self.with(p);
        let ctx = self
            .get_child(&child_handle)
            .ok_or(anyhow!("Can NOT make Child"))?;
        P::load(ctx)?;
        Ok(child_handle)
    }
    fn unplug<P: Plugin<Ps>>(&self, handle: ChildHandle<P>) -> Result<Context<P, Ps>> {
        let child = self
            .children_raw()
            .remove(&handle.id())
            .ok_or(anyhow!("Plugin NOT Exist"))?;
        Ok(unsafe { child.downcast_unchecked(Some(handle.id()), None) })
    }
}
pub trait Plugin<Ps>: Send + Sync + Sized {
    fn load(ctx: Context<Self, Ps>) -> Result<()>;
}

pub mod dynamic {
    use crate::{any::*, context::*};
    use anyhow::Result;
    use std::sync::Arc;

    pub type LoadFn<Ps> = fn(ctx: Context<dyn IStableAny, Ps>) -> Result<()>;
    pub type CreateFn = fn() -> Arc<dyn IStableAny>;
    pub trait DynPluggable {}
}
