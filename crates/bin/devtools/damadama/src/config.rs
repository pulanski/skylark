// config.rs

use crate::cli::OutputFormat;
use anyhow::{Context, Result};
use getset::{Getters, MutGetters, Setters};
use serde::Deserialize;
use std::path::{Path, PathBuf};
use thiserror::Error;

#[derive(Debug, Deserialize, Getters, MutGetters, Setters)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct Reindeer {
    /// The path to the reindeer.toml file. This is where all vendored
    /// dependencies are listed and the buckified dependencies are written
    /// to (e.g. `third-party/rust/reindeer.toml`).
    pub path: PathBuf,
}

impl Reindeer {
    pub fn directory(&self) -> Option<&Path> {
        self.path.parent()
    }
}

#[derive(Debug, Deserialize, Getters, MutGetters, Setters)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct Config {
    #[serde(rename = "reindeer")]
    pub reindeer_directory: Reindeer,

    /// The output format for the generated file (e.g. `BUCK`, `BUILD`, `BUILD.bazel`, or both)
    pub output_format: OutputFormat,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("Failed to read config file")]
    ReadFile,

    #[error("Failed to parse config file")]
    ParseFile,
}

pub fn load_config(config_path: &Path) -> Result<Config> {
    if !config_path.exists() {
        create_default_config(config_path)?;
    }

    if let Ok(content) = std::fs::read_to_string(config_path) {
        if let Ok(config) = toml::from_str(&content) {
            Ok(config)
        } else {
            Err(ConfigError::ParseFile.into())
        }
    } else {
        Err(ConfigError::ReadFile.into())
    }
}

fn create_default_config(config_path: &Path) -> Result<()> {
    tracing::info!("Creating default config file at {:?}", config_path);
    let default_content = r#"# General configurations

# The output format for the generated files.
output_format = "BUCK" # or "BUILD", "BUILD.bazel", or "BOTH"
# NOTE: This is case-insensitive (i.e. "BUCK" and "buck" are equivalent)

# Configurations specific to reindeer
[reindeer]
# The path to the reindeer.toml file This is where all vendored
# dependencies are listed and the buckified dependencies are written
# to.
path = "third-party/rust/reindeer.toml"
"#;
    std::fs::write(config_path, default_content).context(ConfigError::ReadFile)?;

    tracing::info!("Successfully created default config file");
    Ok(())
}
