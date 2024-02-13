use kokoro_core::context::{
    scope::{LocalCache, Scope, ScopeId},
    Context,
};
use parking_lot::Mutex;
use rand::{rngs::mock::StepRng, Rng};
use std::sync::Arc;

/// Plugin needs to impl this trait
pub trait Plugin: LocalCache {
    /// Name of the plugin
    const NAME: &'static str;
    /// Is executed when the plugin is applied
    fn apply(ctx: Context<Self>);
}
/// Impl this for plug-ins
pub trait Pluginable<P: Plugin + 'static> {
    /// Call this for plug-ins
    fn plugin(&self, plugin: P) -> ScopeId;
}
/// Impl this for unplug-ins
pub trait Unpluginable {
    /// Unplugin from the context
    fn unplugin(&self, id: ScopeId);
}
impl<T: LocalCache + 'static, P: Plugin + 'static> Pluginable<P> for Context<T> {
    #[inline(always)]
    fn plugin(&self, plugin: P) -> ScopeId {
        let scope_id_gen = self
            .scope()
            .dyn_cache()
            .default("kokoro-plugin-impl/scope_id_gen", || {
                Arc::new(ScopeIdGen::new(StepRng::new(0, 1)))
            });
        let id = scope_id_gen.next(P::NAME);
        let plugin = Box::new(plugin);
        let scope = Arc::new(Scope::create(plugin));
        P::apply(self.with(Arc::clone(&scope)));
        self.scope().subscopes().insert(id.clone(), Box::new(scope));
        id
    }
}
impl<T: LocalCache + 'static> Unpluginable for Context<T> {
    #[inline(always)]
    fn unplugin(&self, id: ScopeId) {
        self.scope().subscopes().remove(&id);
    }
}
/// Used to generate consecutive Scopeids that do not repeat
pub struct ScopeIdGen<R: Rng> {
    rand: Mutex<R>,
}
impl<R: Rng> ScopeIdGen<R> {
    #[inline(always)]
    /// Iterate to get a new identifier
    pub fn next(&self, name: &'static str) -> ScopeId {
        let num = self.rand.lock().next_u64();
        ScopeId::new(name, num)
    }
}
impl<R: Rng> ScopeIdGen<R> {
    #[inline(always)]
    /// Create a ScopeIdGen
    pub fn new(rand: R) -> Self {
        Self {
            rand: Mutex::new(rand),
        }
    }
}
