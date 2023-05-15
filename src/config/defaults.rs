/// OpenAI Model
///
/// Default to the latest, universally
/// accessible model by OpenAI for code
/// generation.
///
/// https://platform.openai.com/docs/models/models
///
pub const OPENAI_MODEL: &str = "gpt-3.5-turbo";

/// Model Temperature
///
/// Sane default used for OpenAI Codex.
///
/// https://platform.openai.com/docs/guides/code/best-practices
///
pub const MODEL_TEMPERATURE: f32 = 0.1;

/// Output Max Tokens
///
/// Maximum amount of tokens to return
/// from the model output. Should be big
/// enough to contain any reasonable output.
///
/// Hand-tweaked.
///
pub const OUTPUT_MAX_TOKENS: u16 = 128;
