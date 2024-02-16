#![warn(missing_docs)]
#![doc = "Provide Kokoro with the ability to dynamically load plugins"]

pub use toml;
use kokoro_core::context::{
    scope::{Resource, Scope, ScopeId},
    Context,
};
use kokoro_default_impl::plugin::ScopeIdGen;
pub use libloading;
use libloading::Library;
use rand::rngs::mock::StepRng;
use std::sync::Arc;
use kokoro_core::context::scope::Mode;
use toml::value::Value;

type CreateFn = fn(config: Option<Value>) -> Arc<dyn Resource>;
type NameFn = fn() -> &'static str;
type ApplyFn<M> = fn(Context<dyn Resource, M>);

/// Create a plugin with the given config
pub trait Create {
    /// Create a plugin with the given config
    fn create(config: Option<Value>) -> Self;
}

/// Provide Context with the ability to dynamically load plugins
pub trait DynamicPluginable<T: Resource> {
    /// Dynamically loading plugins
    fn plugin_dynamic(&self, lib: Arc<Library>, config: Option<Value>) -> Result<ScopeId, libloading::Error>;
}

impl<R: Resource + 'static, M: Mode + 'static> DynamicPluginable<R> for Context<R, M> {
    #[inline(always)]
    fn plugin_dynamic(&self, lib: Arc<Library>, config: Option<Value>) -> Result<ScopeId, libloading::Error> {
        let scope_id_gen = self
            .scope()
            .cache()
            .default("kokoro-plugin-impl/scope_id_gen", || {
                Arc::new(ScopeIdGen::new(StepRng::new(0, 1)))
            });
        let create_fn = unsafe { lib.get::<CreateFn>(b"__plugin_create")? };
        let name_fn = unsafe { lib.get::<NameFn>(b"__plugin_name")? };
        let apply_fn = unsafe { lib.get::<ApplyFn<M>>(b"__plugin_apply")? };
        let plugin = create_fn(config);
        let name: &'static str = name_fn();
        let scope = Arc::new(Scope::create(plugin));
        apply_fn(self.with(Arc::clone(&scope)));
        let id = scope_id_gen.next(name);
        scope
            .cache()
            .insert("kokoro-dynamic-plugin/lib-cache", lib);
        // plugin.dyn_apply(&self.with(Arc::clone(&scope)));
        self.scope().subscopes().insert(id.clone(), Box::new(scope));
        Ok(id)
    }
}
