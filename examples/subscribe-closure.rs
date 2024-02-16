use kokoro::{core::context::scope::Triggerable, prelude::*};
use kokoro_core::context::scope::Scope;
use std::sync::atomic::{AtomicI32, Ordering::Relaxed};
use std::sync::Arc;

fn main() {
    static I: AtomicI32 = AtomicI32::new(0);
    let scope = Scope::create(Arc::new(Root::default()));
    let ctx = Context::create(Arc::new(scope), Arc::new(()));
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
    runner(&ctx);
    /* Typically, the output will be :
     *  3
     */
}

#[derive(Event)]
struct Print;

fn runner(ctx: &Context<Root, ()>) {
    let ctx_dyn = ctx.dynref();
    ctx.scope()
        .trigger_recursive(Arc::new(PhantomEvent), ctx_dyn);
    ctx.scope().trigger_recursive(Arc::new(Print), ctx_dyn);
}
