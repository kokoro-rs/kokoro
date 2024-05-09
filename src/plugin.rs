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
            .remove(&handle.stable_id())
            .ok_or(anyhow!("Plugin NOT Exist"))?;
        Ok(unsafe { child.downcast_unchecked(Some(handle.stable_id()), None) })
    }
}
pub trait Plugin<Ps>: Send + Sync + Sized {
    fn load(ctx: Context<Self, Ps>) -> Result<()>;
}
pub mod dynamic {
    use crate::{any::*, context::*};
    use anyhow::{anyhow, Result};
    use libloading::{Library, Symbol};
    use std::{
        marker::PhantomData,
        sync::{Arc, Weak},
    };

    pub type LoadFn<Ps> = fn(ctx: Context<dyn IStableAny, Ps>) -> Result<()>;
    pub type CreateFn = fn() -> Arc<dyn IStableAny>;
    pub type VerifyFn = fn() -> u64;
    pub struct DynPlugin<Ps> {
        pub lib: Library,
        _p: PhantomData<Ps>,
    }
    impl<Ps> TryFrom<Library> for DynPlugin<Ps> {
        type Error = anyhow::Error;

        fn try_from(value: Library) -> Result<Self, Self::Error> {
            let verify_fn: Symbol<VerifyFn> = unsafe { value.get(b"__verify__")? };
            let id = verify_fn();
            if Ps::TYPE_ID != id {
                Err(anyhow!("Plugin Misfit"))
            } else {
                Ok(DynPlugin {
                    lib: value,
                    _p: PhantomData,
                })
            }
        }
    }
    pub trait DynPluggable<Ps> {
        fn dyn_plug<L: Into<DynPlugin<Ps>>>(&self, lib: L) -> Result<()>;
    }
    impl<T: IStableAny + 'static + ?Sized, Ps: Send + Sync> DynPluggable<Ps> for Context<T, Ps> {
        fn dyn_plug<L: Into<DynPlugin<Ps>>>(&self, lib: L) -> Result<()> {
            let dyn_plugin: DynPlugin<Ps> = lib.into();
            let create_fn: Symbol<CreateFn> = unsafe { dyn_plugin.lib.get(b"__create__")? };
            let load_fn: Symbol<LoadFn<Ps>> = unsafe { dyn_plugin.lib.get(b"__load__")? };
            let scope = create_fn();
            let raw: Arc<RawContext<Ps>> = RawContext {
                scope,
                children: Children::new(),
                parent: Weak::new(),
                avails: Avails::new(),
            }
            .into();
            let id = self.children_raw().add(Arc::clone(&raw));
            let ctx = unsafe { raw.downcast_unchecked::<dyn IStableAny>(Some(id), None) };
            load_fn(ctx)?;
            Ok(())
        }
    }
}
