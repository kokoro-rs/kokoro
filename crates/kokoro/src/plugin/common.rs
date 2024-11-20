use std::sync::Arc;

use crate::{
    definitions::{later::CommonLater, types::SharedLinker},
    manager::manager_trait::{InnerManager, Manager},
};
use anyhow::Result;
use dashmap::DashMap;
use wasmtime::{component::*, AsContextMut, Engine, Store};
use wasmtime_wasi::WasiView;

use super::manager::PluginManager;

pub struct CommonInnerPluginManager<T> {
    engine: Engine,
    store: Store<T>,
    linker: SharedLinker<T>,
    instances: DashMap<String, Arc<Instance>>,
}

impl<T: WasiView> InnerManager<T> for CommonInnerPluginManager<T> {
    fn engine(&self) -> &Engine {
        &self.engine
    }

    fn store(&mut self) -> impl AsContextMut<Data = T> {
        &mut self.store
    }

    fn linker(&self) -> SharedLinker<T> {
        self.linker.clone()
    }

    fn storing(&mut self, instance: Instance, name: &str) {
        self.instances.insert(name.to_string(), Arc::new(instance));
    }

    fn get_instance(&self, name: &str) -> Option<Arc<Instance>> {
        self.instances.get(&name.to_string()).map(|i| i.clone())
    }
}

pub struct CommonPluginManager<T> {
    inner: CommonInnerPluginManager<T>,
    laters: Vec<CommonLater>,
}

impl<T: WasiView + 'static> Manager<T, CommonLater> for CommonPluginManager<T> {
    fn inner(&self) -> &impl InnerManager<T> {
        &self.inner
    }
    fn inner_mut(&mut self) -> &mut impl InnerManager<T> {
        &mut self.inner
    }
    fn later(&mut self, later: CommonLater) {
        self.laters.push(later);
    }
    fn laters_mut(&mut self) -> &mut Vec<CommonLater> {
        &mut self.laters
    }
}

impl<T: WasiView + 'static> PluginManager<T, CommonLater> for CommonPluginManager<T> {
    fn init(&mut self) -> Result<()> {
        self.init_all_laters()
    }
    fn load_plugin(&mut self, path: impl AsRef<std::path::Path>) -> Result<()> {
        todo!()
    }
}
