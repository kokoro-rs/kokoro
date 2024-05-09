use anyhow::Result;
use kokoro_neo::{
    context::Context,
    plugin::{Pluggable, Plugin},
};
use std::sync::Arc;

fn main() -> Result<()> {
    let ctx: Context<_, &'static str> = Context::new(());
    ctx.plug(MyPlugin)?;
    ctx("Hello Plugin");
    Ok(())
}

struct MyPlugin;
impl Plugin<&'static str> for MyPlugin {
    fn load(ctx: kokoro_neo::context::Context<Self, &'static str>) -> anyhow::Result<()> {
        ctx.avails().add(|str: Arc<&str>| println!("{}", str));
        ctx("Hello From Plugin");
        Ok(())
    }
}
