use std::fmt;

use rowan::{TextRange, TextSize};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SyntaxError(String, TextRange);

impl SyntaxError {
    pub fn new(msg: String, range: TextRange) -> SyntaxError {
        SyntaxError(msg, range)
    }

    pub fn new_at_offset(message: impl Into<String>, offset: TextSize) -> SyntaxError {
        SyntaxError(message.into(), TextRange::empty(offset))
    }

    pub fn range(&self) -> TextRange {
        self.1
    }

    pub fn with_range(mut self, range: TextRange) -> Self {
        self.1 = range;
        self
    }
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.fmt(f)
    }
}
