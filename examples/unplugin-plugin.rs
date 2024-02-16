use kokoro::prelude::*;

fn main() {
    let ctx = mpsc_context();
    for _ in 0..=1000000 {
        let id = ctx.plugin(Test);
        ctx.unplugin(id);
    }
    ctx.plugin(Test);
    ctx.publish(PhantomEvent);
    ctx.run();
}

struct Test;

impl Plugin for Test {
    type MODE = MPSC;
    const NAME: &'static str = "test";
    fn apply(ctx: Context<Self, MPSC>) {
        ctx.subscribe(sub);
    }
}

fn sub() {
    println!("Hello World");
}
