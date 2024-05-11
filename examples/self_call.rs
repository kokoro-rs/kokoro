use kokoro_neo::context::Context;
/// self_call
fn main() {
    let ctx: Context<(), ()> = Context::new(());
    ctx.avails().add(self_call);
    ctx(());
}
fn self_call(ctx: Context<(), ()>) {
    ctx(());
}
