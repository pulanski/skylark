#![allow(unused)]

use crate::{syntax_error::SyntaxError, SyntaxKind};
use anyhow::{anyhow, Result};
use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::{SimpleFile, SimpleFiles},
    term::{
        self,
        termcolor::{ColorChoice, StandardStream},
    },
};
use derive_more::Display;
use getset::{Getters, MutGetters, Setters};
use itertools::Itertools;
use lazy_static::lazy_static;
use logos::Logos;
use owo_colors::OwoColorize;
use std::{
    fmt::{self, write, Debug, Display},
    fs::{self, create_dir_all, File, OpenOptions},
    io::{Read, Write},
    ops::Range,
    path::{Path, PathBuf},
};
use typed_builder::TypedBuilder;

lazy_static! {
    pub static ref STDIN_PATH: PathBuf = dirs_next::cache_dir()
        .expect("Unable to find cache dir")
        .join("sky_lexer")
        .join("STDIN");
}

impl AsRef<Path> for STDIN_PATH {
    fn as_ref(&self) -> &Path {
        self
    }
}

impl Debug for STDIN_PATH {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_lossy())
    }
}

pub type FileId = usize;

#[derive(Debug, Display, Default, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
#[display(fmt = "{start}..{end}")]
pub struct Span {
    start: usize,
    end: usize,
}

use rowan::{TextRange, TextSize};

impl From<Span> for TextRange {
    fn from(val: Span) -> Self {
        TextRange::new(
            TextSize::from(val.start as u32),
            TextSize::from(val.end as u32),
        )
    }
}

impl From<Range<usize>> for Span {
    fn from(range: Range<usize>) -> Self {
        Self {
            start: range.start,
            end: range.end,
        }
    }
}

impl Span {
    pub fn new(start: usize, end: usize) -> Self {
        Self { start, end }
    }

    /// Returns true if the span overlaps with the given range
    /// inclusively.
    ///
    /// # Examples
    ///
    /// ```
    /// use starlark_lexer::Span;
    ///
    /// let span = Span::new(2, 3);
    /// assert!(span.overlaps(2..=3));
    /// assert!(span.overlaps(2..=4));
    /// assert!(span.overlaps(1..=3));
    /// assert!(span.overlaps(1..=4));
    /// assert!(!span.overlaps(0..=1));
    /// assert!(!span.overlaps(4..=5));
    /// ```
    pub fn overlaps(&self, range: Range<usize>) -> bool {
        self.start <= range.end && range.start <= self.end
    }

    fn merge(&self, range: Range<usize>) -> Self {
        Self {
            start: self.start.min(range.start),
            end: self.end.max(range.end),
        }
    }

    pub fn len(&self) -> usize {
        self.end - self.start
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Getters, MutGetters, Setters, TypedBuilder,
)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct Token {
    kind: TokenKind,
    lexeme: String,
    span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: String, span: Span) -> Self {
        Self { kind, lexeme, span }
    }

    fn set_text(&mut self, text: String) {
        self.lexeme = text;
    }

    pub fn range(&self) -> Range<usize> {
        self.span.start..self.span.end
    }

    pub fn pretty_print(&self) -> String {
        format!("{} {} {}", self.kind.blue(), self.lexeme, self.span.red(),)
    }

    pub fn new_eof() -> Self {
        Self {
            kind: TokenKind::EOF,
            lexeme: String::new(),
            span: Span::new(0, 0),
        }
    }

    pub fn is_keyword(&self, kw: &str) -> bool {
        self.lexeme().eq(kw)
    }

    pub fn is_trivia(&self) -> bool {
        matches!(
            self.kind(),
            TokenKind::WHITESPACE | TokenKind::COMMENT // | TokenKind::NEWLINE
                                                       // | TokenKind::INDENT
                                                       // | TokenKind::OUTDENT
        )
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.lexeme)
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd, Getters, MutGetters, Setters, TypedBuilder,
)]
pub struct TokenStream {
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    text: String,
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    tokens: Vec<Token>,
    #[builder(default = 0)]
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    cursor: usize,
    #[getset(get = "pub", get_mut = "pub", set = "pub")]
    file_name: PathBuf,
    file_id: FileId,
}

impl TokenStream {
    fn new(file_id: FileId, file_name: PathBuf) -> Self {
        let mut file = File::open(&file_name).expect("Unable to open file for lexing");
        let mut text = String::new();
        file.read_to_string(&mut text)
            .expect("Unable to read file for lexing");

        Self {
            tokens: Vec::new(),
            cursor: 0,
            file_id,
            file_name,
            text,
        }
    }

    fn from_db_file(file_id: FileId, db_file: SimpleFile<&str, String>) -> Self {
        let text = db_file.source().to_string();

        Self {
            tokens: Vec::new(),
            cursor: 0,
            file_id,
            file_name: db_file.name().into(),
            text,
        }
    }

    fn push(&mut self, token: Token) {
        self.tokens.push(token);
    }

    pub fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }

    fn peek_nth(&self, n: usize) -> Option<&Token> {
        assert!(
            n < self.len(),
            "Cannot peek a length greater than that ofthe token stream"
        );
        self.tokens.get(self.cursor + n)
    }

    pub fn lookahead_nth(&self, n: usize) -> Option<&Token> {
        self.peek_nth(n)
    }

    fn peek_next(&self) -> Option<&Token> {
        self.tokens.get(self.cursor + 1)
    }

    fn advance(&mut self) -> Option<&Token> {
        self.cursor += 1;
        self.tokens.get(self.cursor)
    }

    pub fn advance_n(&mut self, n: usize) -> Option<&Token> {
        self.cursor += n;
        self.tokens.get(self.cursor)
    }

    fn len(&self) -> usize {
        self.tokens.len()
    }

    fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    fn is_eof(&self) -> bool {
        self.cursor >= self.len()
    }

    fn is_at_end(&self) -> bool {
        self.cursor == self.len() - 1
    }

    fn is_keyword(&self, keyword: &str) -> bool {
        self.peek().map_or(false, |token| token.lexeme == keyword)
    }

    pub fn file_id(&self) -> FileId {
        self.file_id
    }

    pub fn source(&self) -> &str {
        &self.text
    }

    pub fn current_range(&self) -> Range<usize> {
        self.peek().map_or(0..0, |token| token.range())
    }

    pub fn current(&self) -> Option<&Token> {
        self.peek()
    }

    pub fn bump(&mut self) -> Option<&Token> {
        self.advance()
    }

    pub fn empty_stream() -> Self {
        Self {
            text: String::new(),
            tokens: Vec::new(),
            cursor: 0,
            file_id: 0,
            file_name: "empty_stream".into(),
        }
    }

    pub(crate) fn remove_trivia(&mut self) {
        self.tokens.retain(|token| !token.is_trivia());
    }
}

impl Iterator for TokenStream {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.cursor += 1;
        self.tokens.get(self.cursor).cloned()
    }
}

impl Display for TokenStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut output = String::new();

        for token in &self.tokens {
            output.push_str(&token.lexeme);
        }

        write!(f, "{output}")
    }
}

impl From<&str> for TokenStream {
    fn from(input: &str) -> Self {
        let mut files = SimpleFiles::new();

        let file_id = files.add("STDIN", input.to_string());
        let file_name = PathBuf::from("STDIN");
        let file = files.get(file_id).expect("Failed to get file from db");

        let token_sink = TokenSink::from_db_file(file_id, file);
        let mut lexer = StarlarkLexer::new();
        lexer.lex_db_file(file_id).expect("Failed to lex input");

        let tokens = token_sink.tokens.collect_vec();
        Self {
            tokens,
            cursor: 0,
            file_id,
            file_name,
            text: input.to_string(),
        }
    }
}

#[derive(Debug, Clone, Getters, MutGetters, Setters, TypedBuilder)]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
pub struct StarlarkLexer {
    files: SimpleFiles<String, String>,
    file_ids: Vec<FileId>,
    token_sink: TokenSink,
}

impl Default for StarlarkLexer {
    fn default() -> Self {
        Self::new()
    }
}

impl StarlarkLexer {
    pub fn new() -> Self {
        let mut files = SimpleFiles::new();
        let mut file_ids = Vec::new();

        let token_sink = TokenSink::empty_sink();

        Self {
            files,
            file_ids,
            token_sink,
        }
    }

    fn num_errors(&self) -> usize {
        self.token_sink.lexical_errors().len()
    }

    pub fn from_file(path: PathBuf) -> Result<(Self, FileId)> {
        let mut file = File::open(path.clone())?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        let mut files = SimpleFiles::new();
        let file_id = files.add(path.to_string_lossy().to_string(), contents.clone());

        Ok((
            Self {
                files,
                file_ids: vec![file_id],
                token_sink: TokenSink::new(file_id, path),
            },
            file_id,
        ))
    }

    pub fn add_file(&mut self, path: PathBuf) -> Result<FileId> {
        let mut file = File::open(&path)?;

        let mut input = String::new();
        file.read_to_string(&mut input)?;

        let file_id = self
            .files
            .add(path.to_string_lossy().to_string(), input.clone());

        self.file_ids.push(file_id);

        Ok(file_id)
    }

    pub fn lex_db_file(&mut self, file_id: FileId) -> Result<TokenSink> {
        let file = self.files.get(file_id)?;
        let input = file.source();

        let mut lexer = TokenKind::lexer(input);
        let mut token_sink = TokenSink::new(file_id, file.name().to_string().into());

        let mut current_unknown_token: Option<Token> = None;

        while let Some(token_result) = lexer.next() {
            match token_result {
                Ok(token) => {
                    if let Some(unknown_token) = current_unknown_token.clone() {
                        token_sink
                            .lexical_errors
                            .push(create_unknown_token_diagnostic(file_id, &unknown_token));
                        token_sink.tokens.push(unknown_token);
                        current_unknown_token = None;
                    }

                    token_sink.tokens.push(Token::new(
                        token,
                        lexer.slice().to_string(),
                        lexer.span().into(),
                    ));
                }
                Err(()) => {
                    if let Some(unknown_token) = current_unknown_token.clone() {
                        let Token {
                            kind: _,
                            span,
                            lexeme,
                        } = unknown_token;

                        let span = span.merge(lexer.span());
                        let updated_lexeme = format!("{}{}", lexeme, lexer.slice());

                        tracing::debug!(
                            "Gluing together unknown tokens {} and {} to form {} at {}",
                            lexeme,
                            lexer.slice(),
                            updated_lexeme,
                            span
                        );

                        current_unknown_token =
                            Some(Token::new(TokenKind::UNKNOWN, updated_lexeme, span));
                    } else {
                        tracing::debug!(
                            "Creating new unknown token {} at {:?}",
                            lexer.slice(),
                            lexer.span()
                        );

                        current_unknown_token = Some(Token::new(
                            TokenKind::UNKNOWN,
                            lexer.slice().to_string(),
                            lexer.span().into(),
                        ));
                    }
                }
            }
        }

        token_sink.tokens.push(Token::new(
            TokenKind::EOF,
            "".to_string(),
            lexer.span().into(),
        ));

        self.set_token_sink(token_sink.clone());

        Ok(token_sink)
    }

    pub fn emit_errors(&self) {
        let mut writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();

        tracing::debug!(
            "Emitting {} lexically invalid tokens",
            self.token_sink.lexical_errors().len()
        );

        for error in self.token_sink.lexical_errors() {
            term::emit(&mut writer, &config, self.files(), error).expect("Could not emit error");
        }

        tracing::info!(
            "{} lexically invalid tokens emitted",
            self.token_sink.lexical_errors().len()
        );

        // Ok(())
    }

    fn get_db_file(&self, file_id: usize) -> Result<&SimpleFile<String, String>> {
        if let Ok(file) = self.files.get(file_id) {
            Ok(file)
        } else {
            Err(anyhow!("File not found"))
            // Err(SkylarkError::FileNotFound(file_id).into())
        }
    }

    pub fn tokenize(&mut self, source: &str) -> TokenSink {
        // Create all intermediate directories if they don't exist
        let stdin_path = STDIN_PATH.as_os_str();
        let parent_dir = Path::new(&stdin_path)
            .parent()
            .expect("Failed to get parent directory");

        fs::create_dir_all(parent_dir).expect("Failed to create intermediate directories");

        // Create the file if it doesn't already exist, otherwise truncate it
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&STDIN_PATH)
            .expect("Could not open <STDIN>. This is a bug and should be reported.");

        // get the last segment of STDIN_PATH
        let stdin = STDIN_PATH
            .file_name()
            .expect("Failed to get file name")
            .to_str()
            .expect("Failed to convert file name to str");

        let mut file_id = self.files.add(stdin.to_string(), source.to_string());

        // create file if it doesn't already exist, otherwise truncate it
        let mut stdin = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&STDIN_PATH)
            .expect("Could not open <STDIN>. This is a bug and should be reported.");

        // write the source to the file
        stdin.write_all(source.as_bytes());

        let mut lexer = TokenKind::lexer(source);

        let mut token_sink = TokenSink::new(file_id, STDIN_PATH.to_path_buf());
        let mut current_unknown_token: Option<Token> = None;

        while let Some(token_result) = lexer.next() {
            match token_result {
                Ok(token) => {
                    if let Some(unknown_token) = current_unknown_token.clone() {
                        token_sink
                            .lexical_errors
                            .push(create_unknown_token_diagnostic(file_id, &unknown_token));
                        token_sink.tokens.push(unknown_token);
                        current_unknown_token = None;
                    }

                    token_sink.tokens.push(Token::new(
                        token,
                        lexer.slice().to_string(),
                        lexer.span().into(),
                    ));
                }
                Err(()) => {
                    if let Some(unknown_token) = current_unknown_token.clone() {
                        let Token {
                            kind: _,
                            span,
                            lexeme,
                        } = unknown_token;

                        let span = span.merge(lexer.span());
                        let updated_lexeme = format!("{}{}", lexeme, lexer.slice());

                        tracing::debug!(
                            "Gluing together unknown tokens {} and {} to form {} at {}",
                            lexeme,
                            lexer.slice(),
                            updated_lexeme,
                            span
                        );

                        current_unknown_token =
                            Some(Token::new(TokenKind::UNKNOWN, updated_lexeme, span));
                    } else {
                        tracing::debug!(
                            "Creating new unknown token {} at {:?}",
                            lexer.slice(),
                            lexer.span()
                        );

                        current_unknown_token = Some(Token::new(
                            TokenKind::UNKNOWN,
                            lexer.slice().to_string(),
                            lexer.span().into(),
                        ));
                    }
                }
            }
        }

        token_sink.tokens.push(Token::new(
            TokenKind::EOF,
            "".to_string(),
            lexer.span().into(),
        ));

        self.set_token_sink(token_sink.clone());

        token_sink
    }
}

fn create_unknown_token_diagnostic(file_id: usize, unknown_token: &Token) -> Diagnostic<usize> {
    Diagnostic::error()
        .with_code("E0000")
        .with_message(format!(
            "Unknown token encountered{} {}{}{}",
            ":".black(),
            "`".red(),
            unknown_token.lexeme.yellow(),
            "`".red(),
        ))
        .with_notes(
            vec![
                format!(
                    "The parser encountered an {}{} {}{}{}",
                    "unknown token".cyan().italic(),
                    ":".black(),
                    "`".red(),
                    unknown_token.lexeme.yellow(),
                    "`".red(),
                ),
                format!(
                    "This may be due to a {} or an {} in the input{}",
                    "typo".magenta().italic(),
                    "unsupported character".magenta().italic(),
                    ".".black()
                ),
                format!(
                    "Please check the input and make sure it contains {} {}{}",
                    "ONLY".blue().italic(),
                    "supported tokens".green().italic(),
                    ".".black()
                ),
                format!(
                    "For more information on {}{} please refer to the {}{}",
                    "supported tokens".green().italic(),
                    ",".black(),
                    "Starlark Language Specification".cyan().italic(),
                    ".".black()
                ),
            ]
            .into_iter()
            .collect(),
        )
        .with_labels(vec![
            Label::primary(file_id, unknown_token.span.start..unknown_token.span.end).with_message(
                format!(
                    "Unknown token found here{} {}{}{}",
                    ":".black(),
                    "`".red(),
                    unknown_token.lexeme.yellow(),
                    "`".red(),
                ),
            ),
            Label::secondary(file_id, unknown_token.span.start..unknown_token.span.end)
                .with_message(format!(
                    "Valid tokens should be used exclusively in the input{}",
                    ".".black()
                )),
        ])
}

#[derive(Debug, Clone, PartialEq, Eq, Getters, Setters, MutGetters, TypedBuilder)]
#[getset(get = "pub", set = "pub", get_mut = "pub")]
pub struct TokenSink {
    pub tokens: TokenStream,
    pub lexical_errors: Vec<Diagnostic<FileId>>,
}

impl TokenSink {
    pub fn new(file_id: FileId, file_name: PathBuf) -> Self {
        Self {
            tokens: TokenStream::new(file_id, file_name),
            lexical_errors: Vec::new(),
        }
    }

    pub fn empty_sink() -> Self {
        Self {
            tokens: TokenStream::empty_stream(),
            lexical_errors: Vec::new(),
        }
    }

    pub fn from_db_file(file_id: FileId, db_file: &SimpleFile<&str, String>) -> Self {
        Self {
            tokens: TokenStream::from_db_file(file_id, db_file.clone()),
            lexical_errors: Vec::new(),
        }
    }

    pub fn has_errors(&self) -> bool {
        !self.lexical_errors.is_empty()
    }

    pub fn add_error(&mut self, error: Diagnostic<FileId>) {
        self.lexical_errors.push(error);
    }
}

#[allow(non_camel_case_types, clippy::upper_case_acronyms)]
#[derive(Logos, Debug, Display, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum TokenKind {
    // Punctuation
    // #[regex("[\\+\\-\\*/%&|^<>]=?")]
    // #[regex("(==)|(!=)|(<=)|(>=)|(\\*\\*)|<<|>>|=")]
    // Punctuation
    #[token("+")]
    PLUS,
    #[token("-")]
    MINUS,
    #[token("*")]
    STAR,
    #[token("/")]
    SLASH,
    #[token("//")]
    DSLASH,
    #[token("%")]
    PERCENT,
    #[token("**")]
    DSTAR,
    #[token("~")]
    TILDE,
    #[token("&")]
    AMP,
    #[token("|")]
    PIPE,
    #[token("^")]
    CARET,
    #[token("<<")]
    LSHIFT,
    #[token(">>")]
    RSHIFT,
    #[token("=")]
    EQ,
    #[token("<")]
    LT,
    #[token(">")]
    GT,
    #[token(">=")]
    GE,
    #[token("<=")]
    LE,
    #[token("==")]
    EQEQ,
    #[token("!=")]
    NE,
    #[token("+=")]
    PLUSEQ,
    #[token("-=")]
    MINUSEQ,
    #[token("*=")]
    STAREQ,
    #[token("/=")]
    SLASHEQ,
    #[token("//=")]
    DSLASHEQ,
    #[token("%=")]
    PERCENTEQ,
    #[token("&=")]
    AMPEQ,
    #[token("|=")]
    PIPEEQ,
    #[token("^=")]
    CARETEQ,
    #[token("<<=")]
    LSHIFTEQ,
    #[token(">>=")]
    RSHIFTEQ,

    #[token(".")]
    DOT,
    #[token(",")]
    COMMA,
    #[token(";")]
    SEMICOLON,
    #[token(":")]
    COLON,
    #[token("(")]
    LPAREN,
    #[token(")")]
    RPAREN,
    #[token("[")]
    LBRACKET,
    #[token("]")]
    RBRACKET,
    #[token("{")]
    LBRACE,
    #[token("}")]
    RBRACE,

    // Keywords
    #[token("and")]
    AND_KW,
    #[token("else")]
    ELSE_KW,
    #[token("load")]
    LOAD_KW,
    #[token("break")]
    BREAK_KW,
    #[token("for")]
    FOR_KW,
    #[token("not")]
    NOT_KW,
    #[token("continue")]
    CONTINUE_KW,
    #[token("if")]
    IF_KW,
    #[token("or")]
    OR_KW,
    #[token("def")]
    DEF_KW,
    #[token("in")]
    IN_KW,
    #[token("pass")]
    PASS_KW,
    #[token("elif")]
    ELIF_KW,
    #[token("lambda")]
    LAMBDA_KW,
    #[token("return")]
    RETURN_KW,

    // Identifiers and literals
    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    IDENTIFIER,
    #[regex("\\d+")]
    INT,
    #[regex("(0x[0-9a-fA-F]+)|(0o[0-7]+)")]
    NumericLiteral,
    #[regex("\\d*\\.?\\d+([eE][\\+-]?\\d+)?", priority = 2)]
    FLOAT,
    #[regex("\"([^\"\\\\]|\\\\.)*\"|'([^'\\\\]|\\\\.)*'")]
    STRING,
    #[regex("b\"([^\"\\\\]|\\\\.)*\"|b'([^'\\\\]|\\\\.)*'")]
    BYTES,

    // Whitespace and special tokens
    #[regex("#[^\n]*")]
    COMMENT,
    #[regex("[ \t]+")]
    WHITESPACE,
    #[regex("\r?\n")]
    NEWLINE,

    #[token("    ")]
    INDENT,
    OUTDENT,
    UNKNOWN,
    #[end]
    EOF,
}

impl TokenKind {
    /// Convert a given [`TokenKind`] to a [`SyntaxKind`].
    /// This is used to convert the tokens from the **lexer** to the tokens
    /// used in the **parser** and the **syntax tree**.
    pub fn to_syntax(self) -> SyntaxKind {
        match self {
            TokenKind::PLUS => SyntaxKind::PLUS,
            TokenKind::MINUS => SyntaxKind::MINUS,
            TokenKind::STAR => SyntaxKind::STAR,
            TokenKind::SLASH => SyntaxKind::SLASH,
            TokenKind::DSLASH => SyntaxKind::DSLASH,
            TokenKind::PERCENT => SyntaxKind::PERCENT,
            TokenKind::DSTAR => SyntaxKind::DSTAR,
            TokenKind::TILDE => SyntaxKind::TILDE,
            TokenKind::AMP => SyntaxKind::AMP,
            TokenKind::PIPE => SyntaxKind::PIPE,
            TokenKind::CARET => SyntaxKind::CARET,
            TokenKind::LSHIFT => SyntaxKind::LSHIFT,
            TokenKind::RSHIFT => SyntaxKind::RSHIFT,
            TokenKind::EQ => SyntaxKind::EQ,
            TokenKind::LT => SyntaxKind::LT,
            TokenKind::GT => SyntaxKind::GT,
            TokenKind::GE => SyntaxKind::GE,
            TokenKind::LE => SyntaxKind::LE,
            TokenKind::EQEQ => SyntaxKind::EQEQ,
            TokenKind::NE => SyntaxKind::NE,
            TokenKind::PLUSEQ => SyntaxKind::PLUSEQ,
            TokenKind::MINUSEQ => SyntaxKind::MINUSEQ,
            TokenKind::STAREQ => SyntaxKind::STAREQ,
            TokenKind::SLASHEQ => SyntaxKind::SLASHEQ,
            TokenKind::PERCENTEQ => SyntaxKind::PERCENTEQ,
            TokenKind::AMPEQ => SyntaxKind::AMPEQ,
            TokenKind::PIPEEQ => SyntaxKind::PIPEEQ,
            TokenKind::CARETEQ => SyntaxKind::CARETEQ,
            TokenKind::LSHIFTEQ => SyntaxKind::LSHIFTEQ,
            TokenKind::RSHIFTEQ => SyntaxKind::RSHIFTEQ,
            TokenKind::DOT => SyntaxKind::DOT,
            TokenKind::COMMA => SyntaxKind::COMMA,
            TokenKind::SEMICOLON => SyntaxKind::SEMICOLON,
            TokenKind::COLON => SyntaxKind::COLON,
            TokenKind::LPAREN => SyntaxKind::LPAREN,
            TokenKind::RPAREN => SyntaxKind::RPAREN,
            TokenKind::LBRACKET => SyntaxKind::LBRACKET,
            TokenKind::RBRACKET => SyntaxKind::RBRACKET,
            TokenKind::LBRACE => SyntaxKind::LBRACE,
            TokenKind::RBRACE => SyntaxKind::RBRACE,
            TokenKind::AND_KW => SyntaxKind::AND_KW,
            TokenKind::ELSE_KW => SyntaxKind::ELSE_KW,
            TokenKind::LOAD_KW => SyntaxKind::LOAD_KW,
            TokenKind::BREAK_KW => SyntaxKind::BREAK_KW,
            TokenKind::FOR_KW => SyntaxKind::FOR_KW,
            TokenKind::NOT_KW => SyntaxKind::NOT_KW,
            TokenKind::CONTINUE_KW => SyntaxKind::CONTINUE_KW,
            TokenKind::IF_KW => SyntaxKind::IF_KW,
            TokenKind::OR_KW => SyntaxKind::OR_KW,
            TokenKind::DEF_KW => SyntaxKind::DEF_KW,
            TokenKind::IN_KW => SyntaxKind::IN_KW,
            TokenKind::PASS_KW => SyntaxKind::PASS_KW,
            TokenKind::ELIF_KW => SyntaxKind::ELIF_KW,
            TokenKind::LAMBDA_KW => SyntaxKind::LAMBDA_KW,
            TokenKind::RETURN_KW => SyntaxKind::RETURN_KW,
            TokenKind::IDENTIFIER => SyntaxKind::IDENTIFIER,
            TokenKind::INT => SyntaxKind::INT,
            TokenKind::NumericLiteral => SyntaxKind::INT,
            TokenKind::FLOAT => SyntaxKind::FLOAT,
            TokenKind::STRING => SyntaxKind::STRING,
            TokenKind::BYTES => SyntaxKind::BYTES,
            TokenKind::COMMENT => SyntaxKind::COMMENT,
            TokenKind::WHITESPACE => SyntaxKind::WHITESPACE,
            TokenKind::NEWLINE => SyntaxKind::NEWLINE,
            TokenKind::INDENT => SyntaxKind::INDENT,
            TokenKind::OUTDENT => SyntaxKind::OUTDENT,
            TokenKind::UNKNOWN => SyntaxKind::UNKNOWN,
            TokenKind::EOF => SyntaxKind::EOF,
            TokenKind::DSLASHEQ => SyntaxKind::SLASHEQ, // TODO: FIX ME (this is a placeholder for now as need to fix syntaxgen)
        }
    }

    pub fn is_whitespace(self) -> bool {
        matches!(
            self,
            TokenKind::WHITESPACE // TokenKind::WHITESPACE | TokenKind::NEWLINE | TokenKind::INDENT | TokenKind::OUTDENT
        )
    }
}

impl From<&SyntaxKind> for TokenKind {
    fn from(kind: &SyntaxKind) -> Self {
        match kind {
            SyntaxKind::PLUS => TokenKind::PLUS,
            SyntaxKind::MINUS => TokenKind::MINUS,
            SyntaxKind::STAR => TokenKind::STAR,
            SyntaxKind::SLASH => TokenKind::SLASH,
            SyntaxKind::DSLASH => TokenKind::DSLASH,
            SyntaxKind::PERCENT => TokenKind::PERCENT,
            SyntaxKind::DSTAR => TokenKind::DSTAR,
            SyntaxKind::TILDE => TokenKind::TILDE,
            SyntaxKind::AMP => TokenKind::AMP,
            SyntaxKind::PIPE => TokenKind::PIPE,
            SyntaxKind::CARET => TokenKind::CARET,
            SyntaxKind::LSHIFT => TokenKind::LSHIFT,
            SyntaxKind::RSHIFT => TokenKind::RSHIFT,
            SyntaxKind::EQ => TokenKind::EQ,
            SyntaxKind::LT => TokenKind::LT,
            SyntaxKind::GT => TokenKind::GT,
            SyntaxKind::GE => TokenKind::GE,
            SyntaxKind::LE => TokenKind::LE,
            SyntaxKind::EQEQ => TokenKind::EQEQ,
            SyntaxKind::NE => TokenKind::NE,
            SyntaxKind::PLUSEQ => TokenKind::PLUSEQ,
            SyntaxKind::MINUSEQ => TokenKind::MINUSEQ,
            SyntaxKind::STAREQ => TokenKind::STAREQ,
            SyntaxKind::SLASHEQ => TokenKind::SLASHEQ,
            SyntaxKind::PERCENTEQ => TokenKind::PERCENTEQ,
            SyntaxKind::AMPEQ => TokenKind::AMPEQ,
            SyntaxKind::PIPEEQ => TokenKind::PIPEEQ,
            SyntaxKind::CARETEQ => TokenKind::CARETEQ,
            SyntaxKind::LSHIFTEQ => TokenKind::LSHIFTEQ,
            SyntaxKind::RSHIFTEQ => TokenKind::RSHIFTEQ,
            SyntaxKind::DOT => TokenKind::DOT,
            SyntaxKind::COMMA => TokenKind::COMMA,
            SyntaxKind::SEMICOLON => TokenKind::SEMICOLON,
            SyntaxKind::COLON => TokenKind::COLON,
            SyntaxKind::LPAREN => TokenKind::LPAREN,
            SyntaxKind::RPAREN => TokenKind::RPAREN,
            SyntaxKind::LBRACKET => TokenKind::LBRACKET,
            SyntaxKind::RBRACKET => TokenKind::RBRACKET,
            SyntaxKind::LBRACE => TokenKind::LBRACE,
            SyntaxKind::RBRACE => TokenKind::RBRACE,
            SyntaxKind::AND_KW => TokenKind::AND_KW,
            SyntaxKind::ELSE_KW => TokenKind::ELSE_KW,
            SyntaxKind::LOAD_KW => TokenKind::LOAD_KW,
            SyntaxKind::BREAK_KW => TokenKind::BREAK_KW,
            SyntaxKind::FOR_KW => TokenKind::FOR_KW,
            SyntaxKind::NOT_KW => TokenKind::NOT_KW,
            SyntaxKind::CONTINUE_KW => TokenKind::CONTINUE_KW,
            SyntaxKind::IF_KW => TokenKind::IF_KW,
            SyntaxKind::OR_KW => TokenKind::OR_KW,
            SyntaxKind::DEF_KW => TokenKind::DEF_KW,
            SyntaxKind::IN_KW => TokenKind::IN_KW,
            SyntaxKind::PASS_KW => TokenKind::PASS_KW,
            SyntaxKind::ELIF_KW => TokenKind::ELIF_KW,
            SyntaxKind::LAMBDA_KW => TokenKind::LAMBDA_KW,
            SyntaxKind::RETURN_KW => TokenKind::RETURN_KW,
            SyntaxKind::IDENTIFIER => TokenKind::IDENTIFIER,
            SyntaxKind::INT => TokenKind::INT,
            SyntaxKind::FLOAT => TokenKind::FLOAT,
            SyntaxKind::STRING => TokenKind::STRING,
            SyntaxKind::BYTES => TokenKind::BYTES,
            SyntaxKind::COMMENT => TokenKind::COMMENT,
            SyntaxKind::WHITESPACE => TokenKind::WHITESPACE,
            SyntaxKind::NEWLINE => TokenKind::NEWLINE,
            SyntaxKind::INDENT => TokenKind::INDENT,
            SyntaxKind::OUTDENT => TokenKind::OUTDENT,
            SyntaxKind::UNKNOWN => TokenKind::UNKNOWN,
            SyntaxKind::EOF => TokenKind::EOF,
            _ => TokenKind::UNKNOWN,
        }
    }
}

pub fn tokenize(source: &str) -> (TokenStream, Vec<SyntaxError>) {
    // let files = SimpleFiles::new();

    // let stdin = dirs_next::cache_dir()
    //     .expect("Could not find cache dir")
    //     .join("<STDIN>");

    // // create file if it doesn't already exist, otherwise truncate it
    // let mut file = OpenOptions::new()
    //     .read(true)
    //     .write(true)
    //     .create(true)
    //     .truncate(true)
    //     .open(&stdin)
    //     .expect("Could not open <STDIN>. This is a bug and should be reported.");

    // // write the source to the file
    // file.write_all(source.as_bytes());

    // let file_id = files.add(
    //     stdin.to_str().expect("Could not convert path to str"),
    //     source,
    // );

    let mut lexer = StarlarkLexer::new();
    let TokenSink {
        tokens,
        lexical_errors,
    } = lexer.tokenize(source);

    if !lexical_errors.is_empty() {
        lexer.emit_errors();

        (
            tokens,
            lexical_errors
                .into_iter()
                .map(|e| {
                    let span = &e.labels.first().expect("No labels found").range;
                    let text_range = TextRange::new(
                        TextSize::from(span.start as u32),
                        TextSize::from(span.end as u32),
                    );
                    SyntaxError::new(e.message, text_range)
                })
                .collect(),
        )
    } else {
        (tokens, Vec::new())
    }

    // let tokens = TokenStream::from(source);

    // let mut errors = Vec::new();

    // for token in tokens.tokens.iter() {
    //     if let TokenKind::UNKNOWN = token.kind {
    //         errors.push(SyntaxError::new(
    //             format!("Unknown token found here: {}", token.lexeme),
    //             token.span.into(),
    //         ));
    //     }
    // }

    // (tokens, errors)
}
