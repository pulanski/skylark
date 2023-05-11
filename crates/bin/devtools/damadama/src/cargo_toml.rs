// cargo_toml.rs

use crate::{
    cli::OutputFormat,
    config::Config,
    generator::{bazel::BazelGenerator, buck::BuckGenerator, Generator},
    target::Target,
    task::{increment_completed_task, increment_in_progress_task},
};
use anyhow::Result;
use cargo_toml::Manifest;
use getset::{Getters, MutGetters, Setters};
use smartstring::alias::String;
use std::{
    collections::{BTreeMap, BTreeSet, HashSet},
    fmt::Debug,
    fs,
    io::Read,
    path::{Path, PathBuf},
    thread,
    time::Duration,
};
use thiserror::Error;
use typed_builder::TypedBuilder;
use walkdir::WalkDir;

#[derive(Debug, Error)]
pub enum CrateError {
    #[error("Failed to get the crate root directory")]
    CrateRootNotFound,
}

#[derive(
    Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Getters, MutGetters, Setters, TypedBuilder,
)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct Dependency {
    name: String,
    #[builder(default)]
    version: Option<String>,
}

impl Debug for Dependency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.version {
            Some(version) => write!(f, "{}:{}", self.name, version),
            None => write!(f, "{}", self.name),
        }
    }
}

fn extract_dependencies_from_manifest(manifest: &Manifest) -> BTreeSet<Dependency> {
    let mut deps = BTreeSet::new();

    extract_dependency_names(&manifest.dependencies, &mut deps);
    extract_dependency_names(&manifest.dev_dependencies, &mut deps);
    extract_dependency_names(&manifest.build_dependencies, &mut deps);

    if let Some(workspace) = &manifest.workspace {
        for member in &workspace.members {
            let manifest_path = format!("{member}/Cargo.toml");
            let member_manifest =
                Manifest::from_path(manifest_path).expect("Failed to parse Cargo.toml");
            extract_dependency_names(&member_manifest.dependencies, &mut deps);
            extract_dependency_names(&member_manifest.dev_dependencies, &mut deps);
            extract_dependency_names(&member_manifest.build_dependencies, &mut deps);
        }
    }

    deps
}

fn extract_dependency_names(
    dep_map: &BTreeMap<std::string::String, cargo_toml::Dependency>,
    deps: &mut BTreeSet<Dependency>,
) {
    for name in dep_map.keys() {
        deps.insert(Dependency::builder().name(name.to_string().into()).build());
    }
}

/// Recursively search for Cargo.toml files in a given directory and its subdirectories.
///
/// This function skips:
/// - Any directory containing a reindeer.toml file
/// - Any directory or its subdirectories with 'vendor' in the path
///
/// # Arguments
///
/// * `dir` - A reference to the directory path to search for Cargo.toml files
///
/// # Returns
///
/// A `Result` containing a `Vec` of `PathBuf` instances representing the paths of the Cargo.toml files found,
/// or an error if the directory could not be read.
pub fn find_cargo_toml_files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut cargo_toml_files = vec![];

    // If the current directory contains a reindeer.toml file, skip searching it and its subdirectories
    if dir.join("reindeer.toml").exists() {
        return Ok(cargo_toml_files);
    }

    // Iterate through the entries in the directory
    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();

        // If the entry is a directory and its path does not contain 'vendor', search its contents
        if path.is_dir() {
            if !path.to_string_lossy().contains("vendor") {
                cargo_toml_files.append(&mut find_cargo_toml_files(&path)?);
            }
        } else if path.is_file() && path.file_name().unwrap_or_default() == "Cargo.toml" {
            // If the entry is a file with the name 'Cargo.toml', add it to the result list
            cargo_toml_files.push(path);
        }
    }

    Ok(cargo_toml_files)
}

/// Find the workspace Cargo.toml file in a list of Cargo.toml files, if it exists.
pub fn find_ws_cargo_toml(cargo_toml_files: &[PathBuf]) -> Option<PathBuf> {
    cargo_toml_files
        .iter()
        .find(|path| is_workspace_cargo_toml(path).unwrap_or(false))
        .cloned()
}

pub fn is_workspace_cargo_toml(path: &PathBuf) -> Result<bool> {
    let manifest = Manifest::from_path(path)?;
    Ok(manifest.workspace.is_some())
}

/// **Extract dependecies** found within a **Rust package**, where a
/// **Rust package** is defined as a directory containing a `Cargo.toml` file.
#[tracing::instrument(level = "trace", skip(cargo_toml_path))]
pub fn extract_deps(cargo_toml_path: &PathBuf) -> Result<BTreeSet<Dependency>> {
    let manifest = Manifest::from_path(cargo_toml_path)?;
    let deps = extract_dependencies_from_manifest(&manifest);

    Ok(deps)
}

#[tracing::instrument(level = "trace", skip(cargo_toml_path, config))]
pub async fn process_and_generate(
    cargo_toml_path: &PathBuf,
    config: &Config,
    root_cargo_toml: &PathBuf,
) -> Result<()> {
    increment_in_progress_task();

    tracing::debug!(
        "Processing {} with config: {:?}",
        cargo_toml_path.display(),
        config
    );

    // Returns a flat list of dependencies from a Cargo.toml file.
    // This is a superset of the dependencies required to pass `cargo check`.
    // We then analyze the source files to determine which dependencies are actually used,
    // pruning the list to only those dependencies.
    let cargo_toml_deps = extract_deps(cargo_toml_path)?;

    tracing::debug!(
        "Found {} dependencies declared within Cargo.toml",
        cargo_toml_deps.len()
    );

    for dep in &cargo_toml_deps {
        tracing::debug!("\tDependency: {:?}", dep);
    }

    tracing::debug!("Analyzing source files...");
    thread::sleep(Duration::from_secs(1));

    let used_dependencies = analyze_source_files(cargo_toml_path, cargo_toml_deps).await?;

    for dep in &used_dependencies {
        tracing::debug!("\tDependency: {:?}", dep);
    }

    tracing::debug!("Generating BUCK file...");
    thread::sleep(Duration::from_secs(1));

    // Create a list of the targets to be generated
    let targets = Target::from_pruned_cargo_toml(
        used_dependencies,
        config.reindeer_directory(),
        root_cargo_toml,
    )?;

    let generator: Box<dyn Generator> = match config.output_format() {
        OutputFormat::Buck => Box::new(BuckGenerator::new()),
        OutputFormat::Build => Box::new(BazelGenerator::new()),
        OutputFormat::Both => todo!(),
    };

    // Generate the BUILD/BUCK file for the crate
    Generator::generate_build_file(&*generator, &targets)?;

    increment_completed_task();
    Ok(())
}

#[tracing::instrument(level = "trace", skip(cargo_toml_path, cargo_toml_deps))]
async fn analyze_source_files(
    cargo_toml_path: &Path,
    cargo_toml_deps: BTreeSet<Dependency>,
) -> Result<BTreeSet<Dependency>> {
    let mut used_dependencies = BTreeSet::new();
    let crate_root = cargo_toml_path
        .parent()
        .ok_or_else(|| anyhow::Error::from(CrateError::CrateRootNotFound))?;

    // Iterate through all .rs files in the crate
    for entry in WalkDir::new(crate_root)
        .into_iter()
        .filter_entry(|e| !should_skip_analysis(e))
        .filter_map(Result::ok)
        .filter(|e| e.path().extension().map_or(false, |ext| ext == "rs"))
    {
        let path = entry.path();
        tracing::debug!("Analyzing {}", path.display());

        let mut file = fs::File::open(path)?;
        let mut contents = std::string::String::new();
        file.read_to_string(&mut contents)?;

        // TODO:
        // Algorithm here needs to be improved a lot
        // needs to handle the use case of a multi-line use statement, e.g.
        // use std::{
        //    fs,
        //    io,
        //    path};
        // needs to handle the use case of a use statement with a trailing comma, in general
        // surrounding trivia needs to be discarded
        //
        // needs to discard any use statements which include `crate`, `self`, or `super`
        // needs to discard any use statements which are specific to the crate being analyzed
        // (e.g. `mod foo;` or `use foo::bar;` where foo is a module in the crate being analyzed)
        // in general, this process will probably be error prone, so we need a way of pruning
        // safely
        // for now, have this behind an opt-in feature flag so that we can get some real world
        // usage and figure out what the best approach is
        for line in contents.lines() {
            if line.trim().starts_with("use") || line.trim().starts_with("pub use") {
                tracing::debug!("\tInspecting use statement: [{}]", line);
                let dep_name = extract_dependency_name(line);
                if let Some(name) = dep_name {
                    used_dependencies.insert(Dependency::builder().name(name).build());
                }
            }
        }
    }

    tracing::debug!("Found {} used dependencies", used_dependencies.len());

    for dep in &used_dependencies {
        tracing::debug!("\tDependency: {:?}", dep);
    }

    // Intersect the two sets to get a pruned list of dependencies declared in Cargo.toml
    let pruned_dependencies: HashSet<_> = cargo_toml_deps
        .intersection(&used_dependencies)
        .cloned()
        .collect();

    tracing::debug!(
        "Pruned {} dependencies down to {}",
        cargo_toml_deps.len(),
        pruned_dependencies.len()
    );

    for dep in &pruned_dependencies {
        tracing::debug!("\tPruned Dependency: {:?}", dep);
    }

    Ok(cargo_toml_deps)
    // If feature flag is enabled, return the pruned list of dependencies
    // Ok(pruned_dependencies)
}

// Example use statements:
//
// TODO: need to define a blacklist of patterns/conditions for use statements to ignore
// use std::fs; => ignore (std/core/etc. are not dependencies from crates.io) (also, ignore self, super, crate)
// use ordered_float::OrderedFloat; => ordered_float is a dependency from crates.io, so include it
// use crate::ir::{ => ignore (crate is the crate being analyzed)
//   self,
//   Ir,
//   IrNode,
// };
// use salsa::debug::DebugWithDb; => salsa is a dependency from crates.io, so include it
// use serde::{Deserialize, Deserializer}; => serde is a dependency from crates.io, so include it
// TODO: add support for pruning unused dependencies from the Cargo.toml file itself
//
// TODO: advanced case, skipping modules declared in the crate being analyzed
// this will need to be implemented in a way that doesn't break the simple case,
// we want to minimize the number of false positives here as much as possible
// need to create a hashset of modules declared in the crate being analyzed
// and then check if the use statement is importing a module from that hashset
// in the event that it is, we can ignore it
// in the event that both the name can be qualified to both a module in the crate being analyzed
// and a dependency, we should emit a warning - found a use statement that could be ambiguous
// between a module in the crate being analyzed and a dependency from crates.io (intuition is that
// this shouldn't be possible)
// mod ir; => ignore (ir is a module in the crate being analyzed)
// use ir::Ir; => ignore (ir is a module in the crate being analyzed)
fn extract_dependency_name(line: &str) -> Option<String> {
    // In general, this extraction process needs to be improved a lot
    tracing::debug!("\t\tExtracting dependency name from: [{}]", line);
    let parts: Vec<&str> = line.split("::").collect();
    if parts.len() > 1 {
        tracing::debug!("\t\t\tFound dependency: {}", parts[0]);
        Some(parts[1].to_string().into())
    } else {
        tracing::debug!("\t\t\tNo dependency name found");
        None
    }
}

fn should_skip_analysis(entry: &walkdir::DirEntry) -> bool {
    if entry.file_type().is_dir() {
        let reindeer_toml = entry.path().join("reindeer.toml");
        let has_reindeer_toml = reindeer_toml.exists();
        let path_str = entry.path().to_str().unwrap_or("");
        let is_vendor_or_buck_out = path_str.contains("vendor")
            || path_str.contains("buck-out")
            || path_str.contains("target");

        return has_reindeer_toml || is_vendor_or_buck_out;
    }
    false
}
