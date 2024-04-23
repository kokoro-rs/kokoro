use std::{
    sync::{
        mpsc::{channel, Sender},
        Arc,
    },
    thread,
};

use kokoro_neo::context::*;

enum Event {
    Foo,
    Bar,
    EOP,
}
fn main() {
    let (tx, rx) = channel::<Event>();
    let ctx: Context<_, Event> = Context::new(tx);
    let ctx_clone = ctx.clone();
    let handle = thread::spawn(move || {
        for ele in rx {
            if let Event::EOP = ele {
                break;
            }
            ctx_clone(ele);
        }
    });
    ctx.avails().add(subsctiber); // 订阅
    ctx.send(Event::Foo).unwrap(); // 发布
    ctx.send(Event::Bar).unwrap();

    // 结束
    ctx.send(Event::EOP).unwrap();
    handle.join().unwrap();
}

fn subsctiber(ctx: Context<Sender<Event>, Event>, s: Arc<Event>) {
    ctx.send(Event::Foo).unwrap();
    ctx.avails().add(subsctiber);
    if let Event::Foo = *s {
        println!("hello foo");
    }
}
