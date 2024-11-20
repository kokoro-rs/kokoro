use anyhow::Result;
use wasmtime::component::*;
use wasmtime_wasi::WasiView;

use crate::manager::manager_trait::{AsInstance, InnerManager};

pub trait Plugin {
    type Instance: PluginInstance;
    fn planet(&self) -> &Component;
    fn instantiate<T: WasiView>(&mut self, manager: &mut impl InnerManager<T>) -> Result<Instance> {
        let linker = manager.linker();
        let linker = linker.read().unwrap();
        let instance = linker.instantiate(manager.store(), self.planet())?;
        Ok(instance)
    }
}

pub trait PluginInstance {
    fn planet(&self) -> &Instance;
}

impl<T: PluginInstance> AsInstance for T {
    fn as_instance(&self) -> &Instance {
        self.planet()
    }
}
