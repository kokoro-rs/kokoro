#![warn(missing_docs)]
#![doc = "Provide Kokoro with the ability to dynamically load plugins"]

use kokoro_core::context::{
    scope::{LocalCache, Scope, ScopeId},
    Context,
};
use kokoro_default_impl::plugin::ScopeIdGen;
pub use libloading;
use libloading::Library;
use rand::rngs::mock::StepRng;
use std::sync::Arc;

type CreateFn = fn() -> Box<dyn LocalCache>;
type NameFn = fn() -> &'static str;
type ApplyFn = fn(Context<dyn LocalCache>);

/// Provide Context with the ability to dynamically load plugins
pub trait DynamicPluginable<T: LocalCache> {
    /// Dynamically loading plugins
    fn plugin_dynamic(&self, lib: Arc<Library>) -> Result<ScopeId, libloading::Error>;
}

impl<T: LocalCache + 'static> DynamicPluginable<T> for Context<T> {
    #[inline(always)]
    fn plugin_dynamic(&self, lib: Arc<Library>) -> Result<ScopeId, libloading::Error> {
        let scope_id_gen = self
            .scope()
            .dyn_cache()
            .default("kokoro-plugin-impl/scope_id_gen", || {
                Arc::new(ScopeIdGen::new(StepRng::new(0, 1)))
            });
        let create_fn = unsafe { lib.get::<CreateFn>(b"__plugin_create")? };
        let name_fn = unsafe { lib.get::<NameFn>(b"__plugin_name")? };
        let apply_fn = unsafe { lib.get::<ApplyFn>(b"__plugin_apply")? };
        let plugin = create_fn();
        let name: &'static str = name_fn();
        let scope = Arc::new(Scope::create(plugin));
        apply_fn(self.with(Arc::clone(&scope)));
        let id = scope_id_gen.next(name);
        scope
            .dyn_cache()
            .insert("kokoro-dynamic-plugin/lib-cache", lib);
        // plugin.dyn_apply(&self.with(Arc::clone(&scope)));
        self.scope().subscopes().insert(id.clone(), Box::new(scope));
        Ok(id)
    }
}
