use crate::{parser::Parser, SyntaxKind::*, TokenSet, T};

use super::{expr, statements};

pub(super) const DECLARATION_RECOVERY_SET: TokenSet =
    TokenSet::new(&[T![def], T![load], T![if], T![for], T![in], T![return]]);

/// Represents **one or more** variables in a **loop statement**.
///
/// See [`expr::primary_expr`] for more information.
///
/// ## Ungrammar
///
/// ```
/// LoopVariables = PrimaryExpr (',' PrimaryExpr)*
/// ```
///
/// ## Examples
///
/// ```starlark
/// x
/// x, y
/// key, value
/// ```
pub(super) fn loop_variables(p: &mut Parser) {
    let m = p.start();

    expr::primary_expr(p);
    while p.eat(T![,]) {
        expr::primary_expr(p);
    }
    m.complete(p, LOOP_VARIABLES);
}

/// A **suite of statements**, either indented or a **simple statement**.
///
/// See [`statements::simple_stmt`] and [`statements::statement`] for more
/// information.
///
/// ## Ungrammar
///
/// ```
/// Suite =
///  ('newline' 'indent' Statement* 'outdent')?
/// | SimpleStmt
/// ```
///
/// ## Examples
///
/// ```starlark
/// x = 1
///
/// if x > 0:
///    y = x
/// else:
///   y = -x
/// ```
pub(super) fn suite(p: &mut Parser) {
    if p.at(T![newline]) {
        let m = p.start();
        p.bump(T![newline]);
        p.expect(T![indent]);

        while !p.at(T![outdent]) {
            statements::statement(p);
        }
        p.expect(T![outdent]);
        m.complete(p, SUITE);
    } else {
        statements::simple_stmt(p);
    }
}
