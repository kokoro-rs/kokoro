use std::sync::Arc;

use kokoro::core::context::scope::Scope;
use kokoro::core::context::Context;
use kokoro::core::mpsc;
struct Local {
    something: u32,
}
impl Local {
    fn say(&self) {
        println!("{}", self.something)
    }
}
struct Other;
impl Other {
    fn cry(&self){
        println!("啊啊啊啊啊啊啊啊！")
    }
}
fn main() {
    let scope = Scope::create(Box::new(Local { something: 123 }));
    let ctx = Context::new(Arc::new(scope), mpsc());
    ctx.say();
    let ctx = ctx.with(Arc::new(Scope::create(Box::new(Other))));
    ctx.cry();
}
