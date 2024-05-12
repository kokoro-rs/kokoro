use kokoro_neo::context::Context;
use kokoro_neo::plugin::Plugin;
use kokoro_neo::{export_plugin, result::*};

struct MyPlugin;

impl Plugin for MyPlugin {
    type Pars = &'static str;
    type Global = ();
    fn load(ctx: Context<Self, Self::Pars, Self::Global>) -> Result<()> {
        ctx.avails().add(print);
        ctx("Hello");
        Ok(())
    }
}
fn print(s: &str) {
    println!("from plugin {}", s);
}
impl Default for MyPlugin {
    fn default() -> Self {
        MyPlugin
    }
}

export_plugin!(MyPlugin, Ok(MyPlugin::default()));
