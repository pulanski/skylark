// main.rs

mod cargo_toml;
mod cli;
mod config;
mod generator;
mod package;
mod target;
mod task;
mod util;

use crate::{
    cargo_toml::{find_cargo_toml_files, is_workspace_cargo_toml, process_and_generate},
    config::load_config,
    util::{color_start, elapsed_subsec, RIGHT_ARROW_SYMBOL},
};
use anyhow::Result;
use clap::Parser;
use cli::Cli;
use derive_new::new;
use getset::{Getters, MutGetters, Setters};
use humantime::format_rfc3339;
use indicatif::{ProgressState, ProgressStyle};
use std::{
    env,
    path::{Path, PathBuf},
    process::ExitCode,
    time::{Duration, SystemTime},
};
use tracing::{metadata::LevelFilter, Level};
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::{fmt::format::FmtSpan, layer::SubscriberExt};
use tracing_subscriber::{
    fmt::{format::DefaultFields, time::Uptime},
    util::SubscriberInitExt,
};
use typed_builder::TypedBuilder;

/// Finds the root of a Rust workspace or a standalone Rust project (i.e. a
/// rust binary or library similar to `cargo new --lib` or `cargo new --bin`).
///
/// Starts from the given `current_dir` and walks up the file system directory
/// tree looking for a `Cargo.toml` file. If it finds a `Cargo.toml` file with a
/// `[workspace]` table, it returns the path of the directory containing that
/// file. If no workspace is found, but a `Cargo.toml` file is found, it returns
/// the path of the directory containing the `Cargo.toml` file. If neither a
/// workspace nor a standalone Rust project is found, an error is returned.
///
/// # Arguments
///
/// * `current_dir` - The directory to start the search from.
///
/// # Errors
///
/// Returns an error if neither a workspace nor a standalone Rust project is
/// found in the current directory or its ancestors.
fn find_build_root(current_dir: &Path) -> Result<(PathBuf, bool)> {
    let mut current_dir = current_dir.to_path_buf();
    let mut last_cargo_toml = None;

    loop {
        let cargo_toml_path = current_dir.join("Cargo.toml");
        if cargo_toml_path.exists() {
            if is_workspace_cargo_toml(&cargo_toml_path)? {
                return Ok((current_dir, true));
            } else {
                last_cargo_toml = Some(current_dir.clone());
            }
        }

        if !current_dir.pop() {
            break;
        }
    }

    match last_cargo_toml {
        Some(project_root) => Ok((project_root, false)),
        None => anyhow::bail!(
            "No workspace Cargo.toml or standalone Rust project found in the current directory or its ancestors. Please run this command within a workspace or a Rust project."
        ),
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, new, TypedBuilder, Getters, MutGetters, Setters)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct Crate {
    name: String,
    version: Option<String>,
    path: PathBuf,
}

#[tokio::main]
async fn main() -> Result<ExitCode> {
    let args = Cli::parse();

    let indicatif_layer = IndicatifLayer::new()
        .with_progress_style(
            ProgressStyle::with_template(
                r"{spinner:.green}{color_start}{span_child_prefix}{span_fields} -- {span_name}{wide_msg}{elapsed_subsec}{color_end}",
            )
            .expect("Failed to initialize TUI")
            .tick_strings(&[
            "    ",
            "◐   ",
            "◓   ",
			"=◑  ",
			"==◒ ",
			"===◐",
			" ===",
			"  ==",
			"   =",
			"    ",
			"   ◓",
			"  ◑=",
			" ◒==",
			"◐===",
			"◓== ",
			"◑=  ",
			"◒   ",
            "    ",])
            .with_key("elapsed_subsec", elapsed_subsec)
            .with_key("color_start", color_start)
            .with_key(
                "color_end",
                |state: &ProgressState, writer: &mut dyn std::fmt::Write| {
                    if state.elapsed() > Duration::from_secs(4) {
                        let _ = write!(writer, "\x1b[0m");
                    }
                },
            )
        )
        .with_span_child_prefix_symbol(&RIGHT_ARROW_SYMBOL)
        .with_span_child_prefix_indent(" ");

    // tracing_subscriber::fmt()
    //     .with_max_level(*args.verbosity())
    //     .init();
    let max_level_filter = match *args.verbosity() {
        Level::ERROR => LevelFilter::ERROR,
        Level::WARN => LevelFilter::WARN,
        Level::INFO => LevelFilter::INFO,
        Level::DEBUG => LevelFilter::DEBUG,
        Level::TRACE => LevelFilter::TRACE,
    };

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(indicatif_layer.get_stderr_writer())
                .fmt_fields(DefaultFields::new())
                .with_line_number(true)
                // .without_time()
                .with_span_events(FmtSpan::CLOSE)
                .with_ansi(true)
                .with_timer(Uptime::default()),
        )
        .with(max_level_filter)
        .with(indicatif_layer)
        .init();

    let start_time = SystemTime::now();
    let formatted_start_time = format_rfc3339(start_time);
    tracing::info!("Starting at {}", formatted_start_time);
    tracing::debug!("Args:\n{:#?}", args);

    let config = load_config(args.config_path())?;
    tracing::debug!("Config:\n{:#?}", config);

    let current_dir = env::current_dir()?;
    let (root_dir, is_workspace) = find_build_root(&current_dir)?;
    if is_workspace {
        tracing::trace!("Found workspace root: {:?}", root_dir);
    } else {
        tracing::trace!("Found standalone project root: {:?}", root_dir);
    }

    // workspace name is the last component of the root directory
    let workspace_name = root_dir
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    tracing::debug!("Workspace/Cell name: {}", workspace_name);

    tracing::debug!(
        "Recursively searching for Cargo.toml files in {}",
        workspace_name
    );

    let cargo_toml_files = find_cargo_toml_files(&root_dir)?;
    tracing::debug!("Found {} Cargo.toml files", cargo_toml_files.len());

    for cargo_toml_file in cargo_toml_files {
        tracing::debug!("Buckifying {:?}", cargo_toml_file);
        process_and_generate(&cargo_toml_file, &config, &root_dir).await?;
    }

    Ok(ExitCode::SUCCESS)
}
