use kokoro::prelude::*;
struct App;
fn main() -> Result<()> {
    let ctx = mpsc_context(App);
    for _ in 0..=1000000 {
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
    type MODE = MPSC<App>;
    const NAME: &'static str = "test";
    fn apply(ctx: Context<Self, MPSC<App>>) -> Result<()> {
        ctx.subscribe(sub);
        Ok(())
    }
}

fn sub() {
    println!("Hello World");
}
