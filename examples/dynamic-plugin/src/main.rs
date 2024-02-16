use kokoro::dynamic_plugin::*;
use kokoro::prelude::*;
use std::sync::Arc;
use kokoro::dynamic_plugin::toml::Value;
use plugin_example::SetupMyService;

fn main() {
    let ctx = mpsc_context();
    let lib = Arc::new(unsafe { libloading::Library::new("plugin_example.dll").expect("plugin_example.dll unable to load") });
    if let Some(service) = ctx.my_service() {
        service.hello();
    } else {
        println!("no service");
    }
    ctx.plugin_dynamic(lib, Some(Value::String("I am plugin".to_string()))).unwrap();
    if let Some(service) = ctx.my_service() {
        service.hello();
    } else {
        println!("no service");
    }
    ctx.publish(PhantomEvent);
    ctx.run();
}
