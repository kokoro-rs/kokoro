use kokoro::prelude::*;
fn main() -> Result<()> {
    let ctx = channel_ctx();
    for _ in 0..=1000 {
        let id = ctx.plugin(Test)?;
        ctx.unplug(id);
    }
    ctx.plugin(Test)?;
    ctx.publish(PhantomEvent);
    ctx.run();
    Ok(())
}

struct Test;

impl Plugin for Test {
    type MODE = MPSC;
    const NAME: &'static str = "test";
    fn apply(ctx: Context<Self, MPSC>) -> Result<()> {
        ctx.subscribe(sub);
        Ok(())
    }
}

fn sub() {
    println!("Hello World");
}
