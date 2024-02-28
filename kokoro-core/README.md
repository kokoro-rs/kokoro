<div align="center" alt="Kokoro">
  <img src="https://github.com/BERADQ/kokoro-rs/assets/78293733/57a6178e-186f-4526-8ff9-52dd88712daa"></img>
  <h1>Kokoro</h1>

  [![docs.rs](https://img.shields.io/docsrs/kokoro)](https://docs.rs/kokoro/latest/kokoro/)
  [![Crates.io Version](https://img.shields.io/crates/v/kokoro)](https://crates.io/crates/kokoro)
  ![Crates.io License](https://img.shields.io/crates/l/kokoro)
  
  Dynamic publish-subscribe pattern framework. 
  
  Support for dynamic plug-ins and AOP
  
  <h2>Not yet stable, do not use in production !!</h2>
</div>

<br/>

**下面的内容有些老旧了，但是 example 都是正确的**

**如果你懂得中文，并且想要了解这个仓库到底是什么东西，请查看以下 Markdown**

[关于 Kokoro-core](https://github.com/kokoro-rs/kokoro/blob/main/%E5%85%B3%E4%BA%8E%20Kokoro-core.md)

## Simple publish/subscribe

```rust
use std::fmt::Display;
use kokoro::prelude::*;
fn main() {
    let ctx = channel_ctx();
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
use kokoro::prelude::*;

fn main() -> Result<()> {
    let ctx = channel_ctx();
    // let dyp = DynamicPlugin::from_path("path to Plugin (Dynamic link library)"); // Also can do it
    // let dyp = DynamicPlugin::try_from(unsafe { libloading::Library::new("path to Plugin (Dynamic link library)") }); // Also can do it
    let dyp = "path to Plugin (Dynamic link library)"; // String or Library or DynamicPlugin
    let config = toml::toml! {
        hello = "I am plugin"
    };
    ctx.plugin_dynamic(dyp, Some(content.into()))?;
    ctx.publish(PhantomEvent);
    ctx.run();
    /* Typically, the output will be :
     *  I am plugin plugin-example
    */
    Ok(())
}
```

**Plugin (Dynamic link library)**
```rust
use kokoro::prelude::*;
use kokoro::dynamic_plugin::toml::Value;
use serde::Deserialize;

#[derive(DynamicPlugin, Deserialize)]
struct MyPlugin {
    hello: String,
}

impl Plugin for MyPlugin {
    type MODE = MPSC;
    const NAME: &'static str = "plugin-example";
    fn apply(ctx: Context<Self, MPSC>) -> Result<()> {
        ctx.subscribe(sub);
        Ok(())
    }
}

impl Create for MyPlugin {
    fn create(config: Option<Value>) -> Result<Self> {
        if let Some(config) = config {
            let config = MyPlugin::deserialize(config)?;
            Ok(config)
        } else {
            Err(anyhow!("Required Config"))
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

## Star History

<a href="https://star-history.com/#kokoro-rs/kokoro&Date">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://api.star-history.com/svg?repos=kokoro-rs/kokoro&type=Date&theme=dark" />
    <source media="(prefers-color-scheme: light)" srcset="https://api.star-history.com/svg?repos=kokoro-rs/kokoro&type=Date" />
    <img alt="Star History Chart" src="https://api.star-history.com/svg?repos=kokoro-rs/kokoro&type=Date" />
  </picture>
</a>

<br/>

## todo list

- [x] `kokoro-default-impl`
  - [x] `kokoro-plugin-impl`
  - [x] `kokoro-thread-impl`
  - [x] `kokoro-service-impl (AOP Support)`
- [x] `kokoro-dynamic-plugin-impl`
- [x] plugin config api
- [ ] `loader` for dynamically and schematically loading plugins.
- [ ] `logger` for uniform output logging of plugins.
- [x] `k-onfig` is used to hint configuration schema.
- [ ] `Satori (EventType only)` for instant messaging or chatbots
