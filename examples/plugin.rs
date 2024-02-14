use std::sync::Arc;
use kokoro::prelude::*;
use kokoro_core::context::scope::Scope;

fn main() {
    let scope = Scope::create(Box::new(Root::default()));
    let mode = MPSC::default();
    let ctx = Context::create(Arc::new(scope), Arc::new(mode));
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

impl Plugin<MPSC> for P {
    const NAME: &'static str = "P";
    fn apply(ctx: Context<Self, MPSC>) {
        ctx.subscribe(sub);
    }
}

fn sub(ctx: &Context<P, MPSC>) {
    println!("{}", ctx.content);
    println!("{}", ctx.message);
}

struct N;

impl Plugin<MPSC> for N {
    const NAME: &'static str = "N";
    fn apply(ctx: Context<Self, MPSC>) {
        ctx.publish(PhantomEvent);
    }
}
