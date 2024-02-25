use kokoro::prelude::*;
use plugin_example::SetupMyService;
fn main() -> Result<()> {
    let ctx = channel_ctx();
    let config = toml::toml! {
        content = "I am plugin"
    };
    let pf = PluginFinder::new("../../target/release/");
    ctx.plugin_dynamic(pf.find("plugin_example"), Some(config.into()))?;
    if let Some(service) = ctx.my_service() {
        service.hello();
    } else {
        println!("no service");
    }
    ctx.publish(PhantomEvent);
    ctx.run();
    Ok(())
}
