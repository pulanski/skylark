use crate::{parser::Parser, SyntaxKind::*, T};

use super::{decl, expr};

/// A **list comprehension expression**. In Starlark, **list comprehension
/// expressions** are shorthand for creating a list in a _terse_ manner.
///
/// See [`expr::test`] and [`comp_clause`] for
/// more information.
///
/// ## Ungrammar
///
/// ```
/// ListComp = '[' Test CompClause* ']'
/// ```
///
/// ## Examples
///
/// ```starlark
/// [x * 2 for x in range(10) if x % 2 == 0]
/// ```
pub(super) fn list_comp(p: &mut Parser) {
    let m = p.start();

    p.expect(T!['[']);
    expr::test(p);
    while p.at(T![for]) || p.at(T![if]) {
        comp_clause(p);
    }
    p.expect(T![']']);

    m.complete(p, LIST_COMP);
}

/// A **dictionary expression**. In Starlark, **dictionary expressions** are
/// **comma-separated lists** of [`entries`]. Entries are **key-value pairs**
/// separated by a colon.
///
/// See [`entries`] for more information.
///
/// ## Ungrammar
///
/// ```
/// DictExpr = '{' (Entries (',')?)? '}'
/// ```
///
/// ## Examples
///
/// ```starlark
/// {"x": 1, "y": 2}
/// ```
pub(super) fn dict_expr(p: &mut Parser) {
    let m = p.start();
    p.expect(T!['{']);

    if !p.at(T!['}']) {
        entries(p);
        while p.eat(T![,]) {
            if p.at(T!['}']) {
                break;
            }
            entries(p);
        }
    }

    p.expect(T!['}']);
    m.complete(p, DICT_EXPR);
}

/// A **dictionary comprehension expression**.
///
/// See [`expr::expression`] and [`expr::test`] for
/// usage examples.
///
/// ## Ungrammar
///
/// ```
/// DictComp = '{' Entry CompClause* '}'
/// ```
///
/// ## Examples
///
/// ```starlark
/// {x: x * 2 for x in range(10) if x % 2 == 0}
/// ```
pub(super) fn dict_comp(p: &mut Parser) {
    let m = p.start();
    p.expect(T!['{']);
    entry(p);

    while p.at(T![for]) || p.at(T![if]) {
        comp_clause(p);
    }

    p.expect(T!['}']);
    m.complete(p, DICT_COMP);
}

/// A **list of entries** in a **dictionary**.
///
/// See [`entry`] for more information.
///
/// ## Ungrammar
///
/// ```
/// Entries = Entry (',' Entry)*
/// ```
///
/// ## Examples
///
/// ```starlark
/// "x": 1, "y": 2
/// ```
pub(super) fn entries(p: &mut Parser) {
    let m = p.start();

    entry(p);
    while p.eat(T![,]) {
        entry(p);
    }

    m.complete(p, ENTRIES);
}

/// A single entry in a dictionary.
///
/// See [`expr::expression`] and [`expr::test`] for
/// usage examples.
///
/// ## Ungrammar
///
/// ```
/// Entry = Test ':' Test
/// ```
///
/// ## Examples
///
/// ```starlark
/// "x": 1
/// ```
pub(super) fn entry(p: &mut Parser) {
    let m = p.start();

    expr::test(p);
    p.expect(T![:]);
    expr::test(p);

    m.complete(p, ENTRY);
}

/// A comprehension clause, including for and if clauses.
///
/// See [`expr::expression`] and [`expr::test`] for
/// usage examples.
///
/// ## Ungrammar
///
/// ```
/// CompClause = 'for' LoopVariables 'in' Test | 'if' Test
/// ```
///
/// ## Examples
///
/// ```starlark
/// for x in range(10)
/// if x % 2 == 0
/// ```
pub(super) fn comp_clause(p: &mut Parser) {
    // TODO: add error diagnostics
    let m = p.start();

    if p.at(T![for]) {
        p.bump(T![for]);
        decl::loop_variables(p);
        p.expect(T![in]);
        expr::test(p);
        m.complete(p, COMP_CLAUSE);
    } else if p.at(T![if]) {
        p.bump(T![if]);
        expr::test(p);
        m.complete(p, COMP_CLAUSE);
    } else {
        // p.error("expected 'for' or 'if'");
        m.abandon(p);
    }
}
