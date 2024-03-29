use kokoro::prelude::*;
use kokoro_core::query::EventQuery;
use std::{fmt::Display, thread, time::Duration};
fn main() {
    let ctx = channel_ctx();
    // Register a subscriber
    ctx.subscribe(sub_print);
    // Create a publisher
    let handle = ctx.spawn(|ctx, s| {
        // when thread should be terminated
        for t in s {
            // Publish the event:Print
            if t {
                println!("Bye")
            }
            ctx.publish(Print(&"Hello World"));
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
    ctx.add_disposable(handle);
    let handle = ctx.run();
    ctx.add_disposable(handle);
    thread::sleep(Duration::from_secs(5));
    dispose(ctx);
    /* Typically, the output will be :
     *  Hello World
     *  ...
     */
}

#[derive(Event)]
// This is a event:Print
struct Print(&'static dyn Display);

// This is a subscriber who subscribes to the event:Print
fn sub_print(print: EventQuery<Print>) {
    println!("{}", print.0);
}
