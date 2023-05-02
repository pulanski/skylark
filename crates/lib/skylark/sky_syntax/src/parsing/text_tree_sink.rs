use rowan::{TextRange, TextSize};

use crate::{lexer::TokenStream, parser::ParseError, syntax_tree::SyntaxTreeBuilder, SyntaxKind};

use super::TreeSink;

pub(crate) struct TextTreeSink {
    /// Contains the tokens that are being parsed, and the current cursor position, along
    /// with the file id and file name and raw text used for error reporting.
    tokens: TokenStream,
    text_pos: TextSize,
    inner: SyntaxTreeBuilder,
}

// text: String,
//     tokens: Vec<Token>,
//     cursor: usize,
//     file_id: FileId,
//     file_name: PathBuf,

impl TextTreeSink {
    // ...
    pub fn new(tokens: TokenStream) -> TextTreeSink {
        TextTreeSink {
            tokens,
            text_pos: 0.into(),
            inner: SyntaxTreeBuilder::new(),
        }
    }
}

impl TreeSink for TextTreeSink {
    fn token(&mut self, kind: SyntaxKind, n_tokens: u8) {
        // Get the token from the token stream
        let token = self.tokens.next().unwrap(); // TODO refactor here a bit

        // Add the token to the syntax tree
        self.inner.add_token(&token);

        // Advance the text position
        self.text_pos += TextSize::from(token.lexeme().len() as u32);
    }

    fn start_node(&mut self, kind: SyntaxKind) {
        self.inner.start_node(kind);
    }

    fn finish_node(&mut self) {
        self.inner.finish_node();
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
