<div align="center" alt="Kokoro">
  <img src="https://github.com/BERADQ/kokoro-rs/assets/78293733/57a6178e-186f-4526-8ff9-52dd88712daa"></img>
  <h1>Kokoro</h1>
  Dynamic publish-subscribe pattern framework.
  
  Support for dynamic plug-ins and AOP
</div>

## Not yet stable, do not use in production !!

## Simple publish/subscribe
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

## Plug-in system with dynamic capabilities
**APP**
```rust
use kokoro::dynamic_plugin::*;
use kokoro::prelude::*;
use std::sync::Arc;
fn main() {
    let ctx = Context::default();
    let lib = Arc::new(unsafe { libloading::Library::new("path to Plugin (Dynamic link library)").unwrap() });
    ctx.plugin_dynamic(lib).unwrap();
    ctx.publish(PhantomEvent);
    ctx.run();
    /* Typically, the output will be :
     *  Hello from plugin plugin-example
    */
}
```
**Plugin (Dynamic link library)**
```rust
use std::sync::Arc;

use kokoro::dynamic_plugin::*;
use kokoro::prelude::*;

#[derive(DynamicPlugin)]
struct MyPlugin;
impl Plugin for MyPlugin {
    fn apply(&self, ctx: &Context<Self>) {
        ctx.subscribe(sub);
    }

    fn name(&self) -> &'static str {
        "plugin-example"
    }
}
impl Default for MyPlugin {
    fn default() -> Self {
        Self
    }
}
fn sub(ctx: &Context<impl Plugin + 'static>) {
    println!("Hello from plugin {}", ctx.name());
}
```
