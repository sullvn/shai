use crate::args;
use crate::config::ConfigSourceFile;
use crate::result::Result;

pub fn configure(args: args::Configure) -> Result<()> {
    ConfigSourceFile::write_config(&args.openai_api_key)
}
