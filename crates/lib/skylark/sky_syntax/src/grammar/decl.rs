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
#[tracing::instrument(level = "debug", skip(p))]
pub(super) fn loop_variables(p: &mut Parser) {
    tracing::debug!("Parsing loop variables. Current token: {:?}", p.current());
    let m = p.start();

    expr::primary_expr(p);
    while p.eat(T![,]) {
        tracing::debug!("Found comma. Parsing primary expression");
        expr::primary_expr(p);
    }

    tracing::debug!("Finished parsing loop variables");
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
#[tracing::instrument(level = "debug", skip(p))]
pub(super) fn suite(p: &mut Parser) {
    tracing::debug!("Parsing suite. Current token: {:?}", p.current());
    let m = p.start();

    if p.at(T![newline]) {
        tracing::debug!("Found newline. Bumping to next token");
        p.bump(T![newline]);
        p.expect(T![indent]);

        while !p.at(T![outdent]) {
            tracing::debug!("Not reached outdent yet. Parsing statement");
            statements::statement(p);
        }

        p.expect(T![outdent]);
    } else {
        tracing::debug!("No newline found. Parsing simple statement");
        statements::simple_stmt(p);
    }

    tracing::debug!("Finished parsing suite");
    m.complete(p, SUITE);
}
