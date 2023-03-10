use std::env::{self, current_dir};
use std::fs::File;
use std::path::PathBuf;
use std::process::Command;
use tempfile::tempdir;

use shai::{Result, CONFIG_DIR_NAME, CONFIG_FILE_NAME, HOME_CONFIG_DIR, OPENAI_API_KEY_ENV_KEY};

// Setup helpers:
//
// - Get path of built binary
//   - Hardcode
// - Set environment variables
//   - Command::env
// - Get output
//   - Command::output
// - Create temporary directory
//
fn shai_command() -> Result<Command> {
    let binary_path: PathBuf = [current_dir()?, "target/debug/shai".into()]
        .iter()
        .collect();
    Ok(Command::new(binary_path))
}

// Integration tests:
//
// Now:
//
// - Get failure on no api key setup
#[test]
fn no_api_key() -> Result<()> {
    let home_dir = tempdir()?;
    let command_output = shai_command()?
        .args(["command", "path of largest file on system"])
        .env_clear()
        .env("HOME", home_dir.path())
        .output()?;

    assert!(command_output.stdout.is_empty());
    assert!(!command_output.stderr.is_empty());
    assert!(!command_output.status.success());

    Ok(())
}

// - API key via environment variable
//   - Call "command" with API key in environment variable
//   - Confirm valid output
#[test]
fn api_key_environment_variable() -> Result<()> {
    let api_key = env::var(OPENAI_API_KEY_ENV_KEY)?;
    let home_dir = tempdir()?;

    let command_output = shai_command()?
        .args(["command", "path of largest file on system"])
        .env_clear()
        .env("SystemRoot", env::var("SystemRoot").unwrap())
        .env("HOME", home_dir.path())
        .env(OPENAI_API_KEY_ENV_KEY, &api_key)
        .output()?;

    assert!(!command_output.stdout.is_empty());
    assert!(command_output.stderr.is_empty());
    assert!(command_output.status.success());

    Ok(())
}

// - API key via config file
//   - Set $HOME
//   - Call "configure" with api key
//   - Confirm the config file location
//   - Call "command"
//   - Confirm valid output
//   - Set $HOME to bad directory, set $XDG_CONFIG_HOME
//   - Call "command"
//   - Confirm valid output
#[test]
fn api_key_config_file() -> Result<()> {
    let api_key = env::var(OPENAI_API_KEY_ENV_KEY)?;
    let home_dir = tempdir()?;
    let not_home_dir = tempdir()?;
    let xdg_config_dir: PathBuf = [home_dir.path(), HOME_CONFIG_DIR.as_ref()].iter().collect();
    let config_path: PathBuf = [
        xdg_config_dir.as_path(),
        CONFIG_DIR_NAME.as_ref(),
        CONFIG_FILE_NAME.as_ref(),
    ]
    .iter()
    .collect();

    let configure_output = shai_command()?
        .args(["configure", "--openai-api-key", &api_key])
        .env_clear()
        .env("HOME", home_dir.path())
        .output()?;

    assert!(configure_output.stdout.is_empty());
    assert!(configure_output.stderr.is_empty());
    assert!(configure_output.status.success());
    assert!(File::open(config_path).is_ok());

    let command_output = shai_command()?
        .args(["command", "path of largest file on system"])
        .env_clear()
        .env("SystemRoot", env::var("SystemRoot").unwrap())
        .env("HOME", home_dir.path())
        .output()?;

    assert!(
        !command_output.stdout.is_empty(),
        "stdout='{:?}' stderr='{:?}' status='{:?}'",
        command_output.stdout,
        command_output.stderr,
        command_output.status.code(),
    );
    assert!(command_output.stderr.is_empty());
    assert!(command_output.status.success());

    let command_output = shai_command()?
        .args(["command", "path of largest file on system"])
        .env_clear()
        .env("SystemRoot", env::var("SystemRoot").unwrap())
        .env("HOME", not_home_dir.path())
        .env("XDG_CONFIG_HOME", xdg_config_dir)
        .output()?;

    assert!(!command_output.stdout.is_empty());
    assert!(command_output.stderr.is_empty());
    assert!(command_output.status.success());

    Ok(())
}
//
// Later:
//
// - Mock out OpenAI API service
//   - Test match with real service once a day
//   - Drop-in replacement for ureq-implementation
//   - Assert parts of API request
//   - Provide response
//
//
