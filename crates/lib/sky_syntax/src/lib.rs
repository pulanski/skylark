mod ast;
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
    ast::SyntaxKind,
    lang::{SyntaxNode, SyntaxToken},
    lexer::{StarlarkLexer, TokenSink},
    logging::init_logging,
    parser::StarlarkParser,
    parser::TextTreeSink,
};
use ast::AstNode;
use rowan::GreenNode;
use std::{marker::PhantomData, sync::Arc};
use syntax_error::SyntaxError;

#[derive(Debug, PartialEq, Eq)]
pub struct Parse<T> {
    green: GreenNode,
    errors: Arc<[SyntaxError]>,
    _ty: PhantomData<fn() -> T>,
}

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
    fn new(green: GreenNode, errors: Vec<SyntaxError>) -> Parse<T> {
        Parse {
            green,
            errors: Arc::from(errors),
            _ty: PhantomData,
        }
    }
    pub fn syntax_node(&self) -> SyntaxNode {
        SyntaxNode::new_root(self.green.clone())
    }
}
impl<T: AstNode> Parse<T> {
    pub fn into_syntax(self) -> Parse<SyntaxNode> {
        Parse {
            green: self.green,
            errors: self.errors,
            _ty: PhantomData,
        }
    }
    pub fn tree(&self) -> T {
        T::cast(self.syntax_node()).unwrap()
    }
    pub fn errors(&self) -> &[SyntaxError] {
        &self.errors
    }
    pub fn ok(self) -> Result<T, Arc<[SyntaxError]>> {
        if self.errors.is_empty() {
            Ok(self.tree())
        } else {
            Err(self.errors)
        }
    }
}
