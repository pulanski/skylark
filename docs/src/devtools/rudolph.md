# Rudolph: Managing Third-Party Rust Dependencies with Buck2

Rudolph is a powerful tool for managing third-party Rust dependencies in the context of Buck2. By extending Reindeer, Rudolph offers streamlined management of third-party crates, global caching, and an embedded Reindeer executable.

## Key Features

- **Context-aware global caching strategy**: Rudolph uses a global caching strategy that speeds up sequential "buckifications" as much as possible. Only needs to perform work when the dependency set defined in a `Cargo.toml` of a Buck2 workspace has changed. This works for all Buck2 workspaces on your system, requiring no additional configuration.
- **Embedded Reindeer executable**: Rudolph embeds Reindeer's executable, eliminating the need to download Reindeer separately.
- **Build correctness**: Rudolph ensures the correctness of builds by running a Buck2 build when a cached buckification of the dependency graph is not found. This means incurring the extra build cost for the new dependencies up front, and then never needing it again until the next change, taking advantage of incremental builds.

## Core Algorithm

Rudolph's core algorithm is as follows:

1. **Check cache**: Rudolph checks if a cached version of the third-party dependencies is available and up-to-date. If a cached version is found, Rudolph returns early to avoid unnecessary work.

2. **Embed Reindeer**: Rudolph lazily embeds the Reindeer binary if it hasn't already been embedded.

3. **Vendor third-party crates**: Rudolph uses the embedded Reindeer binary to vendor third-party Rust crates into the third-party directory.

4. **Buckify third-party crates**: Rudolph uses the embedded Reindeer binary to buckify third-party Rust crates into the third-party directory.

5. **Build workspace**: Rudolph builds all targets in the workspace using Buck2 to ensure that the newly buckified third-party crates are available and that the workspace is in a consistent state.

## Using Rudolph

To use Rudolph in your Rust project, follow these steps:

1. Install Rudolph by following the instructions below:

    ```
    cargo +nightly-2023-03-07 install --git https://github.com/pulanski/rudolph.git
    ```

2. Ensure that your `Cargo.toml` file contains the necessary third-party dependencies.

3. Run Rudolph in your project's root directory. By default, Rudolph will perform the following actions:

   - Check if a cached version of the third-party dependencies is available and up-to-date.
   - If not, it will embed the Reindeer binary, vendor third-party Rust crates, and buckify them.
   - Finally, it will build all targets in the workspace using Buck2.

4. If you need to force a re-vendoring and re-buckifying of third-party Rust crates, you can re-run Rudolph with the `--force` flag.

With Rudolph, managing third-party Rust dependencies in the context of Buck2 is seamless and efficient, leveraging the benefits of incremental builds and a global caching strategy.

<!-- ## Features -->

<!-- ### Easy Onboarding with Devcontainers

In addition to setting up a Buck2-based Rust project with third-party dependencies, the Rudolph init process also takes care of initializing everything needed for setting up Devcontainers as a development environment. This greatly simplifies the onboarding process for new developers or anyone who wants to explore the code.

By utilizing Devcontainers, developers can start working on the project without needing to install Rust or Buck2 on their local machines. All that's required is Docker, as the development environment will be containerized, ensuring a consistent and easily shareable setup across different machines and developers.

Once the Devcontainer is set up, developers can build and run the project within the containerized environment, making it an ideal solution for seamless collaboration and easy onboarding. -->