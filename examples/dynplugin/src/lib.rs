use std::sync::Arc;

use kokoro_neo::plugin::Plugin;
use kokoro_neo::result::*;

struct MyPlugin;

impl Plugin<&'static str> for MyPlugin {
    fn load(ctx: kokoro_neo::context::Context<Self, &'static str>) -> Result<()> {
        ctx.avails().add(|s: Arc<&str>| {
            println!("{}", s);
        });
        Ok(())
    }
}

extern "Rust" fn __load__() -> Result<()> {}
