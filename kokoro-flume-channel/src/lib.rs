use flume::{unbounded, Receiver, Sender};
use kokoro_core::base_impl::{Root, SSE};
use kokoro_core::context::scope::{Resource, Scope, Triggerable};
use kokoro_core::context::Context;
use kokoro_core::event::Event;
use parking_lot::Mutex;
use std::sync::Arc;
use std::thread;

pub struct MPSC {
    sender: Sender<Arc<SSE>>,
    receiver: Receiver<Arc<SSE>>,
    runner: Mutex<RunnerCache>,
}

impl MPSC {}

impl Default for MPSC {
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
pub struct RunnerCache(Box<dyn FnMut(&Context<Root, MPSC>) + 'static>);

unsafe impl Send for RunnerCache {}

unsafe impl Sync for RunnerCache {}

impl RunnerCache {
    /// Wrap a runner
    #[inline(always)]
    pub fn new<F>(runner: F) -> Self
    where
        F: FnMut(&Context<Root, MPSC>) + 'static,
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

impl Runnable for Context<Root, MPSC> {
    #[inline(always)]
    fn runner<F: FnMut(&Context<Root, MPSC>) + 'static>(&self, runner: F) {
        self.global().runner.lock().0 = Box::new(runner);
    }
    #[inline(always)]
    fn run(&self) {
        self.global().runner.lock().0(self);
    }
}

/// The default runner
pub fn default_runner(ctx: &Context<Root, MPSC>) {
    for e in &ctx.global().receiver {
        let ctx = ctx.with(ctx.scope());
        thread::Builder::new()
            .name("main loop thread".to_string())
            .spawn(move || {
                ctx.scope().trigger_recursive(Arc::clone(&e), unsafe {
                    &*(&ctx as *const Context<Root, MPSC> as *const Context<dyn Resource, MPSC>)
                })
            })
            .expect("main loop thread can not spawn");
    }
}

pub trait Publishable<E> {
    fn publish(&self, e: E);
}

impl<T: Resource + ?Sized + 'static, E: Event + Send + Sync + 'static> Publishable<E>
    for Context<T, MPSC>
{
    fn publish(&self, e: E) {
        self.global()
            .sender
            .send(Arc::new(e))
            .expect("can not publish");
    }
}
pub fn mpsc_context() -> Context<Root, MPSC> {
    let scope = Scope::create(Box::new(Root::default()));
    let mode = MPSC::default();
    let ctx = Context::create(Arc::new(scope), Arc::new(mode));
    ctx
}
