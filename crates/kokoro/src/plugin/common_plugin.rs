use std::ops::Deref;

use wasmtime::component::*;

use super::plugin_trait::{Plugin, PluginInstance};
pub struct CommonPlugin {
    planet: Component,
}
impl Plugin for CommonPlugin {
    type Instance = CommonPluginInstance;
    fn planet(&self) -> &Component {
        &self.planet
    }
}

pub struct CommonPluginInstance {
    planet: Instance,
}

impl PluginInstance for CommonPluginInstance {
    fn planet(&self) -> &Instance {
        &self.planet
    }
}
impl Deref for CommonPluginInstance {
    type Target = Instance;
    fn deref(&self) -> &Self::Target {
        self.planet()
    }
}
