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