use anyhow::Result;
use std::fs;
use std::sync::{Arc, RwLock};

use dashmap::*;
use wasmtime::{component::*, AsContextMut};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::{add_to_linker_sync, WasiView};

use crate::definitions::later::CommonLater;

use crate::definitions::types::SharedLinker;
use crate::manager::manager_trait::{InnerManager, Manager};

pub struct CommonInnerManager<T> {
    engine: Engine,
    store: Store<T>,
    linker: SharedLinker<T>,
    instances: DashMap<String, Arc<Instance>>,
}

pub struct CommonManager<T> {
    inner: CommonInnerManager<T>,
    laters: Vec<CommonLater>,
}

impl<T: WasiView> InnerManager<T> for CommonInnerManager<T> {
    type Instance = Instance;
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

    fn get_instance(&self, name: &str) -> Option<Arc<Instance>> {
        self.instances.get(&name.to_string()).map(|i| i.clone())
    }
}

impl<T: WasiView + 'static> Manager<T, CommonLater> for CommonManager<T> {
    type Inner = CommonInnerManager<T>;
    fn inner(&self) -> &CommonInnerManager<T> {
        &self.inner
    }
    fn inner_mut(&mut self) -> &mut CommonInnerManager<T> {
        &mut self.inner
    }
    fn later(&mut self, later: CommonLater) {
        self.laters.push(later);
    }
    fn laters_mut(&mut self) -> &mut Vec<CommonLater> {
        &mut self.laters
    }
    fn load(&mut self, path: impl AsRef<std::path::Path>, name: &str) -> Result<()> {
        let wasm_file = fs::read(path)?;
        let component = Component::new(&self.engine(), &wasm_file)?;
        let linker = self.linker();
        let store = self.store();
        let ins = linker.read().unwrap().instantiate(store, &component)?;
        self.storing(ins, name);
        Ok(())
    }
}

impl<T: WasiView> CommonInnerManager<T> {
    pub fn new(data: T) -> Result<Self> {
        let mut config = Config::new();
        config.wasm_component_model(true);
        let engine = Engine::new(&config)?;
        let mut linker: Linker<T> = Linker::new(&engine);
        linker.allow_shadowing(true);
        add_to_linker_sync(&mut linker)?;
        let store = Store::new(&engine, data);
        Ok(Self {
            engine,
            store,
            linker: Arc::new(RwLock::new(linker)),
            instances: DashMap::new(),
        })
    }
}

impl<T: WasiView> CommonManager<T> {
    pub fn new(data: T) -> Result<Self> {
        Ok(Self {
            inner: CommonInnerManager::new(data)?,
            laters: Vec::new(),
        })
    }
}
