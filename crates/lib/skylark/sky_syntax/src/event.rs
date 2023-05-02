use crate::{parser::ParseError, SyntaxKind};

#[derive(Debug)]
pub enum Event {
    Start {
        kind: SyntaxKind,
        forward_parent: Option<u32>,
    },

    Finish,

    StartError,

    FinishError(ParseError),

    Token,

    Error(ParseError),
}
