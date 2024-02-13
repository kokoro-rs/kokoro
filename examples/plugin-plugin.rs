use kokoro::prelude::*;
fn main() {
    let ctx = Context::default();
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

trait Eat {
    fn what(&self) -> &'static str;
}
struct PF;
impl Plugin for PF {
    const NAME: &'static str = "PF";
    fn apply(ctx: Context<Self>) {
        ctx.plugin(SF);
        ctx.subscribe(sub);
        println!("Hello PF");
    }
}

struct SF;
impl Plugin for SF {
    const NAME: &'static str = "SF";
    fn apply(ctx: Context<Self>) {
        ctx.subscribe(sub);
        println!("Hello SF");
    }
}

fn sub<P: Plugin + 'static>(_ctx: &Context<P>) {
    println!("From: {}", P::NAME);
}
