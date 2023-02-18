use clap::Parser;

#[derive(Parser)]
pub enum Args {
    Command(Command),
    Configure(Configure),
}

#[derive(Parser)]
pub struct Command {
    pub description: String,
}

#[derive(Parser)]
pub struct Configure {
    #[arg(long, value_name = "KEY")]
    pub openai_api_key: Option<String>,
}
