use kokoro::prelude::*;

fn main() -> Result<()> {
    let ctx = channel_ctx();
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
    type MODE = MPSC;
    const NAME: &'static str = "P";
    fn apply(ctx: Context<Self, MPSC>) -> Result<()> {
        ctx.subscribe(sub);
        Ok(())
    }
}

fn sub(ctx: &Context<P, MPSC>) {
    println!("{}", ctx.content);
    println!("{}", ctx.message);
}

struct N;

impl Plugin for N {
    type MODE = MPSC;
    const NAME: &'static str = "N";
    fn apply(ctx: Context<Self, MPSC>) -> Result<()> {
        ctx.publish(PhantomEvent);
        Ok(())
    }
}
