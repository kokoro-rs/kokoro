use kokoro_neo::{
    any::{IStableAny, StableAny},
    context::Context,
    downcast_ref,
};
use std::sync::Arc;
fn main() {
    let ctx = Context::new("bye");
    ctx.avails.add(hello);
    ctx.avails.add(print);
    ctx.avails.run_all(&ctx);

    let test_value: Box<dyn IStableAny> = Box::new("hello world".to_string());
    let value: Option<&String> = downcast_ref!(*test_value, String);
    let test_value2: Box<dyn IStableAny> = Box::new(123);
    let value2: Option<&String> = downcast_ref!(*test_value2, String);
    dbg!(value, value2);
}
fn hello() {
    println!("hello");
}
fn print(s: Arc<&str>) {
    println!("{}", s);
}
