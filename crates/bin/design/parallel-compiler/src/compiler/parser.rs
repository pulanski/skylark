use std::sync::Arc;

use super::lexer::TokenStream;
// task::{TaskInput, TaskOutput},

// Define an AST for the simple example language.
#[derive(Debug)]
pub enum Expr {
    Function(String, Vec<Expr>),
    Integer(i64),
}

#[derive(Debug)]
pub struct AST {
    pub root: Vec<Expr>,
}

pub(crate) async fn parse(input: TokenStream) -> AST {
    // ... (previous code)

    // let ast = todo!();

    // *output.lock().await = Some(Arc::new(ast));

    todo!("parse")
}
