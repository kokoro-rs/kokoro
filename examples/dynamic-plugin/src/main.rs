use kokoro::prelude::*;
use plugin_example::SetupMyService;
fn main() {
    let ctx = mpsc_context(0u8);
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
