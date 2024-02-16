<div align="center" alt="Kokoro">
  <img src="https://github.com/BERADQ/kokoro-rs/assets/78293733/57a6178e-186f-4526-8ff9-52dd88712daa"></img>
  <h1>Kokoro</h1>

  [![docs.rs](https://img.shields.io/docsrs/kokoro)](https://docs.rs/kokoro/latest/kokoro/)
  [![Crates.io Version](https://img.shields.io/crates/v/kokoro)](https://crates.io/crates/kokoro)
  ![Crates.io License](https://img.shields.io/crates/l/kokoro)
  
  Dynamic publish-subscribe pattern framework.
  
  Support for dynamic plug-ins and AOP

  We hope to use Rust to build highly decoupled applications or servers that can be hot-reloaded.

  <h2>Not yet stable, do not use in production !!</h2>
</div>

<br/>

## Simple publish/subscribe

```rust
use std::fmt::Display;
use kokoro::prelude::*;

fn main() {
    let ctx = mpsc_context();
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

#[derive(Event)]
// This is a event:Print
struct Print(&'static dyn Display);

// This is a subscriber who subscribes to the event:Print
fn sub_print(print: &Print) {
    println!("{}", print.0);
}
```

<br/>

## Plug-in system with dynamic capabilities

**APP**
```rust
use kokoro::dynamic_plugin::*;
use kokoro::prelude::*;
use std::sync::Arc;
fn main() {
    let ctx = mpsc_context();
    let lib = Arc::new(unsafe { libloading::Library::new("path to Plugin (Dynamic link library)").unwrap() });
    ctx.plugin_dynamic(lib, Some(Value::String("Hello from plugin".to_string()))).unwrap();
    ctx.publish(PhantomEvent);
    ctx.run();
    /* Typically, the output will be :
     *  Hello from plugin plugin-example
    */
}
```

**Plugin (Dynamic link library)**
```rust
use kokoro::prelude::*;
use kokoro::dynamic_plugin::toml::Value;


#[derive(DynamicPlugin)]
struct MyPlugin {
    hello: String,
}

impl Plugin for MyPlugin {
    type MODE = MPSC;
    const NAME: &'static str = "plugin-example";
    fn apply(ctx: Context<Self, MPSC>) {
        ctx.subscribe(sub);
    }
}

impl Create for MyPlugin {
    fn create(config: Option<Value>) -> Self {
        Self {
            hello: if let Some(Value::String(s)) = config {
                s
            } else {
                "hello".to_string()
            }
        }
    }
}

fn sub(ctx: &Context<MyPlugin, MPSC>) {
    println!(
        "{} {}",
        ctx.hello,
        MyPlugin::NAME
    );
}
```

<br/>

## todo list
- [x] kokoro-default-impl
  - [x] kokoro-plugin-impl
  - [x] kokoro-thread-impl
  - [x] kokoro-service-impl (AOP Support)
- [x] kokoro-dynamic-plugin-impl
- [x] plugin config
- [ ] kokoro-plugin-loader (Hot-Reload Support)
- [ ] kokoro-plugin-logger (Generic logging interface.)
- [ ] kokoro-service-http (Http Service)
