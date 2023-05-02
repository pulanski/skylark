// TODO: refactor from syntax into lexer
// currently all the logic is housed in the syntax crate

//! `StarlarkLexer` is a lexer for the Starlark language.
//!
//! Starlark is a dialect of Python that is designed for use in configuration files and build systems.
//! It is used by the Bazel build tool and is designed to be easy to read, fast, and deterministic.
//! StarlarkLexer is responsible for tokenizing Starlark source code into a sequence of tokens
//! which can then be used by a parser to construct an abstract syntax tree (AST) for further processing.
//!
//! # Examples
//!
//! Basic usage:
//!
//! ```rust,ignore
//! use sky_lexer::StarlarkLexer;
//!
//! let source_code = "load(\"@bazel_tools//tools/build_defs/repo:git.bzl\", \"git_repository\")";
//!
//! let source_file = Pathbuf::from("example.bzl")?;
//! let mut lexer = StarlarkLexer::new();
//! let file_id = lexer.add_source(source_code.to_string()); // Add raw source code text to the lexer. (useful for repl-like environments, or just one-off processing)
//! let file_id = lexer.add_file(source_file); // Add a file to the lexer. (useful for batch processing)
//! let token_sink = lexer.lex(file_id);
//!
//! // Iterate over the tokens and print them.
//! // Tokens have a lot of associated metadata packed into them, including the token kind,
//! // the token text (lexeme), and the token location (span).
//! for token in token_sink.tokens() {
//!     println!("{}", token);
//! }
//!
//! // Emit rustc-style error messages for unknown tokens.
//! if token_sink.has_errors() {
//!    token_sink.emit_errors();
//! }
//! ```
//!
//! # Features
//!
//! - Tokenizes Starlark source code into a sequence of tokens
//! - Handles tokenizing multiple files and assigns unique FileIds to each file
//! - Allows for easy retrieval of tokens and their associated metadata
//! - Provides detailed error reporting for unknown tokens encountered during lexing
//!
//! - Using Starlark lexer you get a sequence of tokens from which you can build off from,
//! as well as detailed rustc-styleerror reporting for unknown tokens for free, just
//! call `emit_errors` on the `TokenSink` returned by `lex`.
//!
//! # Limitations
//!
//! - TokenKind::UNKNOWN represents unknown tokens, and error handling can be improved
//! - Tokenization performance can be further optimized, right now we are storing
//! a lot of metadata in each token, which is not necessary for all use cases
//! - Tokenization is not incremental, and is done in one pass
//! - Tokenization is not configurable, and is done according to the Starlark language spec
//!
//! # Error Reporting
//!
//! In case an unknown token is encountered during lexing, the lexer emits detailed error messages
//! with information about the invalid token and its location in the source code. These error messages
//! can be emitted to stderr using the `emit_errors` method.
//!
//! ```
//! use starlark_lex::StarlarkLexer;
//!
//! let source_code = "load(\"@bazel_tools//tools/build_defs/repo:git.bzl\", \"git_repository\")";
//! let mut lexer = StarlarkLexer::new();
//! let file_id = lexer.add_file(source_code.to_string()).unwrap();
//! let token_sink = lexer.lex(file_id).unwrap();
//!
//! // Emit any errors found during lexing.
//! lexer.emit_errors().unwrap();
//! ```
//!
//! For more information on the Starlark language and its grammar, please refer to the
//! [Starlark Language Specification](https://github.com/bazelbuild/starlark/blob/master/spec.md).
