use kokoro::prelude::*;
use kokoro::core::context::scope::Resource;
use std::sync::{Weak, Arc};

#[derive(DynamicPlugin)]
struct MyPlugin {
    hello: &'static str,
}

impl Plugin for MyPlugin {
    type MODE = MPSC;
    const NAME: &'static str = "plugin-example";
    fn apply(ctx: Context<Self, MPSC>) {
        ctx.subscribe(sub);
        ctx.cache().default("service/plugin-example", || {
            let service_cache = Arc::downgrade(&ctx.scope().resource) as Weak<dyn MyService>;
            Arc::new(service_cache)
        });
    }
}

pub trait MyService {
    fn hello(&self);
    fn bye(&self);
}

pub trait SetupMyService {
    fn my_service(&self) -> Option<Arc<dyn MyService>>;
}

impl<R: Resource + 'static> SetupMyService for Context<R, MPSC> {
    fn my_service(&self) -> Option<Arc<dyn MyService>> {
        self.cache().get::<Weak<dyn MyService>>("service/plugin-example")?.upgrade()
    }
}


impl Resource for dyn MyService + Send + Sync {}

impl MyService for MyPlugin {
    fn hello(&self) {
        println!("{}", self.hello);
        println!("!")
    }
    fn bye(&self) {}
}

impl Default for MyPlugin {
    fn default() -> Self {
        Self {
            hello: "Hello form plugin! ",
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
