use kokoro::prelude::*;
use kokoro_core::context::scope::Resource;
struct App;
fn main() -> Result<()> {
    let ctx = mpsc_context(App);
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
    type MODE = MPSC<App>;
    const NAME: &'static str = "PF";
    fn apply(ctx: Context<Self, MPSC<App>>) -> Result<()> {
        ctx.plugin(SF)?;
        ctx.subscribe(sub);
        println!("Hello PF");
        Ok(())
    }
}

struct SF;

impl Plugin for SF {
    type MODE = MPSC<App>;
    const NAME: &'static str = "SF";
    fn apply(ctx: Context<Self, MPSC<App>>) -> Result<()> {
        ctx.subscribe(sub);
        println!("Hello SF");
        Ok(())
    }
}

fn sub<P: Plugin<MODE = MPSC<App>>, R: Resource>(_ctx: &Context<P, MPSC<R>>) {
    println!("From: {}", P::NAME);
}
