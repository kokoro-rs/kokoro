use std::sync::{Arc, RwLock};
use wasmtime::component::*;

pub type SharedLinker<T> = Arc<RwLock<Linker<T>>>;
