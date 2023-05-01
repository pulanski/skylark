use std::cell::Cell;

use crate::ast::AstNode;
use crate::event::Event;
pub use crate::lang::{SyntaxElement, SyntaxNode, SyntaxToken};
use crate::lexer::{FileId, Span, Token, TokenStream};
use crate::parsing::{TokenSource, TreeSink};
use crate::syntax_tree::SyntaxTreeBuilder;
use crate::token_set::TokenSet;
use crate::T;
use crate::{ast::SyntaxKind, lexer::TokenKind};
use anyhow::Result;
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use codespan_reporting::term::{self, Config};
use getset::{Getters, MutGetters, Setters};
use owo_colors::OwoColorize;

#[derive(Debug, Getters, MutGetters, Setters, TypedBuilder)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct StarlarkParser {
    tokens: TokenStream,
    current: Option<Token>,
    syntax_builder: Option<SyntaxTreeBuilder>,
    tree_sink: TextTreeSink,
    files: SimpleFiles<String, String>,
    file_ids: Vec<FileId>,
    syntax_errors: Vec<Diagnostic<FileId>>,
}

impl StarlarkParser {
    /// Creates a new Parser with the given token stream.
    pub fn new(tokens: TokenStream) -> Self {
        let mut files = SimpleFiles::new();

        files.add(
            tokens.file_name().to_string_lossy().to_string(),
            tokens.source().to_string(),
        );

        Self {
            tokens: tokens.clone(),
            current: None,
            syntax_builder: Some(SyntaxTreeBuilder::new()),
            file_ids: vec![tokens.file_id()],
            files,
            syntax_errors: vec![],
            tree_sink: TextTreeSink::new(),
        }
    }

    // OLD API
    /// Parses the token stream and returns the root SyntaxNode.
    // pub fn parse(&mut self) -> Result<SyntaxNode> {
    //     self.advance()?;
    //     let builder = self
    //         .syntax_builder
    //         .as_mut()
    //         .expect("Syntax builder should be present before parsing");
    //     builder.start_node(SyntaxKind::FILE);

    //     while let Some(token) = &self.current {
    //         self.syntax_builder.as_mut().unwrap().add_token(token);
    //         self.advance()?;
    //     }
    //     self.syntax_builder.as_mut().unwrap().finish_node();

    //     let syntax = self.syntax_builder.take().unwrap().finish();
    //     Ok(syntax)
    // }

    /// Advances the token stream to the next token.
    fn advance(&mut self) -> Result<()> {
        self.current = self.tokens.next();
        Ok(())
    }

    // TODO: move api to this
    // /// Advances the token stream to the next token.
    // fn advance(&mut self) -> Option<Token> {
    //     self.current = self.tokens.next();

    //     &self.current
    // }

    fn parse_file(&mut self) -> Result<()> {
        let builder = self
            .syntax_builder
            .as_mut()
            .expect("Syntax builder should be present before parsing");
        builder.start_node(SyntaxKind::FILE);

        while let Some(token) = &self.current {
            self.try_parse(|parser| parser.parse_statement());
        }

        self.syntax_builder.as_mut().expect("Syntax builder should be present after parsing. If this error occurs, please file a bug report. This is a fatal error.").finish_node();

        // get the File AST
        let syntax = self.syntax_builder.take().expect("Syntax builder should be present after parsing. If this error occurs, please file a bug report. This is a fatal error.").finish();

        // get the file AST Node
        let file_node = syntax.first_child().expect("File should have a first child. If this error occurs, please file a bug report. This is a fatal error.");
        Ok(())
    }

    fn try_parse<F: FnOnce(&mut StarlarkParser) -> Result<(), ParseError>>(
        &mut self,
        parse_func: F,
    ) {
        let result = parse_func(self);
        let Err(error) = result else { return };
        self.emit_error(error);
        self.recover();
    }

    fn recover(&mut self) {
        while let Some(token) = &self.current {
            match token.kind() {
                // You can add other token kinds that you think should be recovery points
                TokenKind::DEF_KW | TokenKind::IF_KW | TokenKind::FOR_KW | TokenKind::EOF => {
                    return;
                }
                _ => self.advance().expect("Unreachable: current token should be present. If this error occurs, please file a bug report. This means we've jumped off the edge without reaching an EOF."),
            }
        }
    }

    fn parse_statement(&mut self) -> Result<(), ParseError> {
        match self.current.as_ref().map(|token| token.kind()) {
            Some(TokenKind::DEF_KW) => self.parse_def_stmt(),
            Some(TokenKind::IF_KW) => self.parse_if_stmt(),
            Some(TokenKind::FOR_KW) => self.parse_for_stmt(),
            // ...
            _ => {
                let found = self
                    .current
                    .as_ref()
                    .unwrap_or(&Token::new(TokenKind::EOF, "".to_owned(), Span::new(0, 0)))
                    .clone();
                Err(ParseError::UnexpectedToken {
                    expected: vec![
                        TokenKind::DEF_KW,
                        TokenKind::IF_KW,
                        TokenKind::FOR_KW,
                        // ...
                    ],
                    found,
                })
            }
        }
    }

    fn parse_def_stmt(&self) -> Result<(), ParseError> {
        todo!("Parse def() statements")
    }

    fn parse_if_stmt(&self) -> Result<(), ParseError> {
        todo!("Parse if statements")
    }

    fn parse_for_stmt(&self) -> Result<(), ParseError> {
        todo!("Parse for loops")
    }

    fn emit_error(&mut self, error: ParseError) {
        let diagnostic = error.to_diagnostic(self.file_ids[0]);

        self.syntax_errors.push(diagnostic);
    }

    fn num_syntax_errors(&self) -> usize {
        self.syntax_errors.len()
    }

    pub(crate) fn emit_errors(&self) -> Result<()> {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();

        tracing::debug!("Emitting {} syntax errors", self.num_syntax_errors());

        for error in self.tree_sink.syntax_errors() {
            term::emit(&mut writer, &config, self.files(), error)?;
        }

        tracing::info!(
            "Found {} syntax errors in {:?} files",
            self.num_syntax_errors(),
            self.files,
        );

        Ok(())
    }

    fn build_syntax_tree(&mut self) -> Result<SyntaxNode> {
        self.advance()?;
        let builder = self
            .syntax_builder
            .as_mut()
            .expect("Syntax builder should be present before parsing");
        builder.start_node(SyntaxKind::FILE);

        while let Some(token) = &self.current {
            self.syntax_builder.as_mut().unwrap().add_token(token);
            self.advance()?;
        }
        self.syntax_builder.as_mut().unwrap().finish_node();

        let syntax = self.syntax_builder.take().unwrap().finish();
        Ok(syntax)
    }

    pub(crate) fn parse(&self) -> Result<TextTreeSink> {
        tracing::debug!("Parsing files...");

        // let source_file = self.parse

        tracing::debug!("Finished parsing files");
        tracing::debug!("Collecting syntax errors...");

        // let tree_sink = TreeSink::new(Box::new(syntax_tree));

        todo!("Construct File AST from syntax tree and collect syntax errors");

        // Ok(tree_sink)
    }
}

use codespan_reporting::diagnostic::{Diagnostic, Label};
use typed_builder::TypedBuilder;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseError {
    UnexpectedToken {
        expected: Vec<TokenKind>,
        found: Token,
    },
    UnexpectedEof,
}

impl ParseError {
    fn to_diagnostic(&self, file_id: usize) -> Diagnostic<usize> {
        match self {
            ParseError::UnexpectedToken { expected, found } => {
                let message = format!(
                    "Unexpected token{}{} {:?}. Expected {:?}, but found {:?} instead.",
                    ":".black(),
                    if expected.len() > 1 { "s" } else { "" },
                    found.kind(),
                    expected,
                    found.kind()
                );
                let label = Label::primary(file_id, found.range()).with_message(message);

                Diagnostic::error()
                    .with_message("Syntax error: Unexpected token")
                    .with_labels(vec![label])
                    .with_notes(vec![format!(
                        "Expected one of the following token kinds: {:?}",
                        expected
                    )])
            }
            ParseError::UnexpectedEof => Diagnostic::error()
                .with_message("Syntax error: Unexpected end of file")
                .with_labels(vec![Label::primary(file_id, usize::MAX..usize::MAX)
                    .with_message("Unexpected end of file")]),
        }
    }
}

fn print_diagnostics(source: &str, file_id: usize, diagnostics: &[Diagnostic<usize>]) {
    let mut files = SimpleFiles::new();
    files.add("source", source);

    let writer = StandardStream::stderr(ColorChoice::Auto);
    let config = Config::default();

    for diagnostic in diagnostics {
        let _ = term::emit(&mut writer.lock(), &config, &files, diagnostic);
    }
}

#[derive(Debug, Clone, Getters, Setters, MutGetters)]
#[getset(get = "pub", set = "pub", get_mut = "pub")]
pub struct TextTreeSink {
    pub syntax_tree: Vec<Box<dyn AstNode>>,
    pub syntax_errors: Vec<Diagnostic<FileId>>,
}

impl TextTreeSink {
    pub fn new() -> Self {
        Self {
            syntax_tree: Vec::new(),
            syntax_errors: Vec::new(),
        }
    }
}

// ----

/// The [`Parser`] and its methods are responsible for processing and parsing tokens
/// from a given input. The parser **does not** actually parse things itself but **provides
/// an interface** for _advancing through the input_, _inspecting tokens_, and _constructing the syntax tree_.
///
/// The [`Parser`] provides a low-level **API** for navigating through a stream of [`Token`]s.
/// and constructing the parse tree. The [`Parser`] is **not** responsible for parsing the
/// input itself, that is handled by the [`grammar`] module.
///
/// The result from the parser is **not** a concrete syntax tree, but instead
/// a stream of [`Event`]s that can be used to construct a concrete syntax tree. The events
/// are of the form `start_expr`, `consume_token`, `finish_expr`, etc. See [`Event`] for more
/// information.
pub struct Parser<'t> {
    token_source: &'t mut dyn TokenSource,
    events: Vec<Event>,
    steps: Cell<u32>,
}

impl<'t> Parser<'t> {
    /// Creates a new [`Parser`] with the given [`TokenSource`].
    ///
    /// # Example
    ///
    /// ```
    /// use crate::parser::Parser;
    /// use crate::lexer::Lexer;
    ///
    /// let mut token_stream = TokenStream::from("x = 5");
    /// let mut parser = Parser::new(&mut token_stream);
    /// ```
    pub fn new(token_source: &'t mut dyn TokenSource) -> Self {
        Self {
            token_source,
            events: Vec::new(),
            steps: Cell::new(0),
        }
    }

    /// Returns the [`SyntaxKind`] of the current token or **EOF** if the parser
    /// has reached the **end** of the input.
    pub(crate) fn current(&self) -> SyntaxKind {
        self.nth(0)
    }

    /// Performs a **lookahead operation** and returns the [`SyntaxKind`]
    /// of the token `n` steps ahead of the current token that the
    /// parser is currently processing (looking at).
    ///
    /// # Panics
    ///
    /// Panics if `n` is greater than `3`. We don't want to lookahead more than `3` tokens.
    /// This is a **hard limit**.
    ///
    /// Additionally, if the parser has performed more than `10 million` steps, we assume
    /// that the parser is stuck in an **infinite loop** and **panic**. This is an **bug** with the
    /// parser itself and would need to be fixed.
    pub fn nth(&self, n: usize) -> SyntaxKind {
        assert!(n <= 3, "Cannot lookahead more than 3 tokens");
        let steps = self.steps.get();
        assert!(steps <= 10_000_000, "Infinite loop detected within the parser. Aborting. This is a bug. Please report it at https://github.com/pulanski/skylark/issues/new");
        self.steps.set(steps + 1);

        self.token_source.lookahead_nth(n).kind().to_syntax()
    }

    pub fn finish(self) -> Vec<Event> {
        self.events
    }

    /// Checks if the current token is `kind`.
    ///
    /// This is a convenience method for performing `self.nth_at(0, kind)`
    /// in a more ergonomic fashion.
    pub(crate) fn at(&self, kind: SyntaxKind) -> bool {
        self.nth_at(0, kind)
    }

    /// Checks if the token `n` steps ahead of the current token is `kind`.
    pub(crate) fn nth_at(&self, n: usize, kind: SyntaxKind) -> bool {
        self.token_source.lookahead_nth(n).kind().to_syntax() == kind
    }

    /// Checks if the current token is in contained within the
    /// given [`TokenSet`], `kinds`.
    pub(crate) fn at_ts(&self, kinds: TokenSet) -> bool {
        kinds.contains(self.current())
    }

    // We don't have to worry about the `at_*` methods being called since we aren't
    // gluing tokens together. The tokens are already glued together by the lexer.
    // fn at_composite2(&self, n: usize, k1: SyntaxKind, k2: SyntaxKind) -> bool {
    //     let t1 = self.token_source.lookahead_nth(n);
    //     let t2 = self.token_source.lookahead_nth(n + 1);
    //     t1.kind().to_syntax() == k1 && t1.is_jointed_to_next && t2.kind().to_syntax() == k2
    // }
    // fn at_composite3(&self, n: usize, k1: SyntaxKind, k2: SyntaxKind, k3: SyntaxKind) -> bool {
    //     let t1 = self.token_source.lookahead_nth(n);
    //     let t2 = self.token_source.lookahead_nth(n + 1);
    //     let t3 = self.token_source.lookahead_nth(n + 2);
    //     (t1.kind == k1 && t1.is_jointed_to_next)
    //         && (t2.kind == k2 && t2.is_jointed_to_next)
    //         && t3.kind == k3
    // }

    /// Checks if the current token is contextual keyword with text `t`.
    pub(crate) fn at_contextual_kw(&self, kw: &str) -> bool {
        self.token_source.is_keyword(kw)
    }
}

pub fn parse(token_source: &mut dyn TokenSource, tree_sink: &mut dyn TreeSink) {
    todo!("Parse source code into AST")
}
