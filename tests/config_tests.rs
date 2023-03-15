//!
//! Integration tests
//!
//! ### Todo
//!
//! - Mock out OpenAI API service
//!   - Test match with real service once a day
//!   - Drop-in replacement for ureq-implementation
//!   - Assert parts of API request
//!   - Provide response
//!
use std::env::{self, current_dir};
use std::path::PathBuf;
use std::process::Command;
use tempfile::tempdir;

use shai::{ConfigSourceFile, Result, HOME_CONFIG_DIR, OPENAI_API_KEY_ENV_KEY};

#[cfg(unix)]
const CONFIG_DIR_ENV_KEY: &str = "XDG_CONFIG_HOME";

#[cfg(windows)]
const CONFIG_DIR_ENV_KEY: &str = "APPDATA";

fn shai_command() -> Result<Command> {
    let binary_path: PathBuf = [current_dir()?, "target/debug/shai".into()]
        .iter()
        .collect();
    let mut command = Command::new(binary_path);
    command.env_clear();

    // `SystemRoot` environment variable is required
    // for working DNS queries on Windows. No clue why.
    if cfg!(windows) {
        command.env(
            "SystemRoot",
            env::var_os("SystemRoot")
                .expect("`SystemRoot` is a standard environment variable on Windows"),
        );
    }

    Ok(command)
}

#[test]
fn no_api_key() -> Result<()> {
    let config_dir = tempdir()?;

    let command_output = shai_command()?
        .args(["command", "path of largest file on system"])
        .env(CONFIG_DIR_ENV_KEY, config_dir.path())
        .output()?;

    assert!(command_output.stdout.is_empty());
    assert!(!command_output.stderr.is_empty());
    assert!(!command_output.status.success());

    Ok(())
}

#[test]
fn api_key_environment_variable() -> Result<()> {
    let api_key = env::var(OPENAI_API_KEY_ENV_KEY)?;
    let config_dir = tempdir()?;

    let command_output = shai_command()?
        .args(["command", "path of largest file on system"])
        .env(CONFIG_DIR_ENV_KEY, config_dir.path())
        .env(OPENAI_API_KEY_ENV_KEY, &api_key)
        .output()?;

    assert!(!command_output.stdout.is_empty());
    assert!(command_output.stderr.is_empty());
    assert!(command_output.status.success());

    Ok(())
}

#[test]
fn api_key_config_file() -> Result<()> {
    let api_key = env::var(OPENAI_API_KEY_ENV_KEY)?;
    let config_dir = tempdir()?;

    let configure_output = shai_command()?
        .args(["configure", "--openai-api-key", &api_key])
        .env(CONFIG_DIR_ENV_KEY, config_dir.path())
        .output()?;

    assert!(configure_output.stdout.is_empty());
    assert!(configure_output.stderr.is_empty());
    assert!(configure_output.status.success());

    let command_output = shai_command()?
        .args(["command", "path of largest file on system"])
        .env(CONFIG_DIR_ENV_KEY, config_dir.path())
        .output()?;

    assert!(!command_output.stdout.is_empty());
    assert!(command_output.stderr.is_empty());
    assert!(command_output.status.success());

    Ok(())
}

#[test]
fn can_construct_config_file_path() -> Result<()> {
    ConfigSourceFile::file_path()?;
    Ok(())
}

#[test]
#[cfg(unix)]
fn xdg_config_home() -> Result<()> {
    let api_key = env::var(OPENAI_API_KEY_ENV_KEY)?;
    let home_dir = tempdir()?;
    let xdg_config_home_dir: PathBuf = [home_dir.path(), HOME_CONFIG_DIR.as_ref()].iter().collect();

    shai_command()?
        .args(["configure", "--openai-api-key", &api_key])
        .env("HOME", home_dir.path())
        .output()?;

    let command_output = shai_command()?
        .args(["command", "path of largest file on system"])
        .env("XDG_CONFIG_HOME", xdg_config_home_dir)
        .output()?;

    assert!(!command_output.stdout.is_empty());
    assert!(command_output.stderr.is_empty());
    assert!(command_output.status.success());

    Ok(())
}

#[test]
#[cfg(unix)]
fn xdg_precedence() -> Result<()> {
    let api_key = env::var(OPENAI_API_KEY_ENV_KEY)?;
    let bad_home_dir = tempdir()?;
    let xdg_config_home_dir = tempdir()?;

    shai_command()?
        .args(["configure", "--openai-api-key", &api_key])
        .env("XDG_CONFIG_HOME", xdg_config_home_dir.path())
        .output()?;

    let command_output = shai_command()?
        .args(["command", "path of largest file on system"])
        .env("HOME", bad_home_dir.path())
        .env("XDG_CONFIG_HOME", xdg_config_home_dir.path())
        .output()?;

    assert!(!command_output.stdout.is_empty());
    assert!(command_output.stderr.is_empty());
    assert!(command_output.status.success());

    Ok(())
}
