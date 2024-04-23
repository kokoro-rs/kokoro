#![feature(coerce_unsized)]
#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![feature(tuple_trait)]
#![feature(downcast_unchecked)]
#![allow(unused)]
use std::{
    any::Any,
    cell::RefCell,
    marker::{PhantomData, Tuple},
    ops::Deref,
    rc::{Rc, Weak},
};
fn main() {
    let root = Node::new("scope".to_string());
    let child = root.make_child(123);
    println!("{}:\t{}", **root, **child);
}

struct Node<T: Any + ?Sized> {
    scope: Rc<dyn Any>,
    children: RefCell<Vec<Rc<Node<dyn Any>>>>,
    parent: Weak<Node<dyn Any>>,
    _self: PhantomData<T>,
}
impl<T: Any> Node<T> {
    fn new(scope: T) -> Rc<Self> {
        let node = Self {
            scope: Rc::new(scope),
            children: RefCell::new(Vec::new()),
            parent: Weak::new(),
            _self: PhantomData,
        };
        Rc::new(node)
    }
}
trait NodeExt {
    fn make_child<N: Any>(&self, scope: N) -> Rc<Node<N>>;
    fn clone_dyn(&self) -> Rc<Node<dyn Any>>;
    unsafe fn borrow_dyn(&self) -> &Rc<Node<dyn Any>>;

    extern "rust-call" fn call_mut(&mut self, args: ()) -> ();
}
impl<T: Any + ?Sized> NodeExt for Rc<Node<T>> {
    fn make_child<N: Any>(&self, scope: N) -> Rc<Node<N>> {
        let node: Node<N> = Node {
            scope: Rc::new(scope),
            children: RefCell::new(Vec::new()),
            parent: Rc::downgrade(unsafe { self.borrow_dyn() }),
            _self: PhantomData,
        };
        let node = Rc::new(node);
        self.children.borrow_mut().push(node.clone_dyn());
        node
    }
    fn clone_dyn(&self) -> Rc<Node<dyn Any>> {
        unsafe { self.borrow_dyn() }.clone()
    }
    unsafe fn borrow_dyn(&self) -> &Rc<Node<dyn Any>> {
        unsafe { &*(self as *const Rc<Node<T>> as *const Rc<Node<dyn Any>>) }
    }

    extern "rust-call" fn call_mut(&mut self, args: ()) -> () {
        todo!()
    }
}
impl<T: Any> Deref for Node<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        unsafe { self.scope.downcast_ref_unchecked() }
    }
}

