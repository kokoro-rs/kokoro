use std::sync::{Arc, OnceLock};

use anyhow::{anyhow, Ok, Result};
use wasmtime::component::*;
use wasmtime_wasi::WasiView;

use crate::manager::manager_trait::{InnerManager, Manager};

pub trait Later: Sized {
    fn init<T: WasiView>(&self, manager: &mut impl InnerManager<T>) -> anyhow::Result<()>;
}

pub enum CommonLater {
    Func(LaterFunc),
}

pub trait CreateCommonLater<T> {
    fn later_func(&mut self, interface: &str, name: &str) -> Result<()>;
}

impl<T: WasiView + 'static, M: Manager<T, CommonLater>> CreateCommonLater<T> for M {
    fn later_func(&mut self, interface: &str, name: &str) -> Result<()> {
        let func = Arc::new(OnceLock::new());
        let clone_func = func.clone();
        let clone_name = name.to_string();
        let clone_interface = interface.to_string();
        self.linker()
            .write()
            .unwrap()
            .instance(interface)?
            .func_new(name, move |mut s, p, r| {
                let fun: &Func = clone_func.get().ok_or(anyhow!(
                    "Function:{}#{} not initialized",
                    clone_interface,
                    clone_name
                ))?;
                fun.call(&mut s, p, r)?;
                fun.post_return(s)
            })?;
        let later = CommonLater::Func(LaterFunc {
            interface: interface.to_string(),
            name: name.to_string(),
            func,
        });
        self.later(later);
        Ok(())
    }
}

impl Later for CommonLater {
    fn init<T: WasiView>(&self, manager: &mut impl InnerManager<T>) -> anyhow::Result<()> {
        match self {
            CommonLater::Func(later) => {
                later.init(manager)?;
                Ok(())
            }
        }
    }
}

pub struct LaterFunc {
    interface: String,
    name: String,
    func: Arc<OnceLock<Func>>,
}
impl Later for LaterFunc {
    fn init<T: WasiView>(&self, manager: &mut impl InnerManager<T>) -> anyhow::Result<()> {
        let instance = manager
            .get_instance(&self.name)
            .ok_or(anyhow!("Instance:{} does not exist.", &self.name))?;
        let mut store = manager.store();
        let export = instance
            .get_export(&mut store, None, &self.interface)
            .ok_or(anyhow!("Interface:{} does not exist.", self.interface))?;
        let export = instance
            .get_export(&mut store, Some(&export), &self.name)
            .ok_or(anyhow!(
                "Function:{}#{} does not exist.",
                self.interface,
                self.name
            ))?;
        let func = instance.get_func(store, export).ok_or(anyhow!(
            "Function:{}#{} does not exist.",
            self.interface,
            self.name
        ))?;
        return if let Err(_) = self.func.set(func) {
            Err(anyhow!(
                "Function:{}#{} already exist.",
                self.interface,
                self.name
            ))
        } else {
            Ok(())
        };
    }
}
