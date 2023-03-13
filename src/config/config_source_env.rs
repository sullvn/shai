use std::env;

use super::config_parts::ConfigParts;
use super::config_source::ConfigSource;

/// OpenAI API Key -- Environment Variable Key
///
/// Reuses name from official examples:
///
/// https://platform.openai.com/docs/quickstart/add-your-api-key
///
pub const OPENAI_API_KEY_ENV_KEY: &str = "OPENAI_API_KEY";

pub struct ConfigSourceEnv {}

impl ConfigSource for ConfigSourceEnv {
    fn read_config() -> ConfigParts {
        ConfigParts {
            api_key: env::var(OPENAI_API_KEY_ENV_KEY).ok(),
            ..Default::default()
        }
    }
}
