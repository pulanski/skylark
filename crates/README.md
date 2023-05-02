# abstraction
High-level abstractions of data structures and traits for use across the entire project.

# apps
Applications for language users:
- hoof: The entry point for all language-related tasks and tools. This starts the hoof daemon server
which handles all requests from the user incrementally.
- analyzer: A tool to analyze code, ensuring correctness and adherence to best practices.
- compiler: Translates source code into an executable format for the target platform and architecture (e.g. **x86_64**, **arm64**, etc.).
- developer (debugger): A debugging tool to help users identify and fix issues in their code.
- skylark: A Starlark interpreter and linter, allowing users to run Starlark scripts and check for errors.

<!-- TODO: # benchmarks
Performance benchmarks for various aspects of the language and its tools to ensure efficiency and optimize improvements. -->

# comptime
Everything related to the compilation process, including optimization, code generation, and error handling.

<!-- TODO: # debugtime
Everything related to the debugging process, such as breakpoints, variable inspection, and step-by-step execution. -->

# devtools
Utility tools for language development, streamlining the development process and reducing boilerplate code.

<!-- TODO: # fs
Everything related to the file system, including file and directory manipulation, path handling, and file I/O. -->

# skylark
A Starlark interpreter and linter, allows users to run Starlark scripts and check for errors. All of the source-code-related tools operate on top of the same core syntax abstraction, which is defined in the `sky_syntax` module.

# ide
Integrated Development Environment (IDE) support, such as auto-completion, diagnostics, syntax highlighting, and other productivity features.

<!-- # infer
Everything related to inference, such as type inference, implicit argument inference, and constraint solving. -->

<!-- # kernel
A minimal set of axioms for the language, serving as the foundation for the entire language ecosystem. -->


# runtime
Runtime module that handles the execution of compiled programs, manages memory, and handles system interactions.

# syntax
Syntax module that defines the structure and rules of the language, enabling the parsing and generation of Abstract Syntax Trees (ASTs).

# try
Module containing example code and test cases, demonstrating language features and usage patterns.

# utils
Utility functions and helpers for use across the entire project, simplifying common tasks and improving code readability.

# vm
Virtual Machine (VM) module responsible for executing compiled code in a managed environment, providing isolation and portability.
