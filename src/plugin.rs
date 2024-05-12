use crate::{any::*, context::*};
use anyhow::{anyhow, Ok, Result};
pub trait Pluggable<Ps: Clone, G: Clone>: Send + Sync + Sized {
    fn plug<P: Plugin<Pars = Ps, Global = G>>(&self, p: P) -> Result<ChildHandle<P>>;
    fn unplug<P: Plugin<Pars = Ps, Global = G>>(
        &self,
        handle: ChildHandle<P>,
    ) -> Result<Context<P, Ps, G>>;
}
impl<T: KAny + 'static + ?Sized, Ps: Send + Sync + Clone, G: Send + Sync + Clone> Pluggable<Ps, G>
    for Context<T, Ps, G>
{
    fn plug<P: Plugin<Pars = Ps, Global = G>>(&self, p: P) -> Result<ChildHandle<P>> {
        let child_handle = self.with(p);
        let ctx = self
            .get_child(&child_handle)
            .ok_or(anyhow!("Can NOT make Child"))?;
        P::load(ctx)?;
        Ok(child_handle)
    }
    fn unplug<P: Plugin<Pars = Ps, Global = G>>(
        &self,
        handle: ChildHandle<P>,
    ) -> Result<Context<P, Ps, G>> {
        let child = self
            .children_raw()
            .remove(&handle.stable_id())
            .ok_or(anyhow!("Plugin NOT Exist"))?;
        Ok(unsafe { child.downcast_unchecked(None, self.global.clone()) })
    }
}
pub trait Plugin: Send + Sync + Sized + 'static {
    type Global: Clone;
    type Pars: Clone;
    fn load(ctx: Context<Self, Self::Pars, Self::Global>) -> Result<()>;
}
pub mod dynamic {
    use crate::{any::*, avail::*, context::*};
    use anyhow::{anyhow, Result};
    pub use libloading::*;
    use std::{
        marker::PhantomData,
        sync::{Arc, OnceLock},
    };
    pub type LoadFn<Ps, G> =
        extern "Rust" fn(ctx: Arc<RawContext<Ps, G>>, self_id: G) -> Result<()>;
    pub type CreateFn = extern "Rust" fn() -> Result<Arc<dyn KAny>>;
    pub type VerifyFn = extern "Rust" fn() -> u64;
    pub struct DyPlugin<Ps, G> {
        pub lib: Library,
        _p: PhantomData<Ps>,
        _g: PhantomData<G>,
    }
    impl<Ps, G> DyPlugin<Ps, G> {
        pub fn from_lib(lib: Library) -> Result<Self> {
            let verify_fn: Symbol<VerifyFn> = unsafe { lib.get(b"__verify__")? };
            let id = verify_fn();
            if Ps::KID | G::KID != id {
                Err(anyhow!("Plugin Misfit"))
            } else {
                Ok(DyPlugin {
                    lib,
                    _p: PhantomData,
                    _g: PhantomData,
                })
            }
        }
    }
    impl<Ps, G> TryFrom<Library> for DyPlugin<Ps, G> {
        type Error = anyhow::Error;
        fn try_from(value: Library) -> Result<Self, Self::Error> {
            Self::from_lib(value)
        }
    }
    pub trait DyPluggable<Ps, G> {
        fn dyn_plug<L: TryInto<DyPlugin<Ps, G>, Error = anyhow::Error>>(
            &self,
            lib: L,
        ) -> Result<()>;
    }
    impl<
            T: KAny + 'static + ?Sized,
            Ps: Send + Sync + Clone + 'static,
            G: Send + Sync + Clone + 'static,
        > DyPluggable<Ps, G> for Context<T, Ps, G>
    {
        fn dyn_plug<L: TryInto<DyPlugin<Ps, G>, Error = anyhow::Error>>(
            &self,
            lib: L,
        ) -> Result<()> {
            let dyn_plugin: DyPlugin<Ps, G> = lib.try_into()?;
            let create_fn: Symbol<CreateFn> = unsafe { dyn_plugin.lib.get(b"__create__")? };
            let load_fn: Symbol<LoadFn<Ps, G>> = unsafe { dyn_plugin.lib.get(b"__load__")? };
            let scope = create_fn()?;
            let raw: Arc<RawContext<Ps, G>> = RawContext {
                scope,
                children: Children::new(),
                parent: Arc::downgrade(self.raw_ref()),
                avails: Avails::new(),
                _effects: Box::new([OnceLock::new()]),
            }
            .into();
            self.children_raw().add(Arc::clone(&raw));
            load_fn(raw.clone(), self.global.clone())?;
            let _ = raw._effects[0].set(Box::new(dyn_plugin));
            Ok(())
        }
    }
    #[macro_export]
    macro_rules! export_plugin {
        ($plugin:ty,$instance:expr) => {
            #[no_mangle]
            extern "Rust" fn __load__(
                ctx: ::std::sync::Arc<
                    $crate::context::RawContext<
                        <$plugin as $crate::plugin::Plugin>::Pars,
                        <$plugin as $crate::plugin::Plugin>::Global,
                    >,
                >,
                global: <$plugin as $crate::plugin::Plugin>::Global,
            ) -> $crate::result::Result<()> {
                let ctx = unsafe {
                    $crate::context::RawContextExt::downcast_unchecked::<$plugin>(ctx, None, global)
                };
                <$plugin as $crate::plugin::Plugin>::load(ctx)?;
                Ok(())
            }
            #[no_mangle]
            extern "Rust" fn __create__(
            ) -> $crate::result::Result<::std::sync::Arc<dyn $crate::any::KAny>> {
                Ok(::std::sync::Arc::new($instance?))
            }
            #[no_mangle]
            extern "Rust" fn __verify__() -> u64 {
                <<$plugin as $crate::plugin::Plugin>::Pars as $crate::any::KID>::KID
                    | <<$plugin as $crate::plugin::Plugin>::Global as $crate::any::KID>::KID
            }
        };
    }
}
