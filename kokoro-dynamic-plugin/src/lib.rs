#![warn(missing_docs)]
#![doc = "Provide Kokoro with the ability to dynamically load plugins"]

use kokoro_core::context::scope::Mode;
use kokoro_core::context::{
    scope::{Resource, Scope, ScopeId},
    Context,
};
use kokoro_default_impl::plugin::{anyhow::Error, Result, ScopeIdGen};
pub use libloading;
use libloading::{Library, Symbol};
use rand::rngs::mock::StepRng;
use std::ffi::OsStr;
use std::sync::Arc;
pub use toml;
use toml::value::Value;

type CreateFn = fn(config: Option<Value>) -> Result<Arc<dyn Resource>>;
type NameFn = fn() -> &'static str;
type ApplyFn<M> = fn(Context<dyn Resource, M>) -> Result<()>;

/// Dynamic Plugin
pub struct DynamicPlugin<M: Mode + 'static> {
    lib: Library,
    create_fn: CreateFn,
    name: &'static str,
    apply_fn: ApplyFn<M>,
}
impl<M: Mode + 'static> DynamicPlugin<M> {
    /// load plugin from path
    pub fn from_path<P: AsRef<OsStr>>(path: P) -> Result<Self> {
        let lib = unsafe { libloading::Library::new(path)? };
        Self::from_lib(lib)
    }
    /// load plugin from lib
    pub fn from_lib(lib: Library) -> Result<Self> {
        let create_fn = unsafe { lib.get::<CreateFn>(b"__plugin_create")?.into_raw() };
        let name = unsafe { lib.get::<NameFn>(b"__plugin_name")?() };
        let apply_fn = unsafe { lib.get::<ApplyFn<M>>(b"__plugin_apply")?.into_raw() };
        Ok(Self {
            create_fn: *create_fn,
            name,
            apply_fn: *apply_fn,
            lib,
        })
    }
    /// call create function
    pub fn create(&self, config: Option<Value>) -> Result<Arc<dyn Resource>> {
        (self.create_fn)(config)
    }
    /// call apply function
    pub fn apply(&self, ctx: Context<dyn Resource, M>) -> Result<()> {
        (self.apply_fn)(ctx)
    }
    /// get name
    pub fn name(&self) -> &'static str {
        self.name
    }
    /// get from lib
    pub unsafe fn get<'lib, T>(
        &'lib self,
        symbol: &[u8],
    ) -> Result<Symbol<'lib, T>, libloading::Error> {
        unsafe { self.lib.get(symbol) }
    }
}
impl<M: Mode + 'static> TryFrom<String> for DynamicPlugin<M> {
    type Error = Error;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        DynamicPlugin::from_path(value)
    }
}
impl<M: Mode + 'static> TryFrom<&str> for DynamicPlugin<M> {
    type Error = Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        DynamicPlugin::from_path(value)
    }
}
impl<M: Mode + 'static> TryFrom<&OsStr> for DynamicPlugin<M> {
    type Error = Error;

    fn try_from(value: &OsStr) -> Result<Self, Self::Error> {
        DynamicPlugin::from_path(value)
    }
}
impl<M: Mode + 'static> TryFrom<Library> for DynamicPlugin<M> {
    type Error = Error;

    fn try_from(value: Library) -> Result<Self, Self::Error> {
        DynamicPlugin::from_lib(value)
    }
}
impl<M: Mode + 'static> TryFrom<Result<Library, libloading::Error>> for DynamicPlugin<M> {
    type Error = Error;
    fn try_from(value: Result<Library, libloading::Error>) -> Result<Self, Self::Error> {
        match value {
            Ok(v) => DynamicPlugin::from_lib(v),
            Err(e) => Err(e.into()),
        }
    }
}

/// Create a plugin with the given config
pub trait Create: Sized {
    /// Create a plugin with the given config
    fn create(config: Option<Value>) -> Result<Self>;
}

/// Provide Context with the ability to dynamically load plugins
pub trait DynamicPluginable<R: Resource, M: Mode + 'static> {
    /// Dynamically loading plugins
    fn plugin_dynamic<DyP>(&self, dyplugin: DyP, config: Option<Value>) -> Result<ScopeId>
    where
        DyP: Into<IntoDynamicPlugin<M>>;
}
/// into dynamic plugin
pub struct IntoDynamicPlugin<M: Mode + 'static>(pub Result<DynamicPlugin<M>>);
impl<M: Mode + 'static> From<Result<DynamicPlugin<M>>> for IntoDynamicPlugin<M> {
    fn from(value: Result<DynamicPlugin<M>>) -> Self {
        IntoDynamicPlugin(value)
    }
}
impl<M: Mode + 'static> From<DynamicPlugin<M>> for IntoDynamicPlugin<M> {
    fn from(value: DynamicPlugin<M>) -> Self {
        IntoDynamicPlugin(Ok(value))
    }
}
impl<M: Mode + 'static> From<&str> for IntoDynamicPlugin<M> {
    fn from(value: &str) -> Self {
        IntoDynamicPlugin(DynamicPlugin::from_path(value))
    }
}
impl<M: Mode + 'static> From<&OsStr> for IntoDynamicPlugin<M> {
    fn from(value: &OsStr) -> Self {
        IntoDynamicPlugin(DynamicPlugin::from_path(value))
    }
}
impl<M: Mode + 'static> From<String> for IntoDynamicPlugin<M> {
    fn from(value: String) -> Self {
        IntoDynamicPlugin(DynamicPlugin::from_path(value))
    }
}
impl<M: Mode + 'static> From<Library> for IntoDynamicPlugin<M> {
    fn from(value: Library) -> Self {
        IntoDynamicPlugin(DynamicPlugin::from_lib(value))
    }
}
impl<M: Mode + 'static> From<Result<Library, libloading::Error>> for IntoDynamicPlugin<M> {
    fn from(value: Result<Library, libloading::Error>) -> Self {
        match value {
            Ok(v) => IntoDynamicPlugin(DynamicPlugin::from_lib(v)),
            Err(e) => IntoDynamicPlugin(Err(e.into())),
        }
    }
}

impl<R: Resource + 'static, M: Mode + 'static> DynamicPluginable<R, M> for Context<R, M> {
    #[inline(always)]
    fn plugin_dynamic<DyP>(&self, dyplugin: DyP, config: Option<Value>) -> Result<ScopeId>
    where
        DyP: Into<IntoDynamicPlugin<M>>,
    {
        let dyplugin = dyplugin.into().0?;
        let scope_id_gen = self
            .scope()
            .cache()
            .default("kokoro-plugin-impl/scope_id_gen", || {
                Arc::new(ScopeIdGen::new(StepRng::new(0, 1)))
            });
        let plugin = dyplugin.create(config)?;
        let name: &'static str = dyplugin.name();
        let scope = Arc::new(Scope::create(plugin));
        dyplugin.apply(self.with(Arc::clone(&scope)))?;
        let id = scope_id_gen.next(name);
        scope
            .cache()
            .insert("kokoro-dynamic-plugin/lib-cache", Arc::new(dyplugin));
        // plugin.dyn_apply(&self.with(Arc::clone(&scope)));
        self.scope().subscopes().insert(id.clone(), Box::new(scope));
        Ok(id)
    }
}
