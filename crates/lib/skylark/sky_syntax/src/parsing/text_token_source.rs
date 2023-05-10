use derive_new::new;
use shrinkwraprs::Shrinkwrap;

use crate::{
    lexer::{Token, TokenStream},
    parsing::TokenSource,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, Shrinkwrap)]
pub struct TextTokenSource {
    tokens: TokenStream,
}

impl TextTokenSource {
    pub fn new(mut tokens: TokenStream) -> Self {
        // Construct a token stream from the tokens, but remove the trivia tokens
        tokens.remove_trivia();

        Self { tokens }
    }
}

impl TokenSource for TextTokenSource {
    fn current(&self) -> Token {
        self.tokens.current().unwrap_or(&Token::new_eof()).clone()
    }

    fn lookahead_nth(&self, n: usize) -> Token {
        self.tokens
            .lookahead_nth(n)
            .unwrap_or(&Token::new_eof())
            .clone()
    }

    fn bump(&mut self) {
        self.tokens.bump();
    }

    fn is_keyword(&self, kw: &str) -> bool {
        self.current().is_keyword(kw)
    }
}
