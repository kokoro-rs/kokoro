use kokoro::core::context::scope::Resource;
use kokoro::default_impl::plugin::Result;
use kokoro::dynamic_plugin::toml::Value;
use kokoro::prelude::scope::Mode;
use kokoro::prelude::*;
use std::sync::Arc;

#[derive(DynamicPlugin)]
struct MyPlugin {
    hello: String,
}

impl Plugin for MyPlugin {
    type MODE = MPSC;
    const NAME: &'static str = "plugin-example";
    fn apply(ctx: Context<Self, MPSC>) -> Result<()> {
        ctx.subscribe(sub);
        kokoro::default_impl::init_service!(ctx, "plugin-example", MyService);
        Ok(())
    }
}

pub trait MyService {
    fn hello(&self);
    fn bye(&self);
}

impl MyService for MyPlugin {
    fn hello(&self) {
        println!("{}", self.hello);
        println!("!")
    }
    fn bye(&self) {}
}

pub trait SetupMyService {
    fn my_service(&self) -> Option<Arc<dyn MyService>>;
}

impl<R: Resource + 'static, M: Mode> SetupMyService for Context<R, M> {
    fn my_service(&self) -> Option<Arc<dyn MyService>> {
        kokoro::default_impl::get_service!(self, "plugin-example", MyService)
    }
}

impl Create for MyPlugin {
    fn create(config: Option<Value>) -> Result<Self> {
        Ok(Self {
            hello: if let Some(Value::String(s)) = config {
                s
            } else {
                "hello".to_string()
            },
        })
    }
}

fn sub(ctx: &Context<MyPlugin, MPSC>) {
    println!("{} {}", ctx.hello, MyPlugin::NAME);
}
