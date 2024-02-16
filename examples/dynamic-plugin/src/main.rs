use kokoro::dynamic_plugin::*;
use kokoro::prelude::*;
use std::sync::Arc;
fn main() {
    let ctx = mpsc_context();
    let lib = Arc::new(unsafe { libloading::Library::new("../../target/release/plugin_example.dll").unwrap() });
    ctx.plugin_dynamic(lib).unwrap();
    ctx.publish(PhantomEvent);
    ctx.run();
}
