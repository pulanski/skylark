# `devtools`

`devtools` is a comprehensive suite of utility tools aimed at simplifying the development of programming languages and build systems. By streamlining the process, reducing boilerplate code, and efficiently handling syntax trees and other language-related features, `devtools` makes it easier than ever to develop a programming language and its build system in a generic and extensible way.

This document provides a brief overview of the tools included in the devtools package:

## Table of Contents

- [`devtools`](#devtools)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Tools](#tools)
    - [SyntaxGen, `syntaxgen`](#syntaxgen-syntaxgen)
    - [Rudolph, `rudolph`](#rudolph-rudolph)

## Introduction

Developing a programming language and its build system can be a **complex** and **time-consuming task**, `devtools`, is our solution to this challenge, providing a suite of utility tools to make the process as seamless as possible. Our primary focus is on generating rich API functionality for syntax trees, which are the backbone of any programming language.

## Tools

### SyntaxGen, `syntaxgen`

**SyntaxGen** is a powerful utility tool designed to generate a rich API for syntax trees, inspired heavily by the source-code generation implemented within `rust-analyzer`. By automatically creating functionality based on the structure of the syntax tree, SyntaxGen saves time and minimizes the chances of human error in the development process.

Key features of SyntaxGen include:

- Automatic generation of API functionality for syntax trees
- Support for custom language syntax and constructs
  - Currently supports:
    - Starlark (`.bzl`, `.star`, `BUILD`, `WORKSPACE`, `BUCK`, etc.)
  - Planned support:
    - hoof (`.hf`)
- Simplification of code generation and manipulation tasks
- Extensibility for adding new features and enhancements
- Comprehensive documentation and examples

For more details and usage instructions, please refer to the [SyntaxGen documentation](./devtools/syntaxgen.md).

### Rudolph, `rudolph`

**Rudolph** is a tool designed to _bridge the gap_ between **Cargo** and **non-Cargo** build systems, such as **Buck2** (and potentially Bazel), by importing and managing third-party crates.

> **Note:** Rudolph is a fairly simple tool at the moment really only designed to support my current needs, the majority of the dependency resolution logic is handled by reindeer. In the future, Rudolph will be extended to support more complex use cases and scenarios.

Rudolph streamlines the process of integrating and managing dependencies, making it easier to work with Rust projects in different build environments.

Key features of Rudolph include:

- Importing and managing third-party crates from various sources such as crates.io, GitHub, and local repositories
- Automatic generation of build rules for imported crates
- Support for custom fixups and patches to handle edge cases in third-party crates
- Integration with Buck build system for seamless development experience
- `rudolph init` command to scaffold and configure new Rust projects with popular third-party dependencies
- Support for devcontainers to simplify onboarding and development environment setup
- Extensibility for adding new features and enhancements
- Comprehensive documentation and examples

For more details and usage instructions, please refer to the [Rudolph documentation](./devtools/rudolph.md).