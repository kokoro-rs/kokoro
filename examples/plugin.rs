use kokoro::prelude::*;

struct App;
fn main() -> Result<()> {
    let ctx = mpsc_context(App);
    ctx.plugin(P {
        content: "Hello Plugin".to_string(),
        message: "Bye World".to_string(),
    })?;
    ctx.plugin(N)?;
    ctx.run();
    Ok(())
    /* Typically, the output will be :
     *  Hello Plugin
     *  Bye World
     */
}

struct P {
    content: String,
    message: String,
}

impl Plugin for P {
    type MODE = MPSC<App>;
    const NAME: &'static str = "P";
    fn apply(ctx: Context<Self, MPSC<App>>) -> Result<()> {
        ctx.subscribe(sub);
        Ok(())
    }
}

fn sub(ctx: &Context<P, MPSC<App>>) {
    println!("{}", ctx.content);
    println!("{}", ctx.message);
}

struct N;

impl Plugin for N {
    type MODE = MPSC<App>;
    const NAME: &'static str = "N";
    fn apply(ctx: Context<Self, MPSC<App>>) -> Result<()> {
        ctx.publish(PhantomEvent);
        Ok(())
    }
}
