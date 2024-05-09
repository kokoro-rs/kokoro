#![feature(
    tuple_trait,
    unboxed_closures,
    lazy_cell,
    fn_traits,
    const_type_name,
    effects
)]

pub mod any;
pub mod avail;
pub mod context;
pub mod plugin;
pub use anyhow as result;

#[test]
fn test() {
    use any::*;
    let foo: &dyn KAny = &String::from("hello");
    let bar = foo.downcast_ref::<String>();
    assert!(bar.is_some())
}
