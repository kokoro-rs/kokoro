#![warn(missing_docs)]
#![doc = "Provide Kokoro with the ability to dynamically load plugins"]
use kokoro_core::context::{
    scope::{LocalCache, Scope, ScopeId, Triggerable},
    Context,
};
pub use libloading;
use libloading::Library;
use std::sync::Arc;
/// Dynamic Plugin needs to impl this trait
pub trait DynamicPlugin: LocalCache {
    /// Is executed when the plugin is applied
    fn dyn_apply(&self, ctx: &Context<dyn LocalCache>);
    /// Name of the plugin
    fn dyn_name(&self) -> &'static str;
}

type CreateFn = fn() -> Arc<dyn DynamicPlugin>;
/// Provide Context with the ability to dynamically load plugins
pub trait DynamicPluginable<T: LocalCache> {
    /// Dynamically loading plugins
    fn plugin_dynamic(&self, lib: Arc<Library>) -> Result<ScopeId, libloading::Error>;
}
impl<T: LocalCache + 'static> DynamicPluginable<T> for Context<T> {
    #[inline(always)]
    fn plugin_dynamic(&self, lib: Arc<Library>) -> Result<ScopeId, libloading::Error> {
        let lib = Arc::new(lib);
        let create_fn = unsafe { lib.get::<CreateFn>(b"_create")? };
        let plugin = create_fn();
        let name: &'static str = plugin.dyn_name();
        let scope = Scope::create(
            Arc::clone(unsafe {
                &*(&plugin as *const Arc<dyn DynamicPlugin> as *const Arc<dyn LocalCache>)
            }),
            self,
        );
        let id = self.scope().scope_id_gen.lock().next(name);
        self.scope().subscopes().insert(
            id.clone(),
            Arc::clone(&scope) as Arc<dyn Triggerable + Send + Sync + 'static>,
        );
        scope
            .dyn_cache()
            .insert("kokoro-dynamic-plugin/lib-cache", lib);
        plugin.dyn_apply(&self.with(scope));
        Ok(id)
    }
}
