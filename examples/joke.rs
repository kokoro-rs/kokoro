use kokoro::prelude::*;
use std::fmt::Display;
use std::thread::Builder;
struct App;
fn main() {
    let ctx = mpsc_context(App);
    ctx.subscribe(sub0);
    ctx.subscribe(sub1);
    ctx.subscribe(sub2);
    let builder = Builder::new()
        .stack_size(10 * 1024 * 1024 * 1024)
        .name("Joke".to_string());
    ctx.spawn_with_builder(builder, |ctx, _| {
        // 你与我之间的距离
        for i in '你'..='我' {
            ctx.publish(Char(i));
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

fn sub0(c: &Char) {
    println!("form sub0 {}", c);
    std::thread::sleep(std::time::Duration::from_secs(1));
    // 没错，他们是按顺序并发执行的。
    // 但是每轮Event结束时就会等待所有subscriber线程，所以请尽量不要故意阻塞！
}

fn sub1(c: &Char) {
    println!("form sub1 {}", c)
}

fn sub2(c: &Char) {
    println!("form sub2 {}", c)
}

