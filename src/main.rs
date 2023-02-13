use clap::{Parser, Subcommand};
use wasmtime::component::{bindgen, Component, Linker};
use wasmtime::{Config, Engine, Store};

bindgen!("task");

use host::*;

#[derive(Parser)]
#[command(author = "Jesse Wellenberg", version = "0.0.1", about = "CLI app for Alice", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Clone, Copy)]
struct HostState;

#[derive(Subcommand)]
enum Commands {
    Add { name: Option<String> },
    Run { id: i32 },
}

impl Host for HostState {
    fn test(&mut self) -> anyhow::Result<()> {
        println!("Howdyhay");
        Ok(())
    }
}

fn main() -> anyhow::Result<()> {
    let mut config = Config::new();
    config.wasm_component_model(true);
    
    let engine = Engine::new(&config)?;
    let mut linker: Linker<HostState> = Linker::new(&engine);
    
    host::add_to_linker(&mut linker, |ctx| ctx)?;

    let component = Component::from_file(&engine, "./tasks/compiled/demotask/demotask.component.wasm")?;
    let mut store = Store::new(&engine, HostState {});
    println!("hallo");

    let t = TaskWorld::instantiate(&mut store, &component, &linker)?.0;
    let init= t.task.call_init(&mut store)?;
    let run= t.task.call_init(&mut store)?;
    let dispose= t.task.call_init(&mut store)?;
    
    println!("Result of init was {}", init);
    println!("Result of run was {}", run);
    println!("Result of dispose was {}", dispose);

    Ok(())

    // match &cli.command {
    //     Commands::Add { name } => {
    //         println!("Add called with '{name:?}'");
    //         Ok(())
    //     }
    //     Commands::Run { id } => {
    //         println!("Run called with id '{id}'");
    //         Ok(())
    //     }
    // }
}
