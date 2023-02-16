/// OpenAI Model
///
/// Defaulted to the latest Codex model,
/// optimized by OpenAI for code.
///
/// https://platform.openai.com/docs/models/models
///
pub const OPENAI_MODEL: &str = "code-davinci-002";

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
