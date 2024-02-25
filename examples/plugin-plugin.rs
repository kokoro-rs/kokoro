use kokoro::prelude::*;
fn main() -> Result<()> {
    let ctx = channel_ctx();
    ctx.plugin(PF)?;
    ctx.publish(PhantomEvent);
    ctx.run();
    Ok(())
    /* Typically, the output will be :
     *  Hello SF
     *  Hello PF
     *  From: SF
     *  From: PF
     */
}

struct PF;

impl Plugin for PF {
    type MODE = MPSC;
    const NAME: &'static str = "PF";
    fn apply(ctx: Context<Self, MPSC>) -> Result<()> {
        ctx.plugin(SF)?;
        ctx.subscribe(sub);
        println!("Hello PF");
        Ok(())
    }
}

struct SF;

impl Plugin for SF {
    type MODE = MPSC;
    const NAME: &'static str = "SF";
    fn apply(ctx: Context<Self, MPSC>) -> Result<()> {
        ctx.subscribe(sub);
        println!("Hello SF");
        Ok(())
    }
}

fn sub<P: Plugin<MODE = MPSC>>(_ctx: &Context<P, MPSC>) {
    println!("From: {}", P::NAME);
}
