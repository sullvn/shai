use super::config_parts::ConfigParts;

pub trait ConfigSource {
    fn read_config() -> ConfigParts;
}
