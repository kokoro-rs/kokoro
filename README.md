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
