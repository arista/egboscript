use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "egbo")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Compile {
        input: PathBuf,
        output: PathBuf,
    },
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Compile { input, output } => {
            let contents = fs::read_to_string(&input)?;
            fs::write(&output, contents)?;
        }
    }
    Ok(())
}
