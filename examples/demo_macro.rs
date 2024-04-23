#![allow(unused)]
use std::{any::Any, marker::PhantomData, mem};
macro_rules! exor {
    {$b1:expr;$b2:expr=>$r1:expr,$r2:expr,$r3:expr} => {
        if($b1 != $b2) {
            $r3
        }else{
            if($b1&&$b2){
                $r1
            }else{
                $r2
            }
        }
    };
}
fn main() {
    let i = exor!(false; false => 1, 2, 3);
    let y = exor!(true; true => 1, 2, 3);
    let a = exor!(false; true => 1, 2, 3);
    assert_eq!(i, 2);
    assert_eq!(y, 1);
    assert_eq!(a, 3);
}
