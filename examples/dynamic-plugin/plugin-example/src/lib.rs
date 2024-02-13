use kokoro::dynamic_plugin::*;
use kokoro::prelude::*;

#[derive(DynamicPlugin)]
struct MyPlugin {
    hello: &'static str,
}

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
        Self {
            hello: "Hello form plugin",
        }
    }
}
fn sub(ctx: &Context<MyPlugin>) {
    println!(
        "{} {}",
        ctx.cache().unwrap().hello,
        ctx.cache().unwrap().name()
    );
}
