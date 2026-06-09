use std::fs;
use std::path::PathBuf;

use clap::{Parser, Subcommand};

use egbo::model;
use egbo::model_builder;
use egbo::model_json;
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
    Parse {
        input: PathBuf,
        /// Include each node's source span in the output
        #[arg(long)]
        span: bool,
    },
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Command::Parse { input, span } => {
            let contents = fs::read_to_string(&input)?;

            let mut pparser = peg_parser::PegParserImpl::new(contents.as_str());
            let parser = parser::Parser::new();

            match parser.file(&mut pparser) {
                Some(parsed) => {
                    let mut model = model::Model::new();
                    let file = model_builder::add_file_to_model(
                        &input.display().to_string(),
                        &parsed,
                        &mut model,
                    );
                    let opts = if span {
                        model_json::DumpOpts::with_spans()
                    } else {
                        model_json::DumpOpts::default()
                    };
                    let json = model_json::file_to_json(&model, file, &opts);
                    println!("{json}");
                }
                None => {
                    eprintln!("Failed to parse {}", input.display());
                }
            }
        }
    }
    Ok(())
}
