use kokoro_default_impl::plugin::*;
use kokoro::prelude::*;

fn main() {
    let ctx = Context::default();
    ctx.plugin(P {
        content: "Hello Plugin".to_string(),
        message: "Bye World".to_string(),
    });
    ctx.plugin(N);
    ctx.run();
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
    fn apply(&self, ctx: &Context<Self>) {
        ctx.subscribe(sub);
    }
    fn name(&self) -> &'static str {
        "P"
    }
}
fn sub(ctx: &Context<P>) {
    println!("{}", ctx.cache().unwrap().content);
    println!("{}", ctx.cache().unwrap().message);
}

struct N;
impl Plugin for N {
    fn apply(&self, ctx: &Context<Self>) {
        ctx.publish(PhantomEvent);
    }
    fn name(&self) -> &'static str {
        "N"
    }
}
