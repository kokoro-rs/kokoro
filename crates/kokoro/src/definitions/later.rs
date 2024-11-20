use std::{
    ops::Deref,
    sync::{Arc, OnceLock},
};

use anyhow::{anyhow, Result};
use nom::{
    bytes::{complete::take_until, streaming::take_till},
    character::complete::one_of,
    sequence::tuple,
    IResult,
};
use wasmtime::{component::*, Engine};
use wasmtime_wasi::WasiView;

use crate::manager::manager_trait::InnerManager;

use super::utils::extend_add;

#[derive(Debug, Clone, Copy)]
pub enum HostType {
    Planet,
    Satellite,
}

#[derive(Debug, Clone)]
pub struct Host(HostType, String);

impl Host {
    pub fn new(ty: HostType, name: String) -> Self {
        Host(ty, name)
    }
    pub fn from_str(input: &str) -> Result<Self> {
        fn parse_host_type(input: &str) -> IResult<&str, HostType> {
            let (input, ty) = take_until(":")(input)?;
            let ty = match ty {
                "planet" => HostType::Planet,
                "satellite" => HostType::Satellite,
                _ => {
                    return Err(nom::Err::Failure(nom::error::Error::new(
                        input,
                        nom::error::ErrorKind::Fail,
                    )));
                }
            };
            Ok((input, ty))
        }
        fn parse_name(input: &str) -> IResult<&str, String> {
            let (input, name) = take_till(|c| c == ':' || c == '/')(input)?;
            let (name, _) = one_of(":/")(name)?;
            Ok((input, name.to_string()))
        }
        if let Ok((_, (host_type, name))) = tuple((parse_host_type, parse_name))(input) {
            Ok(Self(host_type, name))
        } else {
            Err(anyhow!("Can not parse '{}' into Host", input))
        }
    }
    pub fn ty(&self) -> &HostType {
        &self.0
    }
    pub fn name(&self) -> &String {
        &self.1
    }
}

#[derive(Clone, Debug)]
pub struct Path(Vec<String>);
impl Path {
    pub fn new() -> Self {
        Self(Vec::new())
    }
    pub fn take(self) -> Vec<String> {
        self.0
    }
}
impl From<Vec<String>> for Path {
    fn from(value: Vec<String>) -> Self {
        Self(value)
    }
}
impl Deref for Path {
    type Target = Vec<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
pub trait Later: Sized {
    fn init<T: WasiView>(self, manager: &mut impl InnerManager<T>) -> anyhow::Result<()>;
    #[allow(unused)]
    fn from_component_types<'a>(
        leading_path: Option<impl Into<Path>>,
        engine: &Engine,
        ty: impl ExactSizeIterator<Item = (&'a str, types::ComponentItem)>,
    ) -> Vec<Self> {
        todo!()
    }
}

pub enum CommonLater {
    Func(LaterFunc),
}

impl Later for CommonLater {
    fn init<T: WasiView>(self, manager: &mut impl InnerManager<T>) -> anyhow::Result<()> {
        match self {
            CommonLater::Func(later) => {
                later.init(manager)?;
                Ok(())
            }
        }
    }
    fn from_component_types<'a>(
        leading_path: Option<impl Into<Path>>,
        engine: &Engine,
        ty: impl ExactSizeIterator<Item = (&'a str, types::ComponentItem)>,
    ) -> Vec<Self> {
        let path = leading_path.map(|this| this.into().take());
        let mut result = Vec::new();
        for (name, ty) in ty {
            if name.starts_with("wasi:") {
                continue;
            }
            match ty {
                types::ComponentItem::ComponentInstance(ci) => {
                    let tys = ci.exports(engine);
                    let mut other = Self::from_component_types(
                        Some(extend_add(path.clone(), name.to_string())),
                        engine,
                        tys,
                    );
                    result.append(&mut other);
                }
                types::ComponentItem::Component(c) => {
                    let tys = c.exports(engine);
                    let mut other = Self::from_component_types(
                        Some(extend_add(path.clone(), name.to_string())),
                        engine,
                        tys,
                    );
                    result.append(&mut other);
                }
                types::ComponentItem::CoreFunc(_) => {
                    result.push(Self::Func(LaterFunc::new(extend_add(
                        path.clone(),
                        name.to_string(),
                    ))));
                }
                types::ComponentItem::ComponentFunc(_) => {
                    result.push(Self::Func(LaterFunc::new(extend_add(
                        path.clone(),
                        name.to_string(),
                    ))));
                }
                _ => {
                    #[cfg(debug_assertions)]
                    println!(
                        "import-ignore: ->{}->{}: {:?}",
                        path.clone()
                            .map(|this| this.join("->"))
                            .unwrap_or("".to_string()),
                        &name,
                        &ty
                    );
                }
            }
        }
        result
    }
}

pub struct LaterFunc {
    pub path: Path,
    pub func: Arc<OnceLock<Func>>,
}
impl LaterFunc {
    fn new(path: impl Into<Path>) -> Self {
        Self {
            path: path.into(),
            func: Arc::new(OnceLock::new()),
        }
    }
}
impl Later for LaterFunc {
    fn init<T: WasiView>(self, manager: &mut impl InnerManager<T>) -> anyhow::Result<()> {
        let mut full_path = self.path.iter();
        let host_str = full_path.next().ok_or(anyhow!("Requires instance name"))?;
        let host = Host::from_str(host_str)?;
        match host.ty() {
            HostType::Planet => {
                let instance = manager
                    .get_instance(host.name())
                    .ok_or(anyhow!("Instance:{} does not exist.", host.name()))?;
                let mut store = manager.store();
                let mut export = instance
                    .get_export(&mut store, None, host_str)
                    .ok_or(anyhow!("Item:{} does not exist.", host_str))?;
                for path in full_path {
                    export = instance
                        .get_export(&mut store, Some(&export), path)
                        .ok_or(anyhow!("Item:{} does not exist.", path))?;
                }
                let full_name = self.path.join("->");
                let func = instance
                    .get_func(store, export)
                    .ok_or(anyhow!("Item:{} does not exist.", full_name))?;
                return if let Err(_) = self.func.set(func) {
                    Err(anyhow!("Item:{} already exist.", full_name))
                } else {
                    Ok(())
                };
            }
            HostType::Satellite => {
                todo!()
            }
        }
    }
}
