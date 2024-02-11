#![feature(test)]
extern crate test;
use std::fmt::Display;
use std::sync::Arc;

use kokoro::core::context::scope::Triggerable;
use kokoro::prelude::*;
use std::thread::Builder;
fn main() {
    let ctx = Context::default();
    ctx.subscribe(sub);
    let builder = Builder::new()
        .stack_size(10 * 1024 * 1024 * 1024)
        .name("Joke".to_string());
    ctx.spawn_with_builder(builder, |ctx, _| {
        for i in 0x4e00..=0x952f {
            ctx.publish(Char(unsafe { char::from_u32_unchecked(i) }));
        }
    })
    .unwrap();
    ctx.run()
}
#[derive(Event)]
struct Stop;
#[derive(Event)]
struct Char(char);
impl Display for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
fn sub(_c: &Char) {}

#[bench]
fn bench(b: &mut test::Bencher) {
    let ctx = Context::default();
    ctx.runner(bench_runner);
    ctx.subscribe(sub);
    b.iter(|| {
        for i in 0x4e00..=0x952f {
            ctx.publish(Char(unsafe { char::from_u32_unchecked(i) }));
        }
        ctx.publish(Stop);
        ctx.run()
    })
}
fn bench_runner(ctx: &Context<RootCache>) {
    for e in ctx.receiver() {
        let ctx = ctx.with(ctx.scope());
        ctx.scope().trigger_recursive(Arc::clone(&e));
        if e.is::<Stop>() {
            break;
        }
    }
}
