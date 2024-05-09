use kokoro_neo::any::*;

fn main() {
    let value: &dyn StableAny = &MyType;
    let foo = value.downcast_ref::<MyType>();
    assert!(foo.is_some());
    let bar = value.downcast_ref::<()>();
    assert!(bar.is_none());
}
struct MyType;
