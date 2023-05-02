use rowan::{GreenNodeBuilder, Language, TextRange, TextSize};

use crate::{
    lang::Starlark, lexer::Token, parser::ParseError, syntax_error::SyntaxError, SyntaxKind,
    SyntaxNode,
};

#[derive(Debug, Default)]
pub struct SyntaxTreeBuilder {
    builder: GreenNodeBuilder<'static>,
    errors: Vec<SyntaxError>,
}

impl SyntaxTreeBuilder {
    /// Creates a new **syntax tree builder**.
    pub fn new() -> Self {
        Self {
            builder: GreenNodeBuilder::new(),
            errors: Vec::new(),
        }
    }

    /// Starts a new node in the syntax tree.
    pub fn start_node(&mut self, kind: SyntaxKind) {
        self.builder.start_node(Starlark::kind_to_raw(kind))
    }

    /// Finishes the syntax tree and returns the root node.
    pub fn finish(mut self) -> SyntaxNode {
        let root = self.builder.finish();
        SyntaxNode::new_root(root)
    }

    /// Adds a raw token to the current node.
    pub fn add_raw_token(&mut self, kind: SyntaxKind, text: &str) {
        self.builder.token(Starlark::kind_to_raw(kind), text);
    }

    /// Adds a token created from the Lexer to the current node.
    pub fn add_token(&mut self, token: &Token) {
        self.builder.token(
            Starlark::kind_to_raw(SyntaxKind::from(*token.kind())),
            token.lexeme(),
        )
    }

    /// Finishes the current node.
    pub fn finish_node(&mut self) {
        self.builder.finish_node()
    }

    pub fn error(&mut self, error: ParseError, range: TextRange) {
        self.errors.push(SyntaxError::new(
            "Syntax error encountered".to_owned(),
            range,
        ));
    }
}
