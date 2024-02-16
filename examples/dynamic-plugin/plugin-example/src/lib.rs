use kokoro::prelude::*;

#[derive(DynamicPlugin)]
struct MyPlugin {
    hello: &'static str,
}

impl Plugin for MyPlugin {
    type MODE = MPSC;
    const NAME: &'static str = "plugin-example";
    fn apply(ctx: Context<Self, MPSC>) {
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

fn sub(ctx: &Context<MyPlugin, MPSC>) {
    println!(
        "{} {}",
        ctx.hello,
        MyPlugin::NAME
    );
}
