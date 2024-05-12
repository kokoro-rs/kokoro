use anyhow::anyhow;
use kokoro_neo::prelude::*;
use std::{
    sync::{mpsc::channel, Arc},
    thread,
};

struct EOP;

fn main() -> Result<()> {
    let (tx, rx) = channel::<Arc<dyn KAny>>();
    let ctx: Context<_, Arc<dyn KAny>, ()> = Context::new(tx, ());
    let ctx_ = ctx.clone();
    let handle = thread::spawn(move || {
        for event in rx {
            if event.is::<EOP>() {
                break;
            }
            ctx_(event)
        }
    });
    ctx.avails().add(obs);
    ctx.send(Arc::new("Hello kokoro".to_string()))?;

    // stop
    ctx.send(Arc::new(EOP))?;
    handle.join().map_err(|_| anyhow!("Join Err"))?;
    Ok(())
}

fn obs(val: Arc<dyn KAny>) {
    if let Some(s) = val.downcast_ref::<String>() {
        println!("{s}");
    }
}
