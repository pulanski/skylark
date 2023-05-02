# Continuous Integration

The Skylark project follows a continuous integration (CI) process that closely aligns with the resources provided in [jonhoo/rust-ci-conf](https://github.com/jonhoo/rust-ci-conf). This repository contains a collection of general-purpose GitHub workflows that are useful in Rust-based projects.

## CI for Cargo

Currently, the project only supports CI for the Cargo build system. CI for Bazel may be added in the future, and CI for Buck2 is not ready at the moment.

### Workflow Overview

The CI workflow for the Skylark project consists of the following main steps:

1. **Building**: The project is built using the stable, beta, and nightly Rust channels to ensure compatibility with different Rust versions.
2. **Testing**: Unit tests and integration tests are run to ensure the correctness of the codebase.
3. **Linting**: The code is checked for adherence to Rust style guidelines using `clippy`.
4. **Formatting**: The code is checked for consistent formatting using `rustfmt`.
5. **Security auditing**: Dependencies are audited for known security vulnerabilities using `cargo-audit`.

### Workflow Configuration

The GitHub workflow configuration file, `.github/workflows/ci.yml`, sets up the CI pipeline. It is based on the templates provided in the `jonhoo/rust-ci-conf` repository and has been customized to fit the specific needs of the project.

### Integration with GitHub

The CI pipeline is integrated with GitHub, which automatically runs the pipeline whenever a new commit is pushed or a pull request is opened. The results of the CI pipeline can be viewed in the "Actions" tab of the GitHub repository.

This integration helps maintain the code quality and ensures that new changes do not introduce regressions or break existing functionality.

## Future CI Enhancements

As the Skylark project evolves, there may be opportunities to expand and improve the CI process. This could include support for additional build systems like Bazel, more comprehensive testing, and additional static analysis tools (i.e. `address-sanitizer` and `thread-sanitizer`).
