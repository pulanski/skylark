# devtools

Contains a collection of utility tools designed to streamline the development of the language and build system. These tools are aimed at simplifying the development process, reducing boilerplate code, and ensuring efficient handling of syntax trees and other language-related features.

## Table of Contents

- [devtools](#devtools)
  - [Table of Contents](#table-of-contents)
  - [Introduction](#introduction)
  - [Tools](#tools)
    - [SyntaxGen](#syntaxgen)

## Introduction

Developing a programming language and its build system can be a complex and time-consuming task, `devtools`, is our solution to this challenge, providing a suite of utility tools to make the process as seamless as possible. Our primary focus is on generating rich API functionality for syntax trees, which are the backbone of any programming language.

## Tools

### SyntaxGen

**SyntaxGen** is a powerful utility tool designed to generate a rich API for syntax trees, inspired heavily
by the source-code generation implemented within `rust-analyzer`. By automatically creating functionality based on the structure of the syntax tree, SyntaxGen saves time and minimizes the chances of human error in the development process.

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

For more details and usage instructions, please refer to the [SyntaxGen documentation](./syntaxgen/README.md).
