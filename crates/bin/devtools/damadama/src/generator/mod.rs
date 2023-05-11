// generator/mod.rs

use crate::target::Target;
use anyhow::Result;

pub(crate) mod bazel;
pub(crate) mod buck;

/// A trait for implementing a generator for a specific build system (e.g., BUILD or BUCK).
///
/// This trait is implemented by the `BuckGenerator` and `BazelGenerator` structs
/// and could be extended to support other build systems in the future.
pub(crate) trait Generator {
    /// Generate the build file content for a single target. This is called for each target
    /// in a given crate.
    ///
    /// ## Example
    ///
    /// TODO: example of a rust cargo.toml with multiple binaries and tests and the corresponding
    /// buckified BUILD files
    fn generate_target(&self, target: &Target) -> Result<String>;

    fn generate_build_file(&self, targets: &[Target]) -> Result<String> {
        tracing::debug!(
            "Generating build file content for {} targets",
            targets.len()
        );
        let mut build_file_content = String::new();

        for target in targets {
            tracing::debug!("Generating build file content for {:?}", target);
            build_file_content.push_str(&self.generate_target(target)?);
            build_file_content.push('\n');
        }

        Ok(build_file_content)
    }
}
