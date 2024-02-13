use crate::context::{
    scope::{LocalCache, Scope, Triggerable},
    Context,
};
use crate::mpsc;
use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;

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
    #[inline(always)]
    pub fn new<F>(runner: F) -> Self
        where
            F: FnMut(&Context<RootCache>) + 'static,
    {
        Self(Box::new(runner))
    }
}

impl Default for RootCache {
    #[inline(always)]
    fn default() -> Self {
        RootCache {
            runner: Mutex::new(RunnerCache::new(default_runner)),
        }
    }
}

impl Default for Context<RootCache> {
    #[inline(always)]
    fn default() -> Self {
        let scope = Scope::create(Box::new(RootCache::default()));
        Context::new(Arc::new(scope), mpsc())
    }
}

/// That can be run by a runner
pub trait RunnableContext {
    /// Register a runner
    fn runner<F: FnMut(&Self) + 'static>(&self, runner: F);
    /// Utility runner run context
    fn run(&self);
}

impl RunnableContext for Context<RootCache> {
    #[inline(always)]
    fn runner<F: FnMut(&Context<RootCache>) + 'static>(&self, runner: F) {
        self.runner.lock().0 = Box::new(runner);
    }
    #[inline(always)]
    fn run(&self) {
        self.runner.lock().0(self);
    }
}

/// The default runner
pub fn default_runner(ctx: &Context<RootCache>) {
    for e in ctx.receiver() {
        let ctx = ctx.with(ctx.scope());
        thread::Builder::new()
            .name("main loop thread".to_string())
            .spawn(move || {
                ctx.scope()
                    .trigger_recursive(Arc::clone(&e), unsafe {
                        &*(&ctx as *const Context<RootCache> as *const Context<dyn LocalCache>)
                    })
            })
            .expect("main loop thread can not spawn");
    }
}
