use std::sync::Arc;

use kokoro::core::context::scope::Scope;
use kokoro::core::context::Context;
use kokoro::core::context::scope::{Resource, Triggerable};
use kokoro::core::event::Event;
use kokoro::core::event::EventID;
use kokoro::core::event::EventId;
use kokoro::core::disposable::dispose;
use kokoro::core::event::PhantomEvent;

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
    fn cry(&self) {
        println!("啊啊啊啊啊啊啊啊！")
    }
}

struct SyncMode {}

trait Publishable<E> {
    fn publish(&self, event: E);
}

impl<R: Resource + 'static, E> Publishable<E> for Context<R, SyncMode>
    where E: Event + Send + Sync + 'static {
    fn publish(&self, event: E) {
        let dyn_ctx = self.dynref();
        self.scope().trigger_recursive(Arc::new(event), dyn_ctx);
    }
}

fn main() {
    let scope = Scope::create(Arc::new(Local { something: 123 }));
    let ctx = Context::create(Arc::new(scope), Arc::new(SyncMode {}));
    ctx.say();
    let ctx = ctx.with(Arc::new(Scope::create(Arc::new(Other))));
    ctx.cry();
    let dh = ctx.subscribe(sub);
    ctx.publish(Test);
    dispose(dh);

    ctx.subscribe(|_: &Test| {
        println!("Bye World")
    });
    ctx.subscribe(|ctx: &Context<Other, SyncMode>| {
        ctx.publish(Test);
    });
    ctx.publish(PhantomEvent);
    // 我没实现 run 方法，因为这个例子用不到。
    // 从这个例子中我们可以看出：
    // 1. 事件总线实现不一定是事件循环，
    // 也可以是同步的发布订阅，即 channel 不是必须的。
    //
    // 2. 模式并不会随着上下文窗口改变，
    // 可以用于存储模式下所需的数据，
    // 比如默认实现 flume-channel 中的 Sender 和 Receiver
}

struct Test;

impl Event for Test {
    fn event_id(&self) -> &'static EventId {
        &EventId(12345)
    }
}

impl EventID for Test {
    const _EVENT_ID: &'static EventId = &EventId(12345);
}

fn sub(_: &Test) {
    println!("Hello World")
}
