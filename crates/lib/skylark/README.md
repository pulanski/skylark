# Skylark, a Starlark implementation in Rust

Skylark is a Rust implementation of the Starlark language, a dialect of Python designed for use as a configuration language. It provides a comprehensive suite of tools and libraries for parsing, analyzing, and executing Starlark code, all built with performance and extensibility in mind.

# `sky_lexer`
The lexical analysis module is responsible for breaking down source code into tokens, the basic building blocks for the parsing process. This stage of processing reads the input as a stream of characters and groups them into meaningful units, such as keywords, identifiers, literals, operators, and punctuation marks. The lexer also handles comments and whitespace, discarding them as needed to streamline the subsequent parsing process. The lexer is designed with performance and error handling in mind, providing informative error messages and location information for easy debugging and integration with other tools.

# `sky_parser`
The parsing module converts a sequence of tokens into an Abstract Syntax Tree (AST), which serves as the foundation for all further analysis and manipulation of the source code. The AST is a hierarchical data structure that represents the syntactic structure of the source code, with nodes corresponding to different language constructs, such as expressions, statements, and control structures. The parser employs a recursive descent parsing strategy, which allows for clear and modular code structure while efficiently handling Starlark's grammar constructs. The resulting AST provides a rich API, making it extensible enough to build a wide variety of tools on top of it.

<!-- currently in development -->
<!-- # `sky_analyzer` # A tool to analyze Starlark code in an incremental fashion at build time. It performs various static analysis tasks, such as type checking, data flow analysis, and dependency analysis, ensuring code quality and maintainability. -->
<!-- # `sky_interpreter` # A tool to interpret Starlark code at runtime. It evaluates the AST, executing the code and providing a runtime environment for scripts, with support for built-in functions, user-defined functions, and native Rust functions. -->
<!-- # `buckaroo` # A tool for formatting Starlark code, performing static analysis, and linting. It serves as a comprehensive utility for managing code quality and style, similar to tools like `rustfmt`, `clippy`, `rust-analyzer`, `clang-format`, and `clang-tidy`. -->