pub use anyhow::{self, Result};
use kokoro_core::context::scope::Mode;
use kokoro_core::context::{
    scope::{Resource, Scope, ScopeId},
    Context,
};
use parking_lot::Mutex;
use rand::{rngs::mock::StepRng, Rng};
use std::sync::Arc;

/// Plugin needs to impl this trait
pub trait Plugin: Resource {
    /// Mode of the plugin
    type MODE: Mode + 'static;
    /// Name of the plugin
    const NAME: &'static str;
    /// Is executed when the plugin is applied
    fn apply(ctx: Context<Self, Self::MODE>) -> Result<()>;
}

/// Impl this for plug-ins
pub trait Pluginable<M: Mode + 'static, P: Plugin<MODE = M> + 'static> {
    /// Call this for plug-ins
    fn plugin(&self, plugin: P) -> Result<ScopeId>;
    /// Insert a plugin
    fn insert_plugin(&self, plugin: P, id: ScopeId) -> Result<()>;
}

/// Impl this for unplug-ins
pub trait Unpluginable {
    /// Unplugin from the context
    fn unplug(&self, id: ScopeId);
}

impl<T: Resource + 'static, P: Plugin<MODE = M> + 'static, M: Mode + 'static> Pluginable<M, P>
    for Context<T, M>
{
    #[inline(always)]
    fn plugin(&self, plugin: P) -> Result<ScopeId> {
        let scope_id_gen = self
            .scope()
            .cache()
            .default("kokoro-plugin-impl/scope_id_gen", || {
                Arc::new(ScopeIdGen::new(StepRng::new(0, 1)))
            });
        let id = scope_id_gen.next(P::NAME);
        self.insert_plugin(plugin, id.clone())?;
        Ok(id)
    }
    #[inline(always)]
    fn insert_plugin(&self, plugin: P, id: ScopeId) -> Result<()> {
        let plugin = Arc::new(plugin);
        let scope = Arc::new(Scope::create(plugin));
        P::apply(self.with(Arc::clone(&scope)))?;
        self.scope().subscopes().insert(id.clone(), Box::new(scope));
        Ok(())
    }
}

impl<T: Resource + 'static, M: Mode + 'static> Unpluginable for Context<T, M> {
    #[inline(always)]
    fn unplug(&self, id: ScopeId) {
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
