use std::thread;

use kokoro_neo::context::Context;
use kokoro_neo::result;

fn main() -> result::Result<()> {
    let (tx, rx) = std::sync::mpsc::channel::<String>();
    let ctx: Context<_, String, ()> = Context::new(tx.clone(), ());
    ctx.avails().add(subsctiber);
    let _handle;
    {
        let tx = tx.clone();
        _handle = thread::spawn(move || {
            for i in 0..10 {
                tx.clone().send(format!("hello {}", i)).unwrap();
            }
        });
    }
    drop(tx);
    for ele in rx {
        ctx(ele);
    }
    Ok(())
}
fn subsctiber(s: String) {
    println!("{}", s);
}
