use std::mem;

use logos::Source;
use rowan::{GreenNode, TextRange, TextSize};

use crate::{
    lexer::TokenStream, parser::ParseError, syntax_error::SyntaxError,
    syntax_tree::SyntaxTreeBuilder, SyntaxKind,
};

use super::TreeSink;

#[derive(Debug)]
pub(crate) struct TextTreeSink {
    /// Contains the tokens that are being parsed, and the current cursor position, along
    /// with the file id and file name and raw text used for error reporting.
    tokens: TokenStream,
    text_pos: TextSize,
    inner: SyntaxTreeBuilder,
    state: State,
}

// text: String,
//     tokens: Vec<Token>,
//     cursor: usize,
//     file_id: FileId,
//     file_name: PathBuf

#[derive(Debug)]
enum State {
    PendingStart,
    Normal,
    PendingFinish,
}

impl TextTreeSink {
    // ...
    pub fn new(tokens: TokenStream) -> TextTreeSink {
        TextTreeSink {
            tokens,
            text_pos: 0.into(),
            inner: SyntaxTreeBuilder::new(),
            state: State::PendingStart,
        }
    }

    pub(super) fn finish(mut self) -> (GreenNode, Vec<SyntaxError>) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingFinish => {
                self.eat_trivias();
                self.inner.finish_node()
            }
            State::PendingStart | State::Normal => unreachable!(),
        }
        self.inner.finish_raw()
    }

    fn eat_trivias(&mut self) {
        while let Some(token) = self.tokens.peek() {
            if !token.is_trivia() {
                break;
            }
            self.do_token(
                token.kind().to_syntax(),
                TextSize::from(token.span().len() as u32),
                1,
            );
        }
    }

    fn do_token(&mut self, kind: SyntaxKind, len: TextSize, n_tokens: usize) {
        let range = TextRange::at(self.text_pos, len);
        let text = self
            .tokens
            .text()
            .slice(range.into())
            .expect("invalid range")
            .to_string();
        self.text_pos += len;
        self.tokens.advance_n(n_tokens);
        self.inner.add_raw_token(kind, &text);

        // let text = &self.text[range];
        // self.text_pos += len;
        // self.token_pos += n_tokens;
        // self.inner.token(kind, text);
    }
}

impl TreeSink for TextTreeSink {
    fn token(&mut self, kind: SyntaxKind, n_tokens: u8) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingStart => unreachable!(),
            State::PendingFinish => self.inner.finish_node(),
            State::Normal => (),
        }

        // Get the token from the token stream
        let token = self.tokens.next().unwrap(); // TODO refactor here a bit

        // Add the token to the syntax tree
        self.inner.add_token(&token);

        // Advance the text position
        self.text_pos += TextSize::from(token.lexeme().len() as u32);
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        match mem::replace(&mut self.state, State::Normal) {
            State::PendingStart => {
                self.inner.start_node(kind);
                // No need to attach trivias to previous node; there is no previous node.
                return;
            }
            State::PendingFinish => self.inner.finish_node(),
            State::Normal => (),
        }

        self.inner.start_node(kind);
    }

    fn finish_node(&mut self) {
        match mem::replace(&mut self.state, State::PendingFinish) {
            State::PendingStart => unreachable!(),
            State::PendingFinish => self.inner.finish_node(),
            State::Normal => (),
        }
    }

    fn error(&mut self, error: ParseError) {
        self.inner.error(
            error,
            TextRange::new(
                TextSize::from(self.tokens.current_range().start as u32),
                TextSize::from(self.tokens.current_range().end as u32),
            ),
        );
    }
}
