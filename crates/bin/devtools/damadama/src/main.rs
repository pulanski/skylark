// main.rs

mod cargo_toml;
mod cli;
mod config;
mod target;

use crate::{
    cargo_toml::{find_cargo_toml_files, process_cargo_toml},
    config::load_config,
};
use anyhow::Result;
use clap::Parser;
use cli::Cli;
use std::env;

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

    tracing::debug!(
        "Recursively searching for Cargo.toml files in {:?}",
        current_dir
    );

    let cargo_toml_files = find_cargo_toml_files(&current_dir)?;

    tracing::debug!("Found Cargo.toml files: {:#?}", cargo_toml_files);

    for cargo_toml_file in cargo_toml_files {
        process_cargo_toml(&cargo_toml_file, &config)?;
    }

    // let cargo_toml_data = parse_cargo_toml(&args.cargo_toml_path)?;
    // let used_dependencies = analyze_source_files(&cargo_toml_data);
    // let targets = Target::from_cargo_data_and_used_dependencies(
    //     cargo_toml_data,
    //     used_dependencies,
    //     &config.reindeer_directory,
    // );

    // ...

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
