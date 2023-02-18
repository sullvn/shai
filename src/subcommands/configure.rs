use std::io::{self, stdout};

use crate::args;
use crate::config::ConfigSourceFile;
use crate::result::Result;

const CONFIGURATION_INSTRUCTIONS: &str = "";

pub fn configure(args: args::Configure) -> Result<()> {
    match args.openai_api_key {
        None => configure_instructions(),
        Some(key) => configure_record(&key),
    }
}

pub fn configure_instructions() -> Result<()> {
    io::copy(&mut CONFIGURATION_INSTRUCTIONS.as_bytes(), &mut stdout())?;
    Ok(())
}

pub fn configure_record(openai_api_key: &str) -> Result<()> {
    ConfigSourceFile::write_config(openai_api_key)
}
