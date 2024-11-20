use std::sync::{Arc, RwLock};

use crate::{
    definitions::{later::CommonLater, types::SharedLinker},
    manager::manager_trait::{InnerManager, Manager},
};
use anyhow::Result;
use dashmap::DashMap;
use wasmtime::{component::*, AsContextMut, Engine, Store};
use wasmtime_wasi::WasiView;

use super::{
    common_config::CommonPluginManagerConfig,
    common_plugin::{CommonPlugin, CommonPluginInstance},
    manager_trait::PluginManager,
    plugin_trait::Plugin,
};

pub struct CommonInnerPluginManager<T> {
    engine: Engine,
    store: Store<T>,
    linker: SharedLinker<T>,
    instances: DashMap<String, Arc<CommonPluginInstance>>,
}

impl<T: WasiView> CommonInnerPluginManager<T> {
    pub fn new(config: &CommonPluginManagerConfig, data: T) -> Result<Self> {
        let engine = Engine::new(&config.engine_config)?;
        let store = Store::new(&engine, data);
        let mut linker = Linker::new(&engine);
        wasmtime_wasi::add_to_linker_sync(&mut linker)?;
        Ok(Self {
            engine,
            store,
            linker: Arc::new(RwLock::new(linker)),
            instances: DashMap::new(),
        })
    }
}

impl<T: WasiView> InnerManager<T> for CommonInnerPluginManager<T> {
    type Instance = CommonPluginInstance;
    fn engine(&self) -> &Engine {
        &self.engine
    }

    fn store(&mut self) -> impl AsContextMut<Data = T> {
        &mut self.store
    }

    fn linker(&self) -> SharedLinker<T> {
        self.linker.clone()
    }

    fn storing(&mut self, instance: Self::Instance, name: &str) {
        self.instances.insert(name.to_string(), Arc::new(instance));
    }

    fn get_instance(&self, name: &str) -> Option<Arc<Self::Instance>> {
        self.instances.get(&name.to_string()).map(|i| i.clone())
    }
}

pub struct CommonPluginManager<T> {
    inner: CommonInnerPluginManager<T>,
    laters: Vec<CommonLater>,
}

impl<T: WasiView> CommonPluginManager<T> {
    pub fn new(config: &CommonPluginManagerConfig, data: T) -> Result<Self> {
        Ok(CommonPluginManager {
            inner: CommonInnerPluginManager::new(config, data)?,
            laters: Vec::new(),
        })
    }
}

impl<T: WasiView + 'static> Manager<T, CommonLater> for CommonPluginManager<T> {
    type Inner = CommonInnerPluginManager<T>;
    fn inner(&self) -> &CommonInnerPluginManager<T> {
        &self.inner
    }
    fn inner_mut(&mut self) -> &mut CommonInnerPluginManager<T> {
        &mut self.inner
    }
    fn later(&mut self, later: CommonLater) {
        self.laters.push(later);
    }
    fn laters_mut(&mut self) -> &mut Vec<CommonLater> {
        &mut self.laters
    }
    fn load(&mut self, path: impl AsRef<std::path::Path>, name: &str) -> Result<()> {
        todo!()
    }
}

impl<T: WasiView + 'static> PluginManager<T, CommonLater> for CommonPluginManager<T> {
    type Plugin = CommonPlugin;
    fn init(&mut self) -> Result<()> {
        self.init_all_laters()
    }
    fn load_plugin(&mut self, plugin: impl Plugin) -> Result<()> {
        todo!()
    }
    fn plugin_instance(&self, name: &str) -> Option<Arc<<CommonPlugin as Plugin>::Instance>> {
        self.get_instance(name)
    }
}
