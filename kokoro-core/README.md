<div align="center">
  <img src="https://github.com/BERADQ/kokoro-rs/assets/78293733/57a6178e-186f-4526-8ff9-52dd88712daa"></img>
  <h1>Kokoro</h1>
</div>

## Simple publish/subscribe
### Demo
```rust
use std::fmt::Display;
use kokoro::prelude::*;

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
    /* Typically, the output will be :
     *  Hello World
     *  ...
    */
}

// This is a event:Print
#[derive(Event)]
struct Print(&'static dyn Display);

// This is a subscriber who subscribes to the event:Print
fn sub_print(print: &Print) {
    println!("{}", print.0);
}
```