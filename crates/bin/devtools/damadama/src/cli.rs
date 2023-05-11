// cli.rs

use clap::Parser;
use derive_more::Display;
use getset::Getters;
use serde::{Deserialize, Deserializer};
use std::{path::PathBuf, str::FromStr};
use tracing::Level;

#[derive(Parser, Debug, Getters, PartialEq, Eq, Hash)]
#[command(
    author = "pulanski <iopulanski@gmail.com>",
    version = "0.1.0",
    about = "A tool for generating BUILD/BUCK files from Cargo.toml files. We are the knights who say Ni!",
    long_about = "A tool for generating BUILD/BUCK files from Cargo.toml files. Cargo is treated as the source of truth for dependencies, and the generated files are meant to be used with the [reindeer](https://github.com/facebookexperimental/reindeer) build tool in the context of Buck2 and potentially Bazel if reindeer is ever updated to support it.",
    bin_name = "dama"
)]
#[getset(get = "pub")]
pub struct Cli {
    /// The verbosity level to use for logging
    /// [default: info]
    #[clap(
        short = 'v',
        long,
        required = false,
        value_enum,
        default_value = "info"
    )]
    verbosity: Level,

    /// The output format for the generated file (e.g. `BUCK`, `BUILD`, or both)
    /// [default: BUCK]
    #[clap(
        short = 'f',
        long,
        required = false,
        default_value_t = OutputFormat::Buck
    )]
    output_format: OutputFormat,

    /// The path to the configuration file to use
    /// [default: dama.toml in the workspace/cell root]
    /// [env: DAMA_CONFIG_PATH]
    #[clap(short = 'c', long, required = false, default_value = "./dama.toml")]
    config_path: PathBuf,
}

#[derive(Debug, Display, Clone, PartialEq, Eq, Hash)]
pub enum OutputFormat {
    Buck,
    Build,
    Both,
}

impl<'de> Deserialize<'de> for OutputFormat {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: String = Deserialize::deserialize(deserializer)?;
        match s.to_ascii_lowercase().as_str() {
            "buck" => Ok(OutputFormat::Buck),
            "build" => Ok(OutputFormat::Build),
            "both" => Ok(OutputFormat::Both),
            _ => Err(serde::de::Error::custom(format!("Invalid value: {}", s))),
        }
    }
}

impl FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_lowercase().as_str() {
            "buck" => Ok(OutputFormat::Buck),
            "build" => Ok(OutputFormat::Build),
            "both" => Ok(OutputFormat::Both),
            _ => Err(format!("Invalid value: {s}")),
        }
    }
}
