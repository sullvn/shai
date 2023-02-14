use clap::Parser;
use serde::Deserialize;
use std::env;
use std::io::{self, stdout};
use ureq;

mod error;
mod result;

use result::Result;

/// OpenAI Model
///
/// Defaulted to the latest Codex model,
/// optimized by OpenAI for code.
///
/// https://platform.openai.com/docs/models/models
///
const OPENAI_MODEL: &str = "code-davinci-002";

/// OpenAI Model Temperature
///
/// Sane default used for Codex.
///
/// https://platform.openai.com/docs/guides/code/best-practices
///
const OPENAI_MODEL_TEMPERATURE = 0.1;

/// OpenAI Completions API URL
///
/// https://platform.openai.com/docs/api-reference/completions/create
///
const OPENAI_API_URL: &str = "https://api.openai.com/v1/completions";

/// OpenAI API Key -- Environment Variable Key
///
/// Reuses name from official examples:
///
/// https://platform.openai.com/docs/quickstart/add-your-api-key
///
const OPENAI_API_KEY_ENV_KEY: &str = "OPENAI_API_KEY";

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
    let openai_api_key_env = env::var(OPENAI_API_KEY_ENV_KEY)?;
    let openai_api_key_env = openai_api_key_env.trim();

    let mut stdout = stdout();
    let Args { description } = Args::parse();

    let prompt = format!("# Bourne shell command. {}.\n$ ", description);

    let result: CompletionsResponse = ureq::post(OPENAI_API_URL)
        .set("Content-Type", "application/json")
        .set("Authorization", &format!("Bearer {}", openai_api_key_env))
        .send_json(ureq::json!({
            "prompt": prompt,
            "model": OPENAI_MODEL_TEMPERATURE,
            "temperature": 0.1,
        }))?
        .into_json()?;

    let first_result = &result.choices.first().expect("No results").text;
    io::copy(&mut first_result.as_bytes(), &mut stdout).expect("Could not copy");

    Ok(())
}
