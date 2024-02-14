use kokoro::prelude::*;
fn main() {
    let ctx = mpsc_context();
    ctx.plugin(PF);
    ctx.publish(PhantomEvent);
    ctx.run();
    /* Typically, the output will be :
     *  Hello SF
     *  Hello PF
     *  From: SF
     *  From: PF
     */
}

struct PF;
impl Plugin<MPSC> for PF {
    const NAME: &'static str = "PF";
    fn apply(ctx: Context<Self, MPSC>) {
        ctx.plugin(SF);
        ctx.subscribe(sub);
        println!("Hello PF");
    }
}

struct SF;
impl Plugin<MPSC> for SF {
    const NAME: &'static str = "SF";
    fn apply(ctx: Context<Self, MPSC>) {
        ctx.subscribe(sub);
        println!("Hello SF");
    }
}

fn sub<P: Plugin<MPSC>>(_ctx: &Context<P, MPSC>) {
    println!("From: {}", P::NAME);
}
