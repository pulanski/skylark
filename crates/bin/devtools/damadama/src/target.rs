// target.rs

use crate::cargo_toml::{extract_deps, Dependency};
use crate::config::Reindeer;
use anyhow::Result;
use derive_more::Display;
use getset::{Getters, MutGetters, Setters};
use smartstring::alias::String;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::path::PathBuf;
use std::thread;
use std::time::Duration;
use thiserror::Error;
use typed_builder::TypedBuilder;

// Right now we want targets that are just Rust-specific, but in the future we might want to
// be generic across languages (e.g. C++ targets, Java targets, etc.).
#[derive(Debug)]
pub struct Target {
    name: String,
    kind: TargetKind,
    deps: Vec<CanonicalDependency>,
    config: TargetConfiguration,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetKind {
    Binary,
    Library,
    Test,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetConfiguration {
    Rust(RustTargetConfiguration),
    // potential support for other languages here...
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RustTargetConfiguration {
    Binary(RustBinaryConfiguration),
    Library(RustLibraryConfiguration),
    Test(RustTestConfiguration),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RustBinaryConfiguration {
    Buck2(Buck2RustBinaryAttributes),
    Bazel(BazelRustBinaryAttributes),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RustLibraryConfiguration {
    Buck2(Buck2RustLibraryAttributes),
    Bazel(BazelRustLibraryAttributes),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RustTestConfiguration {
    Buck2(Buck2RustTestAttributes),
    Bazel(BazelRustTestAttributes),
}

/// **Attributes** for a `rust_binary` target in the context of **Buck2**.
/// This includes all attributes except for `name` and `deps` which are
/// handled within the `Target` itself.
///
/// See https://buck2.build/docs/api/rules/#rust_binary for more information.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Buck2RustBinaryAttributes {
    default_target_platform: Option<String>,
    target_compatible_with: Vec<String>,
    compatible_with: Vec<String>,
    exec_compatible_with: Vec<String>,
    visibility: Vec<String>,
    tests: Vec<String>,
    crate_root: Option<String>,
    edition: Option<String>,
    env: HashMap<String, String>,
    features: Vec<String>,
    link_style: Option<String>,
    linker_flags: Vec<String>,
    mapped_srcs: HashMap<String, String>,
    named_deps: HashMap<String, String>,
    rpath: bool,
    rustc_flags: Vec<String>,
    srcs: Vec<String>,
}

/// **Attributes** for a `rust_library` target in the context of **Buck2**.
/// This includes all attributes except for `name` and `deps` which are
/// handled within the `Target` itself.
///
/// See https://buck2.build/docs/api/rules/#rust_library for more information.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Buck2RustLibraryAttributes {
    // ...
}

/// **Attributes** for a `rust_test` target in the context of **Buck2**.
/// This includes all attributes except for `name` and `deps` which are
/// handled within the `Target` itself.
///
/// See https://buck2.build/docs/api/rules/#rust_test for more information.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Buck2RustTestAttributes {
    // ...
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BazelRustBinaryAttributes {
    // ...
    // TODO: add support for rust_binary in Bazel
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BazelRustLibraryAttributes {
    // ...
    // TODO: add support for rust_library in Bazel
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BazelRustTestAttributes {
    // ...
    // TODO: add support for rust_test in Bazel
}

#[derive(Debug, Error)]
pub enum TargetError {
    #[error("Reindeer.toml not found at the given path: {0}")]
    ReindeerTomlNotFound(String),

    #[error("Cargo.toml not found alongside Reindeer.toml: {0}")]
    CargoTomlNotFound(String),

    #[error("Failed to parse Cargo.toml: {0}")]
    CargoTomlParseError(#[from] toml::de::Error),
}

/// The canonical name for a dependency in the workspace's registry (i.e. `third-party/rust/`) as referenced
/// by the deps attribute of a target `//third-party/rust:clap` where `clap` is the dependency name
/// which can be found in the `Cargo.toml` file located alongside the `reindeer.toml` file).
#[derive(
    Debug,
    Display,
    Clone,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Getters,
    MutGetters,
    Setters,
    TypedBuilder,
)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct CanonicalDependency {
    canonical_name: String,
}

// TODO: need to figure out how to do mapping for local dependencies (e.g. first-party crates
//       like `//crates/foo:bar` or `//crates/foo/bar:baz`)
impl Target {
    pub fn from_pruned_cargo_toml(
        used_dependencies: BTreeSet<Dependency>,
        reindeer: &Reindeer,
        root_cargo_toml: &PathBuf,
    ) -> Result<Vec<Target>> {
        // Get the workspace members from the root Cargo.toml file, if any
        // TODO:
        // let workspace_members = Self::get_workspace_members(root_cargo_toml)?;

        // Map the used dependencies to their corresponding paths in the local registry
        let local_registry_map = Self::create_local_registry_map(reindeer, root_cargo_toml)?;

        // Map the used dependencies to their corresponding canonical names in the local registry
        let canonical_dependencies_used =
            Self::map_used_dependencies_to_canonical_names(used_dependencies, local_registry_map);

        for canonical_dependency in canonical_dependencies_used {
            tracing::debug!("{}", canonical_dependency);
        }

        // Create a Target struct with the necessary information and used dependencies
        // TODO: here we need to come up with an algorithm for creating the targets
        //      based on the used dependencies and the local registry map
        //
        // We need to be able to support [[bin]] and [[lib]] targets in the Cargo.toml file
        // and generate associated targets in the BUCK/BUILD files
        // if those don't exist, then we fallback to scanning the source files for
        // whether or not they contain a `main.rs` or `lib.rs` file and generate the
        // appropriate target
        // ...
        todo!("Implement Target::from_cargo_toml_and_used_dependencies")
    }

    /// Creates a mapping from dependencies found in the local `Cargo.toml` file to their corresponding
    /// canonical names within the local registry (this include third-party dependencies
    /// generated and managed by `reindeer` as well as first-party dependencies declared in the
    /// workspace's `Cargo.toml` file if it exists).
    ///
    /// Reads the `Cargo.toml` file located in the same directory as the `reindeer.toml`
    /// file, extracts the dependencies, and maps them to their canonical names in the local registry for
    /// later use in generated `Target`s within BUCK/BUILD files (e.g. `//third-party/rust:clap`)
    ///
    /// # Errors
    ///
    /// Returns an error if the `reindeer.toml` or `Cargo.toml` file is not found, or if the `Cargo.toml`
    /// file cannot be parsed.
    fn create_local_registry_map(
        reindeer: &Reindeer,
        root_cargo_toml: &PathBuf,
        // TODO: add support for a workspace Cargo.toml file
        // ws_members: Option<&[PathBuf]>,
    ) -> Result<HashMap<Dependency, CanonicalDependency>> {
        // TODO: add support for a workspace Cargo.toml file
        let reindeer_toml_path = reindeer.path();
        let cargo_toml_path = reindeer
            .directory()
            .ok_or_else(|| {
                TargetError::ReindeerTomlNotFound("Reindeer directory not found".into())
            })?
            .join("Cargo.toml");

        // Sanity check, this should never happen unless the user manually
        // deletes the reindeer.toml file during execution
        if !reindeer_toml_path.exists() {
            tracing::error!(path = %reindeer_toml_path.display(), "Reindeer.toml not found");
            return Err(TargetError::ReindeerTomlNotFound(
                reindeer_toml_path.display().to_string().into(),
            )
            .into());
        }

        if !cargo_toml_path.exists() {
            tracing::error!(path = %cargo_toml_path.display(), "Cargo.toml not found");
            return Err(TargetError::CargoTomlNotFound(
                cargo_toml_path.display().to_string().into(),
            )
            .into());
        }

        // Returns a flat list of dependencies from a Cargo.toml file.
        // This is a superset of the dependencies required to pass `cargo check`.
        let cargo_toml_deps = extract_deps(&cargo_toml_path)?;
        let registry = reindeer.directory().ok_or_else(|| {
            TargetError::ReindeerTomlNotFound("Reindeer directory not found".into())
        })?;

        let mut local_registry_map = HashMap::new();
        for dep in cargo_toml_deps {
            local_registry_map.insert(
                dep.clone(),
                CanonicalDependency::builder()
                    .canonical_name(format!("//{}:{}", registry.display(), dep.name()).into())
                    .build(),
            );
            tracing::debug!(
                dependency = %dep.name(),
                canonical_name = %local_registry_map.get(&dep).unwrap().canonical_name(),
                "Mapped dependency found in Cargo.toml to local registry"
            );
        }

        tracing::debug!(
            "Materialized local registry with {} entries",
            local_registry_map.len()
        );

        Ok(local_registry_map)
    }

    fn map_used_dependencies_to_canonical_names(
        used_dependencies: BTreeSet<Dependency>,
        local_registry_map: HashMap<Dependency, CanonicalDependency>,
    ) -> BTreeSet<CanonicalDependency> {
        let mut used_dependencies_canonical_names = BTreeSet::new();

        for used_dependency in used_dependencies.iter() {
            if let Some(canonical_dependency) = local_registry_map.get(used_dependency) {
                tracing::debug!(
                    dependency = %used_dependency.name(),
                    canonical_name = %canonical_dependency.canonical_name(),
                    "Found canonical name for used dependency"
                );
                used_dependencies_canonical_names.insert(canonical_dependency.clone());
            } else {
                tracing::warn!(
                    dependency = %used_dependency.name(),
                    "Could not find canonical name for used dependency in "
                );
                tracing::warn!(
                    dependency = %used_dependency.name(),
                    "This dependency will not be included in the generated BUCK/BUILD files."
                );
                tracing::warn!(
                    dependency = %used_dependency.name(),
                    " This is likely an issue with aliasing a third-party dependency or a local workspace member.
                    Skipping for now. Please open an issue at
                    https://github.com/pulanski/elk/issues/new to help us improve."
                );
                thread::sleep(Duration::from_secs(3));
            }
        }

        tracing::debug!(
            "Mapped {} used dependencies to canonical names",
            used_dependencies_canonical_names.len()
        );

        used_dependencies_canonical_names
    }

    // fn get_workspace_members(root_cargo_toml: &PathBuf) -> Result<Vec<CanonicalizedDependency>> {
    //     // TODO: implement this
    // }
}
