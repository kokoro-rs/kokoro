use kokoro::core::context::scope::Resource;
use kokoro::default_impl::plugin::anyhow::anyhow;
use kokoro::dynamic_plugin::toml::Value;
use kokoro::prelude::scope::Mode;
use kokoro::prelude::*;
use serde::Deserialize;
use std::sync::Arc;

#[derive(DynamicPlugin, Deserialize)]
struct MyPlugin {
    content: String,
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
impl Plugin for MyPlugin {
    type MODE = MPSC;
    const NAME: &'static str = "plugin-example";
    fn apply(ctx: Context<Self, MPSC>) -> Result<()> {
        kokoro::default_impl::init_service!(ctx, "plugin-example", MyService);
        ctx.subscribe(|ctx: &Context<MyPlugin, _>, s: &Say| println!("{} {}", s.0, ctx.content));
        Ok(())
    }
}

#[derive(Event)]
pub struct Say(String);
impl Say {
    pub fn i<S: Into<String>>(s: S) -> Self {
        Say(s.into())
    }
}

pub trait MyService {
    fn hello(&self);
    fn bye(&self);
}
impl MyService for MyPlugin {
    fn hello(&self) {
        println!("hello {}", self.content);
    }
    fn bye(&self) {
        println!("bye {}", self.content)
    }
}
pub trait SetupMyService {
    fn my_service(&self) -> Option<Arc<dyn MyService>>;
}
impl<R: Resource + 'static, M: Mode> SetupMyService for Context<R, M> {
    fn my_service(&self) -> Option<Arc<dyn MyService>> {
        kokoro::default_impl::get_service!(self, "plugin-example", MyService)
    }
}
