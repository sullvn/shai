use std::env;
use std::fs;
use std::path::PathBuf;

use super::config_parts::ConfigParts;
use super::config_source::ConfigSource;
use crate::result::Result;

/// Home config directory
///
/// Follows the de facto standard for
/// command line apps. In other words,
/// MacOS is treated the same as other
/// Unix flavors, instead of dumping
/// in `Library/Application Support`.
///
/// Neovim is a good example:
///
/// https://neovim.io/doc/user/starting.html#initialization
///
#[cfg(unix)]
const HOME_CONFIG_DIR: &str = ".config";
#[cfg(windows)]
const HOME_CONFIG_DIR: &str = "AppData/Local";

/// Configuration directory name
///
/// Example: `shai`
/// Example in a path: `~/.config/shai`
///
const CONFIG_DIR_NAME: &str = "shai";

/// Configuration file name
const CONFIG_FILE_NAME: &str = "config";

pub struct ConfigSourceFile {}

impl ConfigSourceFile {
    fn file_path() -> Result<PathBuf> {
        let home_path = env::var("XDG_CONFIG_HOME").or_else(|_| env::var("HOME"))?;

        Ok([
            &home_path,
            HOME_CONFIG_DIR,
            CONFIG_DIR_NAME,
            CONFIG_FILE_NAME,
        ]
        .iter()
        .collect())
    }
}

impl ConfigSource for ConfigSourceFile {
    fn read_config() -> ConfigParts {
        let mut parts = ConfigParts::default();
        parts.api_key = Self::file_path()
            .and_then(|fp| Ok(fs::read_to_string(fp)?))
            .ok();
        parts
    }
}
