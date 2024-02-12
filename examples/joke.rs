use std::fmt::Display;
use kokoro::prelude::*;
use std::thread::Builder;
fn main() {
    let ctx = Context::default();
    ctx.subscribe(sub);
    let builder = Builder::new()
        .stack_size(10 * 1024 * 1024 * 1024)
        .name("Joke".to_string());
    ctx.spawn_with_builder(builder, |ctx, _| {
        for i in 0x4e00..=0x952f {
            ctx.publish(Char(unsafe { char::from_u32_unchecked(i) }));
        }
    })
    .unwrap();
    ctx.run()
}
#[derive(Event)]
struct Char(char);
impl Display for Char {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
fn sub(_c: &Char) {}