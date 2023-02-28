use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;

use super::config_parts::ConfigParts;
use super::config_source::ConfigSource;
use crate::error::Error;
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
        let path = match env::var("XDG_CONFIG_HOME") {
            Ok(xdg_config_home) => [&xdg_config_home, CONFIG_DIR_NAME, CONFIG_FILE_NAME]
                .iter()
                .collect(),
            Err(_) => [
                &env::var("HOME")?,
                HOME_CONFIG_DIR,
                CONFIG_DIR_NAME,
                CONFIG_FILE_NAME,
            ]
            .iter()
            .collect(),
        };

        Ok(path)
    }

    pub fn write_config(openai_api_key: &str) -> Result<()> {
        let path = Self::file_path()?;

        let mut file = match File::create(&path) {
            Err(err) if err.kind() == io::ErrorKind::NotFound => {
                let dir = path.parent().ok_or(Error::IO(io::Error::new(
                    io::ErrorKind::NotFound,
                    "Can't find appropriate directory for config",
                )))?;
                fs::create_dir_all(dir)?;
                File::create(path)?
            }
            Err(err) => return Err(Error::IO(err)),
            Ok(file) => file,
        };
        file.write_all(openai_api_key.as_bytes())?;

        Ok(())
    }
}

impl ConfigSource for ConfigSourceFile {
    fn read_config() -> ConfigParts {
        let api_key = Self::file_path()
            .and_then(|fp| Ok(fs::read_to_string(fp)?))
            .ok();
        ConfigParts {
            api_key,
            ..Default::default()
        }
    }
}
