use anyhow::Result;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use wasmtime::component::*;
use wasmtime::{AsContextMut, Engine};
use wasmtime_wasi::WasiView;

use crate::definitions::later::Later;
use crate::definitions::types::SharedLinker;

pub trait InnerManager<T: WasiView> {
    fn engine(&self) -> &Engine;
    fn store(&mut self) -> impl AsContextMut<Data = T>;
    fn linker(&self) -> SharedLinker<T>;
    fn storing(&mut self, instance: Instance, name: &str);
    fn get_instance(&self, name: &str) -> Option<Arc<Instance>>;
}

pub trait Manager<T: WasiView + 'static, L: Later + 'static> {
    fn inner_mut(&mut self) -> &mut impl InnerManager<T>;
    fn inner(&self) -> &impl InnerManager<T>;
    fn engine(&self) -> &Engine {
        self.inner().engine()
    }
    fn store(&mut self) -> impl AsContextMut<Data = T> {
        self.inner_mut().store()
    }
    fn linker(&self) -> SharedLinker<T> {
        self.inner().linker()
    }
    fn storing(&mut self, instance: Instance, name: &str) {
        self.inner_mut().storing(instance, name);
    }
    fn get_instance(&self, name: &str) -> Option<Arc<Instance>> {
        self.inner().get_instance(name)
    }
    fn later(&mut self, later: L);
    fn laters_mut(&mut self) -> &mut Vec<L>;
    fn init_all_laters(&mut self) -> Result<()> {
        while let Some(later) = self.laters_mut().pop() {
            later.init(self.inner_mut())?;
        }
        Ok(())
    }
    fn load(&mut self, path: impl AsRef<Path>, name: &str) -> Result<()> {
        let wasm_file = fs::read(path)?;
        let component = Component::new(&self.engine(), &wasm_file)?;
        let linker = self.linker();
        let store = self.store();
        let ins = linker.read().unwrap().instantiate(store, &component)?;
        self.storing(ins, name);
        Ok(())
    }
}
