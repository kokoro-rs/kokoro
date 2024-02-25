use kokoro::prelude::*;
use plugin_example::SetupMyService;
use plugin_example::Say;
fn main() -> Result<()> {
    let ctx = channel_ctx();
    let config = toml::toml! {
        content = "I am plugin"
    };
    let pf = PluginFinder::new("../../target/release/");
    ctx.plugin_dynamic(pf.find("plugin_example"), Some(config.into()))?;
    if let Some(service) = ctx.my_service() {
        service.hello();
        service.bye();
    } else {
        println!("no service");
    }
    ctx.publish(Say::i("!"));
    ctx.run();
    Ok(())
}
