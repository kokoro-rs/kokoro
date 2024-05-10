use std::sync::mpsc::{channel, Sender};

use kokoro_neo::context::*;
fn main() {
    let (tx, rx) = channel::<String>(); // 这是传送带
    let ctx: Context<_, &str> = Context::new(tx); // 这是个工厂
    ctx.avails().add(worker1); // 动态组合工厂
    ctx.avails().add(worker2);
    ctx.avails().add(worker3);
    ctx("汽车"); // 派发任务，worker由线程池并发执行。全部结束前会阻塞线程。
    let mut builder = String::new();
    while let Ok(s) = rx.try_recv() {
        builder = format!("{}\n{}", builder, s); // 组装工件
    }
    println!("{}", builder);
}

// 负责生产工件的工人
fn worker1(ctx: Context<Sender<String>, &str>, s: &str) {
    ctx.send(format!("{} 的零件1", s)).unwrap();
}
fn worker2(ctx: Context<Sender<String>, &str>, s: &str) {
    ctx.send(format!("{} 的零件2", s)).unwrap();
}
fn worker3(ctx: Context<Sender<String>, &str>, s: &str) {
    ctx.send(format!("{} 的零件3", s)).unwrap();
}
