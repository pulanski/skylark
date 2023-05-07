use crate::{parser::Parser, SyntaxKind, T};

mod decl;
mod expr;
mod statements;

/// The **root** of a **Starlark file**. In Starlark, **files** are
/// the **root** of the **syntax tree**.
///
/// ## Ungrammar
///
/// ```
/// Root = File
/// ```
pub(crate) fn root(p: &mut Parser) {
    let m = p.start();
    file(p);
    m.complete(p, SyntaxKind::FILE);
}

/// A **file**. In Starlark, **files** are **comma-separated lists** of
/// [`statements::statement`]'s.
///
/// See [`statements::statement`] for more information.
///
/// ## Ungrammar
///
/// ```
/// File = (Statement | 'newline')* 'eof'
/// ```
///
/// ## Examples
///
/// ```starlark
/// def area(x, y):
///     return x * y
///
/// def volume(x, y, z):
///     return x * y * z
/// ```
pub(super) fn file(p: &mut Parser) {
    while !p.at(T![eof]) {
        if p.at(T![newline]) {
            p.bump(T![newline]);
        } else {
            statements::statement(p);
        }
    }
}
