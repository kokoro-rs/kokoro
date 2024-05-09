#![feature(
    tuple_trait,
    unboxed_closures,
    lazy_cell,
    fn_traits,
    const_trait_impl,
    const_mut_refs,
    const_hash,
    const_type_name,
    effects,
    hasher_prefixfree_extras
)]

pub mod any;
pub mod avail;
pub mod context;
pub mod plugin;

#[test]
fn test() {
    use any::*;
    let foo: &dyn IStableAny = &String::from("hello");
    let bar = foo.downcast_ref::<String>();
    assert!(bar.is_some())
}
