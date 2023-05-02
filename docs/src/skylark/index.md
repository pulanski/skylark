# Skylark

Skylark is a Rust implementation of the Starlark language, a dialect of Python designed for use as a configuration language. It provides a comprehensive suite of tools and libraries for parsing, analyzing, and executing Starlark code, all built with performance and extensibility in mind.

The library is composed of several components:

- [`sky_lexer`](./sky_lexer.md) - Lexical analysis module
- [`sky_parser`](./sky_parser.md) - Parsing module
- [`sky_analyzer`](./sky_analyzer.md) - Static analysis tool (currently in development)
- [`sky_interpreter`](./sky_interpreter.md) - Runtime interpreter (currently in development)
- [Buckaroo](../devtools/buckaroo.md) - Code formatting, static analysis, and linting tool (currently in development)

Each component is designed to work independently, but they can also be combined to form a powerful and flexible pipeline for processing Starlark code.
