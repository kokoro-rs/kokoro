use flume::{unbounded, Receiver, Sender};
use kokoro_core::context::scope::{Resource, Scope, Triggerable};
use kokoro_core::context::Context;
use kokoro_core::event::Event;
use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;

pub struct MPSC<R: Resource + 'static = ()> {
    sender: Sender<Arc<dyn Event + Send + Sync>>,
    receiver: Receiver<Arc<dyn Event + Send + Sync>>,
    runner: Mutex<RunnerCache<R>>,
}

impl<R: Resource> MPSC<R> {}

impl<R: Resource> Default for MPSC<R> {
    fn default() -> Self {
        let (s, r) = unbounded();
        Self {
            sender: s,
            receiver: r,
            runner: Mutex::new(RunnerCache::new(default_runner)),
        }
    }
}

/// Runner wrapper
pub struct RunnerCache<R: Resource + 'static>(Box<dyn FnMut(&Context<R, MPSC<R>>) + 'static>);

unsafe impl<R: Resource> Send for RunnerCache<R> {}

unsafe impl<R: Resource> Sync for RunnerCache<R> {}

impl<R: Resource> RunnerCache<R> {
    /// Wrap a runner
    #[inline(always)]
    pub fn new<F>(runner: F) -> Self
    where
        F: FnMut(&Context<R, MPSC<R>>) + 'static,
    {
        Self(Box::new(runner))
    }
}

/// That can be run by a runner
pub trait Runnable {
    /// Register a runner
    fn runner<F: FnMut(&Self) + 'static>(&self, runner: F);
    /// Utility runner run context
    fn run(&self);
}

impl<R: Resource> Runnable for Context<R, MPSC<R>> {
    #[inline(always)]
    fn runner<F: FnMut(&Context<R, MPSC<R>>) + 'static>(&self, runner: F) {
        self.global().runner.lock().0 = Box::new(runner);
    }
    #[inline(always)]
    fn run(&self) {
        self.global().runner.lock().0(self);
    }
}

/// The default runner
pub fn default_runner<R: Resource>(ctx: &Context<R, MPSC<R>>) {
    for e in &ctx.global().receiver {
        let ctx = ctx.with(ctx.scope());
        thread::Builder::new()
            .name("main loop thread".to_string())
            .spawn(move || {
                ctx.scope().trigger_recursive(Arc::clone(&e), unsafe {
                    &*(&ctx as *const Context<R, MPSC<R>> as *const Context<dyn Resource, MPSC<R>>)
                })
            })
            .expect("main loop thread can not spawn");
    }
}

pub trait Publishable<E> {
    fn publish(&self, e: E);
}

impl<R: Resource + 'static, RR: Resource, E: Event + Send + Sync + 'static> Publishable<E>
    for Context<R, MPSC<RR>>
{
    fn publish(&self, e: E) {
        self.global()
            .sender
            .send(Arc::new(e))
            .expect("can not publish");
    }
}
pub fn channel_ctx() -> Context<(), MPSC> {
    let scope = Scope::create(Arc::new(()));
    let mode = MPSC::<()>::default();
    let ctx = Context::create(Arc::new(scope), Arc::new(mode));
    ctx
}
pub fn channel_ctx_with<R: Resource>(resource: R) -> Context<R, MPSC<R>> {
    let scope = Scope::create(Arc::new(resource));
    let mode = MPSC::<R>::default();
    let ctx = Context::create(Arc::new(scope), Arc::new(mode));
    ctx
}
