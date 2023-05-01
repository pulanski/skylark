use crate::ast::AstNode;
pub use crate::lang::{SyntaxElement, SyntaxNode, SyntaxToken};
use crate::lexer::{FileId, Span, Token, TokenStream};
use crate::syntax_tree::SyntaxTreeBuilder;
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
