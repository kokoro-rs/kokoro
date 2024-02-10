use kokoro::default_implement::plugin::*;
use kokoro::prelude::*;

fn main() {
    let ctx = Context::default();
    ctx.plugin(P {
        content: "Hello Plugin".to_string(),
        message: "Bye World".to_string(),
    });
    ctx.plugin(N);
    ctx.run()
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
    println!("{}", ctx.content);
    println!("{}", ctx.message);
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
