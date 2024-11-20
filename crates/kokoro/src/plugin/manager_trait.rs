use std::{path::Path, sync::Arc};

use wasmtime_wasi::WasiView;

use anyhow::Result;

use crate::{definitions::later::Later, manager::manager_trait::Manager};

use super::plugin_trait::Plugin;

pub trait PluginManager<T, L>: Manager<T, L>
where
    T: WasiView + 'static,
    L: Later + 'static,
{
    type Plugin: Plugin;
    fn load_plugin(&mut self, plugin: impl Plugin) -> Result<()>;
    fn plugin_instance(&self, name: &str) -> Option<Arc<<Self::Plugin as Plugin>::Instance>>;
    fn init(&mut self) -> Result<()>;
}
