# Buck2 Support for Multi-Language Development

This repository supports both Cargo and Buck2 as underlying build systems for the project. While Cargo is the default build system for Rust projects, Buck2 offers several advantages that make it the preferred development method for this project:

1. **Faster build times:** Buck2 is known for its superior build times compared to Cargo in the context of Rust, making it a more efficient choice for the development process.

2. **Multi-language support:** As the project grows and incorporates tools written in other languages such as OCaml, Python, Go, etc., Buck2 provides the flexibility to manage builds across multiple languages using a single underlying build tool.

3. **Scalability:** Buck2 is designed to scale well with large codebases, making it a suitable choice for projects that are expected to grow over time.

4. **Incremental builds:** Buck2 supports incremental builds, which means that only the changes made to the codebase are rebuilt, leading to faster development cycles.

5. **Parallelism:** Buck2 is optimized for parallel builds, allowing it to better utilize available CPU resources and further speed up build times.

## Using Third-Party Crates with Buck2

One of the challenges in using Buck2 for Rust projects is managing third-party crates from crates.io, as the number of useful crates on crates.io is often one of the core reasons people love Rust so much, it's easy to use some high quality third-party code without having to vendor it all from source, as is common with C/C++ projects, although there is `conan`, which is actually fairly nice from my minimal experience using it. The project uses Rudolph, an extension of the tool Reindeer, to bring third-party Rust dependencies into the context of Buck2. This process involves:

1. **Vendoring third-party crates:** Rudolph vendors all of the third-party crates required by the project.

2. **Buckifying dependencies:** Rudolph then converts the vendored crates into a single large BUCK file, allowing them to be consumed as dependencies from any other target within the workspace.

This powerful process works seamlessly for most dependencies, although some may require additional maintenance or may not be supported yet.

### Benefits of Rudolph

Rudolph expands on Reindeer in several ways:

1. **Context-aware global caching strategy:** Rudolph uses a global caching strategy that speeds up multiple sequential executions of the command when the dependency set defined in a `Cargo.toml` file hasn't changed. This caching strategy is global, meaning it works across your entire system, regardless of which workspace you're working in, requiring no additional configuration.

2. **Embedded Reindeer executable:** Rudolph embeds Reindeer's executable, eliminating the need to download Reindeer separately, it just works out of the box.

## Setting Up Buck2 and Rudolph

To set up Buck2 as the build system for this project and incorporate Rudolph for managing third-party crates, follow the steps below:

### Setting Up Buck2

1. Install Buck2 by following the instructions provided in the [official Buck2 documentation](https://buck2.build/docs/getting_started/).

This is a one-time setup step. Once Buck2 is installed, it can be used to build any Buck2 project. It takes a while to build the first time, so don't worry if it takes a few minutes.

```bash
rustup install nightly-2023-03-07
cargo +nightly-2023-03-07 install --git https://github.com/facebook/buck2.git buck2
```

1. With `buck2` installed, we can test that everything is building and our tests are all passing:

   - To build the project, use `buck2 build //...`. This will build all the targets in the project.
   - To run the tests, use `buck2 test //...`. This will run all the tests in the project.

```bash
buck2 build //...
buck2 test //...
```

You'll notice that if you run either of these commands again, they will be much faster, nearly instant. This is because Buck2 caches the results of previous builds and only builds what is necessary.

> Please note that the project still maintains compatibility with Cargo for developers who prefer to use it. However, using Buck2 is recommended for the reasons mentioned above.

### Setting Up Rudolph

1. Install Rudolph by following the instructions provided in the [official Rudolph documentation](./rudolph.md).

2. Run `rudolph` to vendor all third-party crates and generate the BUCK file.

```bash
# by default, this will look for a Cargo.toml alongside
# a reeindeer.toml in the `third-party` directory
rudolph

# to specify a different directory, use the --third-party-dir flag
rudolph --third-party-dir /third-party/rust
```

3. If everything worked and rudolph exited succesfully, you should see an up-to-date `BUCK` file in the `third-party` directory. This file contains all the third-party crates as targets that can be consumed by any other target in the workspace and will only require re-running `rudolph` fully if the `Cargo.toml` or `reindeer.toml` files change, otherwise it will use the cached results.