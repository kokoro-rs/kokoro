use kokoro_neo::{context::Context, plugin::dynamic::*, result::*};

fn main() -> Result<()> {
    let ctx: Context<(), &'static str> = Context::new(());
    ctx.avails().add(|s: &str| {
        println!("from super {}", s);
    });
    ctx.dyn_plug("dynplugin")?;

    ctx("Bye");
    Ok(())
}
