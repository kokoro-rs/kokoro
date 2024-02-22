use kokoro::core::context::scope::Mode;
use kokoro::dynamic_plugin::*;
use kokoro::prelude::*;
use plugin_example::SetupMyService;

fn main() {
    let ctx = mpsc_context();
    if let Some(service) = ctx.my_service() {
        service.hello();
    } else {
        println!("no service");
    }
    let config = toml::toml! {
        content = "I am plugin"
    };
    ctx.plugin_dynamic("plugin_dynamic.dll", Some(config.into()))
        .unwrap();
    if let Some(service) = ctx.my_service() {
        service.hello();
    } else {
        println!("no service");
    }
    ctx.publish(PhantomEvent);
    ctx.run();
}
trait ConfigSchema {
    fn config_schema(&self);
}
impl<M: Mode + 'static> ConfigSchema for DynamicPlugin<M> {
    fn config_schema(&self) {
        unsafe {
            self.get::<fn()>(b"__config_schema").unwrap()();
        };
    }
}
