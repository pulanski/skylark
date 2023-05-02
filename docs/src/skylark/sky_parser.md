# `sky_parser`

The parsing module converts a sequence of tokens into an Abstract Syntax Tree (AST), which serves as the foundation for all further analysis and manipulation of the source code. The AST is a hierarchical data structure that represents the syntactic structure of the source code, with nodes corresponding to different language constructs, such as expressions, statements, and declarations.

The parser is designed to handle errors gracefully, providing informative error messages and location information to aid in debugging and integration with other tools.

The AST nodes include a rich API that is extensible enough to build a wide variety of tools on top of it, such as static analyzers, interpreters, code generators, and more.

For more details on how the parser works and how to use it in your project, refer to the API documentation, it's fairly expansive.
