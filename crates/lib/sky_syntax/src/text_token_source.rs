use derive_new::new;
use shrinkwraprs::Shrinkwrap;

use crate::lexer::TokenStream;

#[derive(Debug, Clone, PartialEq, Eq, Hash, new, Shrinkwrap)]
pub struct TextTokenSource {
    tokens: TokenStream,
}
