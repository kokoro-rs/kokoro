use kokoro::{
    context::scope::{LocalCache, Scope},
    mpsc,
    prelude::*,
};
use std::sync::Arc;
struct C;
impl LocalCache for C {}
#[test]
fn create_context() {
    let (_ctx, _) = Scope::build(Arc::new(C), |s| Context::new(s, mpsc()));
}
