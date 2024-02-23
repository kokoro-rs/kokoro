use kokoro::prelude::*;
struct App;
fn main() {
    let ctx = mpsc_context(App);
    let sub_handle = ctx.subscribe(|p: &Print| println!("{}", p.0));
    //                   ^^^^^^^^^  Let's call this subscriber 's
    let thread_handle = ctx.spawn(|ctx, s| {
        //                  ^^^^^  Let's call this thread 't
        // s is a signal that is true when the thread should be terminated
        while !s.is() {
            ctx.publish(Print(&"Hello World"));
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
    });

    let _ = ctx.spawn(|ctx, _| {
        std::thread::sleep(std::time::Duration::from_secs(5));
        sub_handle.dispose();
        //         ^^^^^^^^^ 's will expire here
        ctx.subscribe(|p: &Print| println!("next: {}", p.0));
        std::thread::sleep(std::time::Duration::from_secs(5));
        dispose(thread_handle);
        //      ^^^^^^^^^^^^^ 't will expire here and join
    });

    ctx.run();
    /* Typically, the output will be :
     *  Hello World
     *  Hello World
     *  Hello World
     *  Hello World
     *  Hello World
     *  next: Hello World
     *  next: Hello World
     *  next: Hello World
     *  next: Hello World
     *  next: Hello World
     */
}

#[derive(Event)]
struct Print(&'static str);
