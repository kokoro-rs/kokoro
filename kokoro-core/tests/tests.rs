use std::sync::Arc;

use kokoro_core::context::{scope::Scope, Context};
struct App;
#[test]
fn create_context() {
    let app = App.into();
    let _ctx = Context::create(Scope::create(Arc::clone(&app)).into(), app);
}
