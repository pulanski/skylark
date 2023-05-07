use super::chunk::SourceChunk;
// task::{TaskInput, TaskOutput},
use std::sync::Arc;
use tracing::debug;

// Define tokens for our simple example language.
#[derive(Debug, Clone)]
pub enum Token {
    Def,
    Identifier(String),
    Integer(i64),
    Colon,
    Comma,
    OpenParen,
    CloseParen,
    Plus,
    Minus,
    Star,
    Slash,
}

// Implement a simple lexer for the example language.
fn lex_chunk(source_chunk: &SourceChunk) -> Vec<Token> {
    let mut tokens = Vec::new();

    let input = source_chunk.code.as_str();
    let mut iter = input.chars().peekable();

    while let Some(&c) = iter.peek() {
        match c {
            ' ' | '\t' | '\n' | '\r' => {
                iter.next();
            }
            'a'..='z' | 'A'..='Z' => {
                let mut ident = String::new();
                while let Some(&c) = iter.peek() {
                    if c.is_alphabetic() {
                        ident.push(c);
                        iter.next();
                    } else {
                        break;
                    }
                }
                if ident == "def" {
                    tokens.push(Token::Def);
                } else {
                    tokens.push(Token::Identifier(ident));
                }
            }
            '0'..='9' => {
                let mut num = 0;
                while let Some(&c) = iter.peek() {
                    if c.is_ascii_digit() {
                        num = num * 10 + c.to_digit(10).unwrap() as i64;
                        iter.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Integer(num));
            }
            '(' => {
                tokens.push(Token::OpenParen);
                iter.next();
            }
            ')' => {
                tokens.push(Token::CloseParen);
                iter.next();
            }
            ':' => {
                tokens.push(Token::Colon);
                iter.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                iter.next();
            }
            '+' => {
                tokens.push(Token::Plus);
                iter.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                iter.next();
            }
            '*' => {
                tokens.push(Token::Star);
                iter.next();
            }
            '/' => {
                tokens.push(Token::Slash);
                iter.next();
            }
            _ => {
                panic!("Unexpected character: {c}");
            }
        }
    }

    debug!(
        "Lexed tokens: {:?} from source chunk: {:?}",
        tokens, source_chunk
    );

    tokens
}

#[derive(Debug)]
pub struct TokenStream {
    pub tokens: Vec<Token>,
}

pub(crate) async fn lex(input: SourceChunk) -> TokenStream {
    // ... (previous code)
    // let token_stream = todo!();

    // let token_stream = Arc::new(tokio_stream::iter(partition_source(&source_code)));

    // *output.lock().await = Some(Arc::new(token_stream));
    todo!("lex")
}
