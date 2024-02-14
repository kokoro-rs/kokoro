use kokoro::{core::context::scope::Triggerable, prelude::*};
use std::sync::atomic::{AtomicI32, Ordering::Relaxed};
use std::sync::Arc;
use kokoro_core::context::scope::Resource;

fn main() {
    static I: AtomicI32 = AtomicI32::new(0);
    let ctx = Context::default();
    // A closure can capture the environment, which in this case is atom I
    ctx.subscribe(|| {
        I.fetch_add(1, Relaxed);
    });
    ctx.subscribe(|| {
        I.fetch_add(2, Relaxed);
    });
    ctx.subscribe(|_: &Print| {
        println!("{}", I.load(Relaxed));
    });
    ctx.runner(custom_runner);
    ctx.run();
    /* Typically, the output will be :
     *  3
     */
}

#[derive(Event)]
struct Print;

fn custom_runner(ctx: &Context<Root>) {
    let ctx_dyn = unsafe { &*(ctx as *const Context<Root> as *const Context<dyn Resource>) };
    ctx.scope().trigger_recursive(Arc::new(PhantomEvent),ctx_dyn);
    ctx.scope().trigger_recursive(Arc::new(Print),ctx_dyn);
}
