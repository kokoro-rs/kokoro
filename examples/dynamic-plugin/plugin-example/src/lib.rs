use kokoro::core::context::scope::Resource;
use kokoro::default_impl::plugin::anyhow::anyhow;
use kokoro::default_impl::plugin::Result;
use kokoro::dynamic_plugin::toml::Value;
use kokoro::prelude::scope::Mode;
use kokoro::prelude::*;
use serde::Deserialize;
use std::sync::Arc;

#[derive(DynamicPlugin, Deserialize)]
struct MyPlugin {
    content: String,
}

impl Plugin for MyPlugin {
    type MODE = MPSC<u8>;
    const NAME: &'static str = "plugin-example";
    fn apply(ctx: Context<Self, MPSC<u8>>) -> Result<()> {
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
        println!("{}!", self.content);
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
        if let Some(config) = config {
            let config = MyPlugin::deserialize(config)?;
            Ok(config)
        } else {
            Err(anyhow!("需要配置"))
        }
    }
}

fn sub(ctx: &Context<MyPlugin, MPSC<u8>>) {
    println!("{} {}", ctx.content, MyPlugin::NAME);
}
