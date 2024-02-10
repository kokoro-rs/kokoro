use kokoro::prelude::*;
use kokoro_default_impl::plugin::*;
fn main() {
    let ctx = Context::default();
    ctx.plugin(PF);
    ctx.publish(PhantomEvent);
    ctx.run();
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
