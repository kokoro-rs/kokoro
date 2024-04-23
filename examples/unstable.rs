#![feature(fn_traits)]
#![feature(unboxed_closures)]
#![feature(tuple_trait)]
#![allow(unused)]

use std::marker::Tuple;
#[allow(non_camel_case_types)]
struct my_function;
trait Params: Tuple {
    fn get(self) -> ParamsEnum;
    fn get_clone(&self) -> ParamsEnum;
}

enum ParamsEnum {
    I32(i32),
    Str(String),
}
impl Params for (i32,) {
    fn get(self) -> ParamsEnum {
        ParamsEnum::I32(self.0)
    }
    fn get_clone(&self) -> ParamsEnum {
        ParamsEnum::I32(self.0)
    }
}
impl Params for (String,) {
    fn get(self) -> ParamsEnum {
        ParamsEnum::Str(self.0)
    }
    fn get_clone(&self) -> ParamsEnum {
        ParamsEnum::Str(self.0.clone())
    }
}
impl Params for (String, String) {
    fn get(self) -> ParamsEnum {
        ParamsEnum::Str(format!("{}{}", self.0, self.1))
    }
    fn get_clone(&self) -> ParamsEnum {
        ParamsEnum::Str(format!("{}{}", self.0, self.1))
    }
}

impl<P: Params> FnOnce<P> for my_function {
    type Output = ();

    extern "rust-call" fn call_once(self, args: P) -> Self::Output {
        match args.get() {
            ParamsEnum::I32(i) => println!("i32:\t{}", i),
            ParamsEnum::Str(s) => println!("String:\t{}", s),
        }
    }
}
impl<P: Params> FnMut<P> for my_function {
    extern "rust-call" fn call_mut(&mut self, args: P) -> Self::Output {
        match args.get_clone() {
            ParamsEnum::I32(i) => println!("i32:\t{}", i),
            ParamsEnum::Str(s) => println!("String:\t{}", s),
        }
    }
}
fn main() {
    my_function(1);
    my_function("hello".to_string());
    my_function("hello".to_string(), "bye".to_string());
}
