use clap::Parser;

mod args;
mod config;
mod error;
mod process_result;
mod result;
mod subcommands;

use args::Args;
use process_result::ProcessResult;

fn main() -> ProcessResult<()> {
    match Args::parse() {
        Args::Command(args) => subcommands::command(args).into(),
        Args::Configure(args) => subcommands::configure(args).into(),
    }
}
