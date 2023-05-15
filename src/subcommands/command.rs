use serde::Deserialize;
use std::io::{self, stdout};
use ureq;

use crate::args;
use crate::config::{ConfigComplete, ConfigSource, ConfigSourceEnv, ConfigSourceFile};
use crate::error::Error;
use crate::result::Result;

/// OpenAI Completions API URL
///
/// https://platform.openai.com/docs/api-reference/chat/create
///
const OPENAI_API_URL: &str = "https://api.openai.com/v1/chat/completions";

#[derive(Deserialize)]
struct CompletionsResponse {
    choices: Vec<CompletionsResponseChoice>,
}

#[derive(Deserialize)]
struct CompletionsResponseChoice {
    message: CompletionsResponseMessage,
}

#[derive(Deserialize)]
struct CompletionsResponseMessage {
    content: String,
}

pub fn command(args: args::Command) -> Result<()> {
    let config: ConfigComplete = ConfigSourceFile::read_config()
        .merge(ConfigSourceEnv::read_config())
        .try_into()?;

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
        args.description
    );

    let result: CompletionsResponse = ureq::post(OPENAI_API_URL)
        .set("Content-Type", "application/json")
        .set(
            "Authorization",
            &format!("Bearer {}", &config.api_key.trim()),
        )
        .send_json(ureq::json!({
            "messages": [
                {
                    "content": prompt,
                    "role": "user",
                }
            ],
            "model": config.model,
            "temperature": config.temperature,
            "max_tokens": config.max_tokens,
            "stop": ["\r", "\n"],
        }))?
        .into_json()?;

    let first_result = &result
        .choices
        .first()
        .ok_or(Error::NoResults)?
        .message
        .content;
    io::copy(&mut first_result.as_bytes(), &mut stdout())?;

    Ok(())
}
