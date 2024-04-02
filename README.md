<div align="center" alt="Kokoro">
  <a href="https://www.kokoro-rs.dev"><img src="https://github.com/BERADQ/kokoro-rs/assets/78293733/57a6178e-186f-4526-8ff9-52dd88712daa"></img></a>

  [![docs.rs](https://img.shields.io/docsrs/kokoro)](https://docs.rs/kokoro/latest/kokoro/)
  [![Crates.io Version](https://img.shields.io/crates/v/kokoro)](https://crates.io/crates/kokoro)
  ![Crates.io License](https://img.shields.io/crates/l/kokoro)
  
  <p>Kokoro is a Rust-based meta-framework that prioritizes memory safety, performance, and stability to empower the creation of highly decoupled applications.</p>

</div>

<br/>

## Advantages

- **Memory Safety 🦀**: Built with Rust's guarantees of memory safety, Kokoro eliminates the need for manual side-effect management.
- **High Performance ⚡️**: Designed for efficiency, Kokoro ensures rapid response times and exceptional performance.
- **Stable & Reliable 🏗️**: Continuously optimized, Kokoro aims to provide stable support for production environments in the future.
- **Dynamic Plugins 🔌**: Supports dynamic plugins, including WASM and dynamic linking libraries, for functional expansion.
- **Hot Module Replacement 🔄**: Simplifies the hot update process based on dynamic plugins, making it easy to understand and implement.
- **Flexibility 🌟**: Highly decoupled by nature, Kokoro allows for easy extension or modification. Modular, loosely coupled, hot-updatable, and distributed - all possible with Kokoro.

## Getting Started

[官网](https://www.kokoro-rs.dev)
todo

## Demo

```rust
use kokoro::{dynamic_plugin::toml::toml, prelude::*};
use kokoro_plugin_tiny_http_event::{http::Response, *};

fn main() -> Result<()> {
    // Create a context for the channel.
    let ctx = channel_ctx();
    // Initialize a new PluginFinder to search for plugins in the "./plugin" directory.
    let pf = PluginFinder::new("./plugin");
    // Find the "kokoro_plugin_tiny_http" plugin.
    let plugin = pf.find("kokoro_plugin_tiny_http");
    // Define the configuration for the plugin using TOML format.
    let config = toml! {
        host = "0.0.0.0" // The host address where the server will listen.
        port = 1145      // The port number for the server.
    };
    // Load the plugin dynamically with the specified configuration.
    ctx.plugin_dynamic(plugin, Some(config.into()))?;
    // Subscribe to the 'hello' event.
    ctx.subscribe(hello);
    // Run the context synchronously.
    ctx.run_sync();

    // Return Ok if everything executes successfully.
    Ok(())
}

// Define a new Path named 'Hello' targeting the "/hello" endpoint.
path!(Hello, "/hello");
// Define the 'hello' function to handle requests to the 'Hello' path.
fn hello(req: PathQuery<Hello>) {
    // Check if there is a request and take ownership of it.
    if let Some(req) = req.take() {
        // Respond to the request with a "Hello World!" message.
        req.respond(Response::from_string("Hello World!")).unwrap();
    }
}

```

This code sets up a simple HTTP server that responds with “Hello World!” when the “/hello” path is accessed.
It uses the tiny_http plugin for handling HTTP events.

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
  - [x] `kokoro-service-impl`
- [x] `kokoro-dynamic-plugin-impl`
- [x] plugin config api
- [ ] `loader` for dynamically and schematically loading plugins.
- [ ] `logger` for uniform output logging of plugins.
- [x] `k-onfig` is used to hint configuration schema.
- [ ] `Satori (EventType only)` for instant messaging or chatbots
