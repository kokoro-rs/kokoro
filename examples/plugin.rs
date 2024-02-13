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
    const NAME: &'static str = "P";
    fn apply(ctx: Context<Self>) {
        ctx.subscribe(sub);
    }
}
fn sub(ctx: &Context<P>) {
    println!("{}", ctx.content);
    println!("{}", ctx.message);
}

struct N;
impl Plugin for N {
    const NAME: &'static str = "N";
    fn apply(ctx: Context<Self>) {
        ctx.publish(PhantomEvent);
    }
}
