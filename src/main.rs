use clap::Parser;
use serde::Deserialize;
use std::env;
use std::fs;
use std::io::{self, stdout};
use std::path::PathBuf;
use ureq;

mod error;
mod result;

use error::Error;
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
const OPENAI_MODEL_TEMPERATURE: f32 = 0.1;

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

/// Configuration directory name
///
/// Example: `shai`
/// Example in a path: `~/.config/shai`
///
const CONFIG_DIR_NAME: &str = "shai";

/// Configuration file name
const CONFIG_FILE_NAME: &str = "config";

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

fn openai_api_key_env() -> Result<String> {
    Ok(env::var(OPENAI_API_KEY_ENV_KEY)?)
}

fn config_file_path() -> Result<PathBuf> {
    let home_path = env::var("XDG_CONFIG_HOME").or(env::var("HOME"))?;

    let is_windows = true;
    let os_config_dir = if is_windows {
        "AppData/Local"
    } else {
        ".config"
    };

    Ok(
        [&home_path, os_config_dir, CONFIG_DIR_NAME, CONFIG_FILE_NAME]
            .iter()
            .collect(),
    )
}

fn openai_api_key_file() -> Result<String> {
    let cfg_path = config_file_path()?;
    Ok(fs::read_to_string(cfg_path)?)
}

fn main() -> Result<()> {
    let openai_api_key = openai_api_key_env()
        .or_else(|_| openai_api_key_file())
        .or(Err(Error::NoAPIKey))?;

    let mut stdout = stdout();
    let Args { description } = Args::parse();

    let prompt = format!("# Bourne shell command. {}.\n$ ", description);

    let result: CompletionsResponse = ureq::post(OPENAI_API_URL)
        .set("Content-Type", "application/json")
        .set("Authorization", &format!("Bearer {}", openai_api_key))
        .send_json(ureq::json!({
            "prompt": prompt,
            "model": OPENAI_MODEL,
            "temperature": OPENAI_MODEL_TEMPERATURE,
        }))?
        .into_json()?;

    let first_result = &result.choices.first().expect("No results").text;
    io::copy(&mut first_result.as_bytes(), &mut stdout).expect("Could not copy");

    Ok(())
}
