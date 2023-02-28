use clap::Parser;
use shai::{subcommands, Args, ProcessResult};

fn main() -> ProcessResult<()> {
    match Args::parse() {
        Args::Command(args) => subcommands::command(args).into(),
        Args::Configure(args) => subcommands::configure(args).into(),
    }
}
