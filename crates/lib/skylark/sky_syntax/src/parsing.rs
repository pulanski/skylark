mod text_token_source;
mod text_tree_sink;

use crate::{
    lexer::Token,
    parser::{self, ParseError},
    syntax_error::SyntaxError,
    StarlarkLexer, SyntaxKind,
};
use rowan::GreenNode;

pub(crate) fn parse_text(text: &str) -> (GreenNode, Vec<SyntaxError>) {
    // Tokenize the source into a token stream and a list of errors (i.e. unrecognized tokens)
    let mut lexer = StarlarkLexer::new();

    let token_sink = lexer.tokenize(text);
    let tokens = token_sink.tokens();

    if token_sink.has_errors() {
        lexer.emit_errors();
    }

    tracing::debug!("Tokens: {:#?}", tokens);

    let mut token_source = text_token_source::TextTokenSource::new(tokens.clone());
    tracing::debug!("Token Source: {:#?}", token_source);
    let mut tree_sink = text_tree_sink::TextTreeSink::new(tokens.clone());
    tracing::debug!("Tree Sink: {:#?}", tree_sink);

    parser::parse(&mut token_source, &mut tree_sink);

    tree_sink.finish()
}

/// The `TokenSource` trait provides an abstraction over the source of tokens, allowing for
/// seamless interaction with the token stream during parsing. This trait is responsible for
/// managing the current position in the token stream and providing various methods to access
/// tokens and perform lookahead operations.
///
/// Implementing this trait enables integration with parsers and syntax tree builders, such as those
/// found in the [`skylark`] and [`rowan`] crates.
///
/// [`skylark`]: https://crates.io/crates/skylark
/// [`rowan`]: https://crates.io/crates/rowan
pub trait TokenSource {
    /// Retrieves the current [`Token`] in the token stream. The current token is the one that
    /// the parser is presently processing.
    ///
    /// # Returns
    ///
    /// * A [`Token`] instance representing the current token.
    fn current(&self) -> Token;

    /// Performs a lookahead operation in the token stream, returning the `n`-th [`Token`] ahead
    /// of the current position. This method is useful for making parsing decisions based on
    /// upcoming tokens without consuming them.
    ///
    /// # Parameters
    ///
    /// * `n`: The number of positions ahead to look in the token stream.
    ///
    /// # Returns
    ///
    /// * A [`Token`] instance representing the `n`-th token ahead of the current position.
    fn lookahead_nth(&self, n: usize) -> Token;

    /// Advances the current position in the token stream to the next token. This method is called
    /// after the parser has finished processing the current token.
    fn bump(&mut self);

    /// Checks if the current token in the token stream matches the specified keyword.
    ///
    /// # Parameters
    ///
    /// * `kw`: A string slice representing the keyword to be checked against the current token.
    ///
    /// # Returns
    ///
    /// * `true` if the current token is the specified keyword, `false` otherwise.
    fn is_keyword(&self, kw: &str) -> bool;
}

/// The `TreeSink` trait provides an abstraction over the specifics of a syntax tree implementation,
/// allowing for the construction of syntax trees by consuming tokens and parse errors.
///
/// This trait is responsible for adding tokens and managing branches in a syntax tree during parsing.
/// It is inspired by the ["red-green"](https://ericlippert.com/2012/06/08/red-green-trees/) tree model
/// seen in the **Roslyn** compiler and integrates with the [`skylark`] and [`rowan`] crates.
///
/// [`skylark`]: https://crates.io/crates/skylark
/// [`rowan`]: https://crates.io/crates/rowan
pub trait TreeSink {
    /// Adds new tokens with the specified `kind` and `n_tokens` count to the current branch of the
    /// syntax tree. Tokens represent the smallest syntactic units in the source code.
    ///
    /// # Parameters
    ///
    /// * `kind`: The [`SyntaxKind`] of the tokens being added.
    /// * `n_tokens`: The number of tokens to be added with the specified `kind`.
    fn token(&mut self, kind: SyntaxKind, n_tokens: u8);

    /// Starts a new branch with the specified `kind` in the syntax tree and sets it as the current
    /// branch. This method is called when a new syntactic construct begins during parsing.
    ///
    /// # Parameters
    ///
    /// * `kind`: The [`SyntaxKind`] of the new branch being started.
    fn start_node(&mut self, kind: SyntaxKind);

    /// Finishes the current branch in the syntax tree and restores the previous branch as the
    /// current branch. This method is called when a syntactic construct ends during parsing.
    ///
    /// When the current branch is finished, it is attached to the previous branch as a child node.
    fn finish_node(&mut self);

    /// Attaches a parsing error to the current branch of the syntax tree.
    ///
    /// # Parameters
    ///
    /// * `error`: The [`ParseError`] to be attached to the current branch.
    fn error(&mut self, error: ParseError);
}
