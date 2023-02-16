use super::config_parts::ConfigParts;
use super::defaults;
use crate::error::Error;

pub struct ConfigComplete {
    pub api_key: String,
    pub max_tokens: u16,
    pub model: String,
    pub temperature: f32,
}

impl TryFrom<ConfigParts> for ConfigComplete {
    type Error = Error;

    fn try_from(parts: ConfigParts) -> Result<Self, Self::Error> {
        let api_key = parts.api_key.ok_or(Error::NoAPIKey)?;

        Ok(Self {
            api_key,
            max_tokens: parts.max_tokens.unwrap_or(defaults::OUTPUT_MAX_TOKENS),
            model: parts.model.unwrap_or_else(|| defaults::OPENAI_MODEL.into()),
            temperature: parts.temperature.unwrap_or(defaults::MODEL_TEMPERATURE),
        })
    }
}
