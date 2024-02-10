use kokoro::prelude::*;
use std::sync::atomic::{AtomicI32, Ordering::Relaxed};
use std::sync::Arc;

fn main() {
    static I: AtomicI32 = AtomicI32::new(0);
    let mut ctx = Context::default();
    // 闭包可以捕获环境，在这里就是原子 I
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
    ctx.run()
}
#[derive(Event)]
struct Print;

fn custom_runner(ctx: &Context<RootCache>) {
    ctx.scope().trigger_recursive(Arc::new(PhantomEvent), &ctx);
    ctx.scope().trigger_recursive(Arc::new(Print), &ctx);
}
