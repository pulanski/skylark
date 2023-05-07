use crate::{parser::Parser, TokenSet, T};

pub(super) const DECLARATION_RECOVERY_SET: TokenSet =
    TokenSet::new(&[T![def], T![load], T![if], T![for], T![in], T![return]]);

