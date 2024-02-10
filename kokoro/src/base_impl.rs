use crate::context::{
    scope::{LocalCache, Scope, Triggerable},
    Context,
};
use crate::mpsc;
use parking_lot::Mutex;
use std::sync::Arc;
impl<T: Send + Sync> LocalCache for T {}
/// The root's cache
pub struct RootCache {
    runner: Mutex<RunnerCache>,
}
/// Runner wrapper
pub struct RunnerCache(Box<dyn FnMut(&Context<RootCache>) + 'static>);
unsafe impl Send for RunnerCache {}
unsafe impl Sync for RunnerCache {}
impl RunnerCache {
    /// Wrap a runner
    pub fn new<F>(runner: F) -> Self
    where
        F: FnMut(&Context<RootCache>) + 'static,
    {
        Self(Box::new(runner))
    }
}
impl Default for RootCache {
    fn default() -> Self {
        RootCache {
            runner: Mutex::new(RunnerCache::new(default_runner)),
        }
    }
}

pub trait MyDefault {
    fn default() -> Self;
}
impl MyDefault for Context<RootCache> {
    fn default() -> Self {
        Scope::build(Arc::new(RootCache::default()), |s| Context::new(s, mpsc())).1
    }
}
pub trait RunnableContext {
    fn runner<F: FnMut(&Self) + 'static>(&self, runner: F);
    fn run(&self);
}
impl RunnableContext for Context<RootCache> {
    fn runner<F: FnMut(&Context<RootCache>) + 'static>(&self, runner: F) {
        self.scope().cache.runner.lock().0 = Box::new(runner);
    }
    fn run(&self) {
        self.scope().cache.runner.lock().0(self);
    }
}
pub fn default_runner(ctx: &Context<RootCache>) {
    for e in ctx.receiver() {
        let ctx = ctx.with(ctx.scope());
        std::thread::spawn(move || ctx.scope().trigger_recursive(Arc::clone(&e)));
    }
}
