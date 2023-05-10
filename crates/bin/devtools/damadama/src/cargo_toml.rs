// cargo_toml.rs

use anyhow::Result;
use cargo_toml::Manifest;
use smartstring::alias::String;
use std::{
    collections::BTreeMap,
    fmt::Debug,
    fs,
    path::{Path, PathBuf},
};
use typed_builder::TypedBuilder;

use crate::config::Config;

#[derive(TypedBuilder, PartialEq, PartialOrd, Eq, Ord, Clone, Hash)]
struct Dependency {
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

fn extract_dependencies(manifest: &Manifest) -> Vec<Dependency> {
    let mut deps = Vec::new();

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
    deps: &mut Vec<Dependency>,
) {
    for name in dep_map.keys() {
        deps.push(Dependency::builder().name(name.to_string().into()).build());
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

pub fn process_cargo_toml(cargo_toml_path: &Path, config: &Config) -> Result<()> {
    tracing::debug!("Processing {}", cargo_toml_path.display());
    // let cargo_toml_data = parse_cargo_toml(&cargo_toml_path)?;
    // let used_dependencies = analyze_source_files(&cargo_toml_data);
    // let targets = Target::from_cargo_data_and_used_dependencies(
    //     cargo_toml_data,
    //     used_dependencies,
    //     &config.reindeer_directory,
    // );

    // let output_path = cargo_toml_path.with_file_name("BUCK");
    // write_buck_file(&output_path, &targets)?;

    Ok(())
}
