use anyhow::Result;
use kokoro_neo::{
    context::Context,
    plugin::{Pluggable, Plugin},
};
use std::sync::Arc;

fn main() -> Result<()> {
    let ctx: Context<_, String> = Context::new(());
    ctx.plug(MyPlugin)?;
    ctx("Hello Plugin".to_string());
    Ok(())
}
struct MyPlugin;
impl Plugin<String> for MyPlugin {
    fn load(ctx: kokoro_neo::context::Context<Self, String>) -> anyhow::Result<()> {
        ctx.avails().add(|str: Arc<String>| println!("{}", str));
        Ok(())
    }
}

