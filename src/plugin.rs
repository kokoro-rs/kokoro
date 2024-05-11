use crate::{any::*, context::*};
use anyhow::{anyhow, Ok, Result};
pub trait Pluggable<Ps: Clone>: Send + Sync + Sized {
    fn plug<P: Plugin<Ps>>(&self, p: P) -> Result<ChildHandle<P>>;
    fn unplug<P: Plugin<Ps>>(&self, handle: ChildHandle<P>) -> Result<Context<P, Ps>>;
}
impl<T: KAny + 'static + ?Sized, Ps: Send + Sync + Clone> Pluggable<Ps> for Context<T, Ps> {
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
pub trait Plugin<Ps: Clone>: Send + Sync + Sized {
    fn load(ctx: Context<Self, Ps>) -> Result<()>;
}
pub mod dynamic {
    use crate::{any::*, avail::*, context::*};
    use anyhow::{anyhow, Result};
    use libloading::{Library, Symbol};
    use std::{
        marker::PhantomData,
        sync::{Arc, OnceLock},
    };

    pub type LoadFn<Ps> = fn(ctx: Arc<RawContext<Ps>>, self_id: u64) -> Result<()>;
    pub type CreateFn = fn() -> Arc<dyn KAny>;
    pub type VerifyFn = fn() -> u64;
    pub struct DynPlugin<Ps> {
        pub lib: Library,
        _p: PhantomData<Ps>,
    }
    impl<Ps> DynPlugin<Ps> {
        pub fn from_lib(lib: Library) -> Result<Self> {
            let verify_fn: Symbol<VerifyFn> = unsafe { lib.get(b"__verify__")? };
            let id = verify_fn();
            if Ps::KID != id {
                Err(anyhow!("Plugin Misfit"))
            } else {
                Ok(DynPlugin {
                    lib,
                    _p: PhantomData,
                })
            }
        }
    }
    impl<Ps> TryFrom<Library> for DynPlugin<Ps> {
        type Error = anyhow::Error;
        fn try_from(value: Library) -> Result<Self, Self::Error> {
            Self::from_lib(value)
        }
    }
    impl<Ps> TryFrom<&str> for DynPlugin<Ps> {
        type Error = anyhow::Error;
        fn try_from(value: &str) -> Result<Self, Self::Error> {
            let lib = unsafe { libloading::Library::new(value)? };
            Self::from_lib(lib)
        }
    }
    impl<Ps> TryFrom<&String> for DynPlugin<Ps> {
        type Error = anyhow::Error;
        fn try_from(value: &String) -> Result<Self, Self::Error> {
            let lib = unsafe { libloading::Library::new(value)? };
            Self::from_lib(lib)
        }
    }
    pub trait DynPluggable<Ps> {
        fn dyn_plug<L: TryInto<DynPlugin<Ps>, Error = anyhow::Error>>(&self, lib: L) -> Result<()>;
    }
    impl<T: KAny + 'static + ?Sized, Ps: Send + Sync + Clone + 'static> DynPluggable<Ps>
        for Context<T, Ps>
    {
        fn dyn_plug<L: TryInto<DynPlugin<Ps>, Error = anyhow::Error>>(&self, lib: L) -> Result<()> {
            let dyn_plugin: DynPlugin<Ps> = lib.try_into()?;
            let create_fn: Symbol<CreateFn> = unsafe { dyn_plugin.lib.get(b"__create__")? };
            let load_fn: Symbol<LoadFn<Ps>> = unsafe { dyn_plugin.lib.get(b"__load__")? };
            let scope = create_fn();
            let raw: Arc<RawContext<Ps>> = RawContext {
                scope,
                children: Children::new(),
                parent: Arc::downgrade(self.raw_ref()),
                avails: Avails::new(),
                _effects: Box::new([OnceLock::new()]),
            }
            .into();
            let id = self.children_raw().add(Arc::clone(&raw));
            load_fn(raw.clone(), id)?;
            let _ = raw._effects[0].set(Box::new(dyn_plugin));
            Ok(())
        }
    }
    #[macro_export]
    macro_rules! export_plugin {
        ($plugin:ty,$instance:expr,$type:ty) => {
            #[no_mangle]
            extern "Rust" fn __load__(
                ctx: ::std::sync::Arc<$crate::context::RawContext<$type>>,
                self_id: u64,
            ) -> $crate::result::Result<()> {
                let ctx = unsafe {
                    $crate::context::RawContextExt::downcast_unchecked::<$plugin>(
                        ctx,
                        Some(self_id),
                        None,
                    )
                };
                <$plugin as $crate::plugin::Plugin<$type>>::load(ctx)?;
                Ok(())
            }
            #[no_mangle]
            extern "Rust" fn __create__() -> ::std::sync::Arc<dyn $crate::any::KAny> {
                ::std::sync::Arc::new($instance)
            }
            #[no_mangle]
            extern "Rust" fn __verify__() -> u64 {
                <$type as $crate::any::KID>::KID
            }
        };
    }
}
