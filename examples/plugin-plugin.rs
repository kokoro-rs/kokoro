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

struct PF;
impl Plugin for PF {
    const NAME: &'static str = "PF";
    fn apply(&self, ctx: &Context<Self>) {
        ctx.plugin(SF);
        ctx.subscribe(sub);
        println!("Hello PF");
    }
}

struct SF;
impl Plugin for SF {
    const NAME: &'static str = "SF";
    fn apply(&self, ctx: &Context<Self>) {
        ctx.subscribe(sub);
        println!("Hello SF");
    }
}

fn sub(ctx: &Context<impl Plugin + 'static>) {
    println!("From: {}", ctx.name());
    println!("From: {}", ctx.scope().cache().name());
}
