use std::sync::Arc;

use kokoro_neo::context::Context;
fn main() {
    let ctx = Context::new("world");
    ctx.avails().add(hello);
    let child_handle = ctx.with(123);
    let child = ctx.get_child(&child_handle).unwrap();
    child.avails().add(print);

    ctx("hello");
    ctx("bye");
    // output:
    // hello world
    // hello 123
    // bye world
    // bye 123
}
fn hello(ctx: Context<&str, &str>, s: Arc<&str>) {
    println!("{} {}", *s, *ctx);
}
fn print(ctx: Context<i32, &str>, s: Arc<&str>) {
    println!("{} {}", *s, *ctx);
}
