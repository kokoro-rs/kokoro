#![allow(unused)]
use std::{
    any::type_name_of_val,
    cell::{OnceCell, RefCell},
    mem, ptr,
    rc::{Rc, Weak},
};

//use std::sync::{Arc as Rc, Weak};

// use dashmap::DashMap;

fn main() {
    let tree = Tree::new(1u32);
    let child = tree.make("hello", 2u32);
}
trait ENT {}
impl<T> ENT for T {}
struct Tree<T: ?Sized + ENT> {
    scope: Rc<T>,
    // childern: DashMap<&'static str, Rc<Tree<dyn ENT>>>,
    // childern: RefCell<Vec<Rc<Tree<dyn ENT>>>>,
    childern: OnceCell<Rc<Tree<dyn ENT>>>,
    parent: Weak<Tree<dyn ENT>>,
}
impl<T: ENT> Tree<T> {
    fn new(scope: T) -> Rc<Self> {
        let s = Self {
            scope: Rc::new(scope),
            // childern: DashMap::new(),
            // childern: vec![].into(),
            childern: OnceCell::new(),
            parent: Weak::new(),
        };
        Rc::new(s)
    }
}
impl<T: ENT + ?Sized> Drop for Tree<T> {
    fn drop(&mut self) {
        println!(
            "drop {}:\t{}",
            type_name_of_val(self),
            ptr::from_ref(&self.scope) as usize,
        );
    }
}
trait TreeExt {
    fn make<N: ENT>(&self, name: &'static str, scope: N) -> Rc<Tree<N>>;
}
impl<T: ENT> TreeExt for Rc<Tree<T>> {
    fn make<N: ENT>(&self, name: &'static str, scope: N) -> Rc<Tree<N>> {
        let rt = Rc::new(Tree {
            scope: Rc::new(scope),
            // childern: DashMap::new(),
            // childern: vec![].into(),
            childern: OnceCell::new(),
            parent: Rc::downgrade(unsafe {
                &*(self as *const Rc<Tree<T>> as *const Rc<Tree<dyn ENT>>)
            }),
            // parent: Weak::new(),
        });
        // self.childern.insert(
        //     name,
        //     Rc::clone(unsafe { &*(&rt as *const Rc<Tree<N>> as *const Rc<Tree<dyn ENT>>) }),
        // );
        // self.childern.borrow_mut().push(unsafe {
        //     mem::transmute(rt)
        // });
        // self.childern.borrow_mut().push(Rc::clone(unsafe {
        //     &*(&rt as *const Rc<Tree<N>> as *const Rc<Tree<dyn ENT>>)
        // }));
        self.childern.set(Rc::clone(unsafe {
            &*(&rt as *const Rc<Tree<N>> as *const Rc<Tree<dyn ENT>>)
        }));
        rt
    }
}
