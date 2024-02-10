use std::sync::Arc;

use kokoro::context::{
    scope::{LocalCache, Scope, Triggerable},
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
    fn plugin(&self, plugin: P);
}
impl<T: LocalCache + 'static, P: Plugin + 'static> Pluginable<T, P> for Context<T> {
    fn plugin(&self, plugin: P) {
        let name: &'static str = plugin.name();
        let plugin = Arc::new(plugin);
        let scope = Scope::create(Arc::clone(&plugin), self);
        self.scope().subscopes().insert(
            self.scope().scope_id_gen.lock().next(name),
            Arc::clone(&scope) as Arc<dyn Triggerable + Send + Sync + 'static>,
        );
        plugin.apply(&self.with(scope));
    }
}
