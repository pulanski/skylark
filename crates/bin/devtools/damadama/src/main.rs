// main.rs

mod cargo_toml;
mod cli;
mod config;
mod generator;
mod target;

use crate::{
    cargo_toml::{
        find_cargo_toml_files, find_ws_cargo_toml, is_workspace_cargo_toml, process_cargo_toml,
    },
    config::load_config,
};
use anyhow::Result;
use clap::Parser;
use cli::Cli;
use std::path::{Path, PathBuf};
use std::{env, thread, time::Duration};

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
fn find_workspace_root(current_dir: &Path) -> Result<PathBuf> {
    let mut current_dir = current_dir.to_path_buf();
    let mut last_cargo_toml = None;

    loop {
        let cargo_toml_path = current_dir.join("Cargo.toml");
        if cargo_toml_path.exists() {
            if is_workspace_cargo_toml(&cargo_toml_path)? {
                return Ok(current_dir);
            } else {
                last_cargo_toml = Some(current_dir.clone());
            }
        }

        if !current_dir.pop() {
            break;
        }
    }

    match last_cargo_toml {
        Some(project_root) => Ok(project_root),
        None => anyhow::bail!(
            "No workspace Cargo.toml or standalone Rust project found in the current directory or its ancestors. Please run this command within a workspace or a Rust project."
        ),
    }
}

fn main() -> Result<()> {
    let args = Cli::parse();

    tracing_subscriber::fmt()
        .with_max_level(*args.verbosity())
        .init();

    tracing::info!("Starting up");
    tracing::debug!("Args: {:#?}", args);

    let config = load_config(args.config_path())?;
    tracing::debug!("Config: {:#?}", config);

    let current_dir = env::current_dir()?;
    let workspace_root = find_workspace_root(&current_dir)?;
    tracing::debug!("Found workspace root: {:?}", workspace_root);

    tracing::debug!(
        "Recursively searching for Cargo.toml files in {:?}",
        workspace_root
    );

    let cargo_toml_files = find_cargo_toml_files(&workspace_root)?;
    tracing::debug!("Found {} Cargo.toml files", cargo_toml_files.len());
    // tracing::debug!("Found Cargo.toml files: {:#?}", cargo_toml_files);

    let ws_cargo_toml = find_ws_cargo_toml(&cargo_toml_files);
    if let Some(ws_cargo_toml) = ws_cargo_toml {
        tracing::debug!("Found workspace Cargo.toml: {:?}", ws_cargo_toml);
    } else {
        tracing::debug!("No workspace Cargo.toml found");
    }

    thread::sleep(Duration::from_secs(10));

    for cargo_toml_file in cargo_toml_files {
        process_cargo_toml(&cargo_toml_file, &config)?;
    }

    Ok(())
}

// fn main() {
//     let ws_cargo_toml = "Cargo.toml";
//     let crate_cargo_toml = "crates/bin/devtools/damadama/Cargo.toml";

//     let ws_manifest = Manifest::from_path(ws_cargo_toml).expect("Failed to parse Cargo.toml");
//     let crate_manifest = Manifest::from_path(crate_cargo_toml).expect("Failed to parse Cargo.toml");

//     let ws_deps = extract_dependencies(&ws_manifest);
//     let crate_deps = extract_dependencies(&crate_manifest);

//     let ws_deps_set = ws_deps.iter().cloned().collect::<HashSet<Dependency>>();
//     let crate_deps_set = crate_deps.iter().cloned().collect::<HashSet<Dependency>>();

//     println!("ws_deps_set: {ws_deps_set:#?}");
//     println!("crate_deps_set: {crate_deps_set:#?}");
// }
