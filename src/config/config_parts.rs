use std::default::Default;

pub struct ConfigParts {
    pub api_key: Option<String>,
    pub max_tokens: Option<u16>,
    pub model: Option<String>,
    pub temperature: Option<f32>,
}

impl ConfigParts {
    fn merge(self, other: Self) -> Self {
        Self {
            api_key: other.api_key.or(self.api_key),
            max_tokens: other.max_tokens.or(self.max_tokens),
            model: other.model.or(self.model),
            temperature: other.temperature.or(self.temperature),
        }
    }
}

impl Default for ConfigParts {
    fn default() -> Self {
        ConfigParts {
            api_key: None,
            max_tokens: None,
            model: None,
            temperature: None,
        }
    }
}
