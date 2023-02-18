use clap::Parser;

mod args;
mod config;
mod error;
mod result;
mod subcommands;

use args::Args;
use result::Result;

fn main() -> Result<()> {
    match Args::parse() {
        Args::Command(args) => subcommands::command(args),
        Args::Configure(args) => subcommands::configure(args),
    }
}
