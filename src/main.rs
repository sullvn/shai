use clap::Parser;
use serde::Deserialize;
use std::io::{self, stdout};
use ureq;

/// OpenAI Model
///
/// Defaulted to the latest Codex model,
/// optimized by OpenAI for code.
///
/// https://platform.openai.com/docs/models/models
///
const OPENAI_MODEL: &str = "code-davinci-002";
const OPENAI_API_URL: &str = "https://api.openai.com/v1/completions";

const TEST_API_KEY: &str = "";

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

fn main() -> Result<(), ureq::Error> {
    let args = Args::parse();

    let mut stdout = stdout();
    let result: CompletionsResponse = ureq::post(OPENAI_API_URL)
        .set("Content-Type", "application/json")
        .set("Authorization", &format!("Bearer {}", TEST_API_KEY))
        .send_json(ureq::json!({
            "prompt": args.description,
            "model": OPENAI_MODEL,
            //
            // Sane default temperature for Codex
            //
            // https://platform.openai.com/docs/guides/code/best-practices
            //
            "temperature": 0.1,
        }))?
        .into_json()?;

    let first_result = &result.choices.first().expect("No results").text;
    io::copy(&mut first_result.as_bytes(), &mut stdout).expect("Could not copy");

    Ok(())
}
