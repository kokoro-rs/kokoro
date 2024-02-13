use kokoro::prelude::*;

fn main() {
    let ctx = Context::default();
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
    fn apply(&self, ctx: &Context<Self>) {
        ctx.subscribe(sub);
    }

    fn name(&self) -> &'static str {
        "test"
    }
}
fn sub() {
    println!("Hello World");
}
