# SyntaxGen

SyntaxGen is a utility tool designed to generate rich API functionality for syntax trees in the Starlark language. It takes the grammar definition in the ungrammar format, processes it, and generates the necessary code for `SyntaxKind`, `SyntaxNode`, and `SyntaxToken` data structures, which are used to represent the syntax trees of the language.

## Features

- Generates syntax kinds, tokens, and nodes based on the grammar definition
- Ensures up-to-date code generation for syntax-related data structures
- Automatically adds a preamble to generated files with information about the source and a warning not to edit the file by hand
- Logs events and errors during the code generation process

## Usage

To use SyntaxGen, you should have the Starlark grammar definition file `starlark.ungram` in the `grammar` directory. Then, run the SyntaxGen tool with the following command:

```bash
cargo run
```

SyntaxGen will parse the grammar definition, lower it to an AST, and generate the necessary code for the syntax kinds, tokens, and nodes. It will then ensure that the generated files are up-to-date and reformat them using `rustfmt`.

## Code Overview

The main entry point for SyntaxGen is the `main` function, which performs the following steps:

1. Initializes logging for debugging purposes (optional)
2. Parses the grammar definition from `starlark.ungram`
3. Lowers the grammar to an AST
4. Generates the syntax kinds, tokens, and nodes for the Starlark language
5. Ensures the generated files are up-to-date and reformats them using `rustfmt`

The `add_preamble` function is used to add a preamble to the generated files, which includes a warning not to edit the file by hand and a link to the source of the file.

The `ensure_file_contents` function checks whether the contents of the generated files are up-to-date. If they are not, it updates the files and logs a warning.

The `reformat` function reformats the generated code using `rustfmt` with the `fn_single_line` configuration option.

The `init_logging` function initializes logging with the `tracing_subscriber` crate, allowing for debugging and event logging during the code generation process.