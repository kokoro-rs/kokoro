use std::sync::Arc;

use kokoro_core::context::{
    scope::{LocalCache, Scope, ScopeId, Triggerable},
    Context,
};

/// Plugin needs to impl this trait
pub trait Plugin: LocalCache {
    /// Is executed when the plugin is applied
    fn apply(&self, ctx: &Context<Self>);
    /// Name of the plugin
    fn name(&self) -> &'static str;
}
/// Impl this for plug-ins
pub trait Pluginable<T: LocalCache, P: Plugin + 'static> {
    /// Call this for plug-ins
    fn plugin(&self, plugin: P) -> ScopeId;
    /// Unplugin from the context
    fn unplugin(&self, id: &ScopeId);
}
impl<T: LocalCache + 'static, P: Plugin + 'static> Pluginable<T, P> for Context<T> {
    fn plugin(&self, plugin: P) -> ScopeId {
        let name: &'static str = plugin.name();
        let plugin = Arc::new(plugin);
        let scope = Scope::create(Arc::clone(&plugin), self);
        let id = self.scope().scope_id_gen.lock().next(name);
        self.scope().subscopes().insert(
            id.clone(),
            Arc::clone(&scope) as Arc<dyn Triggerable + Send + Sync + 'static>,
        );
        plugin.apply(&self.with(scope));
        id
    }
    fn unplugin(&self, id: &ScopeId) {
        self.scope().subscopes().remove(id);
    }
}
