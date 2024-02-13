use kokoro::dynamic_plugin::*;
use kokoro::prelude::*;

#[derive(DynamicPlugin)]
struct MyPlugin {
    hello: &'static str,
}

impl Plugin for MyPlugin {
    const NAME: &'static str = "plugin-example";
    fn apply(ctx: Context<Self>) {
        ctx.subscribe(sub);
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
        ctx.hello,
        MyPlugin::NAME
    );
}
