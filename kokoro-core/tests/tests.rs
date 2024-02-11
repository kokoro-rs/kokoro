use kokoro_core::{context::scope::Scope, context::Context, mpsc};
use std::sync::Arc;
struct C;
#[test]
fn create_context() {
    let (_ctx, _) = Scope::build(Arc::new(C), |s| Context::new(s, mpsc()));
}
