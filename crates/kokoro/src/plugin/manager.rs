use std::path::Path;

use wasmtime_wasi::WasiView;

use crate::{definitions::later::Later, manager::manager_trait::Manager};

pub trait PluginManager<T: WasiView + 'static, L: Later + 'static>: Manager<T, L> {
    fn load_plugin(&mut self, path: impl AsRef<Path>);
    fn init(&mut self);
}
