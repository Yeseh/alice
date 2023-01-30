use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(author = "Jesse Wellenberg", version = "0.0.1", about = "CLI app for Alice", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add { name: Option<String> },
    Run { id: i32 },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { name } => {
            println!("Add called with '{name:?}'")
        }
        Commands::Run { id } => {
            println!("Run called with id '{id}'")
        }
    }
}
