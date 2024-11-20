use std::fs;

use kokoro::definitions::later::{CommonLater, Later, Path};
use wasmtime::{component::*, Config, Engine};

fn main() -> anyhow::Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    let engine = Engine::new(&config)?;
    let file = fs::read("./demo.wasm")?;
    let component = Component::new(&engine, file)?;
    let component_type = component.component_type();
    let laters =
        CommonLater::from_component_types(None::<Path>, &engine, component_type.imports(&engine));
    for later in laters {
        match later {
            CommonLater::Func(func) => {
                println!("{:?}", func.path)
            }
        }
    }
    Ok(())
}
