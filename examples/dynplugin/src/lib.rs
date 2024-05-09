use std::sync::Arc;

use kokoro_neo::plugin::Plugin;
use kokoro_neo::{export_plugin, result::*};

struct MyPlugin;

impl Plugin<&'static str> for MyPlugin {
    fn load(ctx: kokoro_neo::context::Context<Self, &'static str>) -> Result<()> {
        ctx.avails().add(print);
        ctx("Hello");
        Ok(())
    }
}
fn print(s: Arc<&str>) {
    println!("from plugin {}", s);
}
impl Default for MyPlugin {
    fn default() -> Self {
        MyPlugin
    }
}

export_plugin!(MyPlugin, MyPlugin::default(), &'static str);
