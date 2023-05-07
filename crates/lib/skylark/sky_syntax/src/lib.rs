//! Syntax in [`skylark`], a rich API for working with Starlark files.
//!
//! This crate provides a generic library for working with syntax trees of
//! **Starlark** files (e.g. `BUILD`, `WORKSPACE`, `*.bzl`, `BUCK`, `*.star`)
//!
//! The overall architecture of **skylark's frontend** (e.g. the `skylark_parser`, and `skylark_syntax` crates) is
//! _heavily inspired_ by many of the techniques used in
//! [rust-analyzer](https://github.com/rust-lang/rust-analyzer/blob/master/docs/dev/syntax.md),
//! which was in turn inspired by the work done in Swift's [libsyntax](https://tree-sitter.github.io/tree-sitter/)
//! and C#'s [Roslyn](https://learn.microsoft.com/en-us/dotnet/csharp/roslyn-sdk/).
//!
//! If you're interested in learning more of the motivation behind the design, I highly recommend watching the
//! series, [Explaining rust-analyzer](https://www.youtube.com/playlist?list=PLhb66M_x9UmrqXhQuIpWC5VgTdrGxMx3y) on
//! YouTube, which is a great introduction to the concepts used in this crate.
//!
//! In general, syntax trees possess the following properties:
//! -   **Immutable**: Syntax trees are **immutable**, meaning that once a syntax tree is created,
//! it **cannot be modified**. This is a good thing, because it means that we can **share** them
//! **between threads** without worrying about synchronization.
//!
//! -   **Persistent**: Syntax trees are **persistent**, meaning that when a syntax tree is modified,
//! the original syntax tree is **not modified**. Instead, a **new syntax tree** is created which shares
//! as much data as possible with the original syntax tree. This allows us to **cache syntax trees** and
//! reuse them whenever possible.
//!
//! -   **Lossless**: Syntax trees are **lossless** or **full-fidelity**, meaning that the original source
//! code canbe **recovered** from a syntax tree. This allows us to use syntax trees for things like
//! refactoring and code generation, lending them useful for scenarios such as **code completion** in an
//! **IDE context** and **code formatting** in a **linter context**.
//!
//! -   **Error-resilient**: Syntax trees are **error-resilient**, meaning that they can be constructed
//! even for **invalid** source code. This allows us to **report** and **recover** from errors in a
//! **graceful** manner.

mod ast;
mod event;
mod grammar;
mod lang;
mod lexer;
mod logging;
mod parser;
mod parsing;
mod syntax_error;
mod syntax_tree;
mod text_token_source;
mod token_set;

pub use crate::{
    ast::File,
    ast::SyntaxKind,
    lang::{SyntaxNode, SyntaxToken},
    lexer::{StarlarkLexer, TokenKind, TokenSink},
    logging::init_logging,
    parser::StarlarkParser,
    parser::TextTreeSink,
    token_set::TokenSet,
};
use ast::AstNode;
use rowan::GreenNode;
use std::{marker::PhantomData, sync::Arc};
use syntax_error::SyntaxError;

// /// `Parse` is the result of the parsing: a syntax tree and a collection of errors.
// ///
// /// Note that we always produce a syntax tree, event for completely invalid files.
// #[derive(Debug, PartialEq, Eq)]
// pub struct Parse<T> {
//     green: GreenNode,
//     errors: Arc<[SyntaxError]>,
//     _ty: PhantomData<fn() -> T>,
// }

// impl<T> Clone for Parse<T> {
//     fn clone(&self) -> Parse<T> {
//         Parse {
//             green: self.green.clone(),
//             errors: self.errors.clone(),
//             _ty: PhantomData,
//         }
//     }
// }
// impl<T> Parse<T> {
//     fn new(green: GreenNode, errors: Vec<SyntaxError>) -> Parse<T> {
//         Parse {
//             green,
//             errors: Arc::from(errors),
//             _ty: PhantomData,
//         }
//     }
//     pub fn syntax_node(&self) -> SyntaxNode {
//         SyntaxNode::new_root(self.green.clone())
//     }
// }
// impl<T: AstNode> Parse<T> {
//     pub fn into_syntax(self) -> Parse<SyntaxNode> {
//         Parse {
//             green: self.green,
//             errors: self.errors,
//             _ty: PhantomData,
//         }
//     }
//     pub fn tree(&self) -> T {
//         T::cast(self.syntax_node()).unwrap()
//     }
//     pub fn errors(&self) -> &[SyntaxError] {
//         &self.errors
//     }
//     pub fn ok(self) -> Result<T, Arc<[SyntaxError]>> {
//         if self.errors.is_empty() {
//             Ok(self.tree())
//         } else {
//             Err(self.errors)
//         }
//     }
// }

/// The [`Parse`] represents the **result** of a **parsing operation** on a **piece of source code**.
/// It contains a _syntax tree_ (`green`), a _collection of syntax errors_ (`errors`), and a
/// _phantom type_ (`_ty`) to _associate_ the **parse result** with a **specific AST node**.
///
/// Regardless of whether the parsed source code is valid or not, a syntax tree will always
/// be produced. In cases where the source code is completely invalid, the tree will still be
/// generated, but it will be accompanied by a collection of syntax errors.
#[derive(Debug, PartialEq, Eq)]
pub struct Parse<T> {
    green: GreenNode,
    errors: Arc<[SyntaxError]>,
    _ty: PhantomData<fn() -> T>,
}

/// The `Clone` implementation for the `Parse` struct allows for creating a copy of the
/// parse result, preserving the syntax tree, errors, and associated AST node type.
impl<T> Clone for Parse<T> {
    fn clone(&self) -> Parse<T> {
        Parse {
            green: self.green.clone(),
            errors: self.errors.clone(),
            _ty: PhantomData,
        }
    }
}

impl<T> Parse<T> {
    /// Creates a new `Parse` instance with the given `GreenNode` and syntax errors.
    ///
    /// # Example
    ///
    /// ```
    /// use crate::parser::Parse;
    /// use crate::green::GreenNode;
    /// use crate::syntax_error::SyntaxError;
    ///
    /// let green_node = GreenNode::new(...);
    /// let errors = vec![SyntaxError::new(...)];
    /// let parse = Parse::<MyAstNode>::new(green_node, errors);
    /// ```
    fn new(green: GreenNode, errors: Vec<SyntaxError>) -> Parse<T> {
        Parse {
            green,
            errors: Arc::from(errors),
            _ty: PhantomData,
        }
    }

    /// Returns a `SyntaxNode` constructed from the `Parse` instance's `green` field.
    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }
}

impl<T: AstNode> Parse<T> {
    /// Converts the `Parse` instance into a `Parse<SyntaxNode>` instance, preserving the
    /// syntax tree and errors but discarding the specific AST node type.
    pub fn into_syntax(self) -> Parse<SyntaxNode> {
        Parse {
            green: self.green,
            errors: self.errors,
            _ty: PhantomData,
        }
    }

    /// Retrieves the `T` AST node from the `Parse` instance's syntax tree.
    pub fn tree(&self) -> T {
        T::cast(self.syntax_node()).unwrap()
    }

    /// Returns a slice of the syntax errors contained within the `Parse` instance.
    pub fn errors(&self) -> &[SyntaxError] {
        &self.errors
    }

    /// Consumes the `Parse` instance and returns a `Result` containing either the parsed
    /// AST node of type `T` or an `Arc<[SyntaxError]>` containing the syntax errors.
    ///
    /// If there are no errors, the `Ok` variant containing the AST node is returned. If there
    /// are errors, the `Err` variant containing the errors is returned.
    pub fn ok(self) -> Result<T, Arc<[SyntaxError]>> {
        if self.errors.is_empty() {
            Ok(self.tree())
        } else {
            Err(self.errors)
        }
    }
}
