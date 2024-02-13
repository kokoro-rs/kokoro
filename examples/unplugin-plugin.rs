use std::sync::Arc;

use kokoro::prelude::*;
use kokoro_core::context::scope::{Scope, ScopeId};

fn main() {
    let ctx = Context::default();
    for _ in 0..=1000000 {
        //let id = ctx.plugin(Test);
        //ctx.unplugin(id);
        //ctx.scope()
        //    .subscopes()
        //    .insert(id.clone(), Scope::create(Arc::new(Test), &ctx));
        //ctx.scope().subscopes().remove(&id);
        let scope = Scope::create(Arc::new(Test), &ctx);
        drop(scope);
    }
    ctx.plugin(Test);
    //for _ in 0..=1000000 {
    //    ctx.subscribe(sub).dispose();
    //}
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
