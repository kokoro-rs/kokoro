use std::sync::Arc;

use kokoro_neo::context::Context;

fn main() {
    let ctx = Context::new("hello");
    let p1 = ctx.avails.add(hello);
    ctx.avails.add(hello);
    ctx.avails.run_all(&ctx);
    let mut func = ctx.avails.get(&p1).unwrap();
    drop(ctx.avails.remove(p1));
    // BUG! undefined behaviour
    func.run(&ctx);
    let p2 = ctx.avails.add(print);
    ctx.avails.run_all(&ctx);
    let mut func = ctx.avails.get(&p2).unwrap();
    func.run(&ctx);
}
fn hello() {
    println!("hello");
}
fn print(s: Arc<&str>) {
    println!("{}", s);
}
