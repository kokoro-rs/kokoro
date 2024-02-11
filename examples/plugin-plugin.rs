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
    fn apply(&self, ctx: &Context<Self>) {
        ctx.plugin(SF);
        ctx.subscribe(sub);
        println!("Hello PF");
    }

    fn name(&self) -> &'static str {
        "PF"
    }
}

struct SF;
impl Plugin for SF {
    fn apply(&self, ctx: &Context<Self>) {
        ctx.subscribe(sub);
        println!("Hello SF");
    }

    fn name(&self) -> &'static str {
        "SF"
    }
}

fn sub(ctx: &Context<impl Plugin + 'static>) {
    println!("From: {}", ctx.name());
}
