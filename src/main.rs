use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use egbo::parser;
use egbo::peg_parser;

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

            let mut pparser = peg_parser::PegParserImpl::new(contents.as_str());
            let parser = parser::Parser::new();
            let result = parser.file(&mut pparser);

            println!("Result: {:#?}", result);
            
            fs::write(&output, contents)?;
        }
    }
    Ok(())
}
