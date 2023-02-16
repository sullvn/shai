use clap::Parser;
use serde::Deserialize;
use std::io::{self, stdout};
use ureq;

mod config;
mod error;
mod result;

use config::{ConfigComplete, ConfigSource, ConfigSourceEnv, ConfigSourceFile};
use result::Result;

/// OpenAI Completions API URL
///
/// https://platform.openai.com/docs/api-reference/completions/create
///
const OPENAI_API_URL: &str = "https://api.openai.com/v1/completions";

#[derive(Deserialize)]
struct CompletionsResponse {
    choices: Vec<CompletionsResponseChoice>,
}

#[derive(Deserialize)]
struct CompletionsResponseChoice {
    text: String,
}

#[derive(clap::Parser)]
struct Args {
    description: String,
}

fn main() -> Result<()> {
    let config: ConfigComplete = ConfigSourceFile::read_config()
        .merge(ConfigSourceEnv::read_config())
        .try_into()?;

    let mut stdout = stdout();
    let Args { description } = Args::parse();

    let prompt = format!(
        "# Bourne shell command
         #
         # The command does the following: {}.
         #
         # The command is one line, has no
         # comments, and does not use custom
         # shell scripts.
         #
         $ ",
        description
    );

    let result: CompletionsResponse = ureq::post(OPENAI_API_URL)
        .set("Content-Type", "application/json")
        .set(
            "Authorization",
            &format!("Bearer {}", &config.api_key.trim()),
        )
        .send_json(ureq::json!({
            "prompt": prompt,
            "model": config.model,
            "temperature": config.temperature,
            "max_tokens": config.max_tokens,
            "stop": ["\r", "\n"],
        }))?
        .into_json()?;

    let first_result = &result.choices.first().expect("No results").text;
    io::copy(&mut first_result.as_bytes(), &mut stdout).expect("Could not copy");

    Ok(())
}
