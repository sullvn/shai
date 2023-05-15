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
pub const HOME_CONFIG_DIR: &str = ".config";
#[cfg(windows)]
pub const HOME_CONFIG_DIR: &str = "AppData/Local";

/// Configuration directory name
///
/// Example: `shai`
/// Example in a path: `~/.config/shai`
///
pub const CONFIG_DIR_NAME: &str = env!("CARGO_PKG_NAME");

/// Configuration file name
pub const CONFIG_FILE_NAME: &str = "config";

pub struct ConfigSourceFile {}

impl ConfigSourceFile {
    #[cfg(unix)]
    pub fn file_path() -> Result<PathBuf> {
        //!
        //! ### Unix (Linux + macOS)
        //!
        //! - Adopt Linux practices on macOS
        //!   as that seems to be de facto
        //!   standard
        //! - Fallback to using $HOME as
        //!   $XDG_CONFIG_HOME isn't always
        //!   available
        //!
        if let Ok(xdg_config_home) = env::var("XDG_CONFIG_HOME") {
            let path = [&xdg_config_home, CONFIG_DIR_NAME, CONFIG_FILE_NAME]
                .iter()
                .collect();
            return Ok(path);
        }

        let path = [
            &env::var("HOME")?,
            HOME_CONFIG_DIR,
            CONFIG_DIR_NAME,
            CONFIG_FILE_NAME,
        ]
        .iter()
        .collect();
        Ok(path)
    }

    #[cfg(windows)]
    pub fn file_path() -> Result<PathBuf> {
        //!
        //! ### Windows
        //!
        //! - Prefer %APPDATA% as it is a standard
        //!   variable, and is made for application
        //!   configurations
        //! - Don't prefer %LOCALAPPDATA% as roaming
        //!   support is probably desirable
        //! - Don't fallback to anything, such as
        //!   %LOCALAPPDATA% or %USERPROFILE%. This
        //!   simplifies behavior.
        //!
        let path = [
            &env::var("APPDATA")?,
            CONFIG_DIR_NAME,
            "config",
            CONFIG_FILE_NAME,
        ]
        .iter()
        .collect();
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
