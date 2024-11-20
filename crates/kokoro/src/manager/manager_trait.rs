use anyhow::Result;
use std::fs;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;
use types::ComponentInstance;
use wasmtime::component::*;
use wasmtime::{AsContextMut, Engine};
use wasmtime_wasi::WasiView;

use crate::definitions::later::Later;
use crate::definitions::types::SharedLinker;

pub trait AsInstance {
    fn as_instance(&self) -> &Instance;
}

impl AsInstance for Instance {
    fn as_instance(&self) -> &Instance {
        self
    }
}

pub trait InnerManager<T: WasiView> {
    type Instance: AsInstance;
    fn engine(&self) -> &Engine;
    fn store(&mut self) -> impl AsContextMut<Data = T>;
    fn linker(&self) -> SharedLinker<T>;
    fn storing(&mut self, instance: Self::Instance, name: &str);
    fn get_instance(&self, name: &str) -> Option<Arc<Self::Instance>>;
}

pub trait Manager<T: WasiView + 'static, L: Later + 'static> {
    type Inner: InnerManager<T> + 'static;
    fn inner_mut(&mut self) -> &mut Self::Inner;
    fn inner(&self) -> &Self::Inner;
    fn engine(&self) -> &Engine {
        self.inner().engine()
    }
    fn store(&mut self) -> impl AsContextMut<Data = T> {
        self.inner_mut().store()
    }
    fn linker(&self) -> SharedLinker<T> {
        self.inner().linker()
    }
    fn storing(&mut self, instance: <Self::Inner as InnerManager<T>>::Instance, name: &str) {
        self.inner_mut().storing(instance, name);
    }
    fn get_instance(&self, name: &str) -> Option<Arc<<Self::Inner as InnerManager<T>>::Instance>> {
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
    fn load(&mut self, path: impl AsRef<Path>, name: &str) -> Result<()>;
}
