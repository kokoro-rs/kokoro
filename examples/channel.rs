use std::{sync::Arc, thread};

use kokoro_neo::context::Context;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<String>();
    let ctx: Context<_, String> = Context::new(tx.clone());
    ctx.avails().add(subsctiber);
    thread::spawn(move || {
        for i in 0..10 {
            tx.send(format!("hello {}", i)).unwrap();
        }
        tx.send("done".to_string()).unwrap();
    });
    for ele in rx {
        if ele.eq("done") {
            break;
        }
        ctx(ele);
    }
}
fn subsctiber(s: Arc<String>) {
    println!("{}", *s);
}
