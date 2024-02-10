use std::fmt::Display;
use kokoro::prelude::*;
use kokoro_default_impl::thread::*;

fn main() {
    let ctx = Context::default();
    // Register a subscriber
    ctx.subscribe(sub_print);
    // Create a publisher
    let _ = ctx.spawn(|ctx, s| {
        // s is a signal that is true when the thread should be terminated
        while !s.is() {
            // Publish the event:Print
            ctx.publish(Print(&"Hello World"));
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });
    ctx.run();
}

#[derive(Event)]
// This is a event:Print
struct Print(&'static dyn Display);

// This is a subscriber who subscribes to the event:Print
fn sub_print(print: &Print) {
    println!("{}", print.0);
}