use crate::{parser::Parser, SyntaxKind::*, T};

/// An **expression**. In Starlark, **expressions** are
/// **comma-separated lists** of [`expr::test`]'s.
///
/// See [`expr::test`] for more information.
///
/// ## Ungrammar
///
/// ```
/// Expression = Test (',' Test)*
/// ```
///
/// ## Examples
///
/// ```starlark
/// x
/// x, y, z
/// x + y, y + z, z + x
/// ```
pub(super) fn expression(p: &mut Parser) {
    let m = p.start();
    test(p);
    while p.eat(T![,]) {
        test(p);
    }
    m.complete(p, EXPRESSION);
}

/// A **test**. In Starlark, **tests** are **syntax nodes** that represent
/// the **basis** of **expressions**.
///
/// See [`expr::expression`] for more information.
///
/// ## Ungrammar
///
/// ```
/// Test =
///    IfExpr
///  | PrimaryExpr
///  | UnaryExpr
///  | BinaryExpr
///  | LambdaExpr
/// ```
///
/// ## Examples
///
/// ```starlark
/// x + y
/// x > 3
/// x if x > 3 else y
/// ```
pub(super) fn test(p: &mut Parser) {
    let m = p.start();
    // TODO: Add `UnaryExpr`, `BinaryExpr`, and `LambdaExpr`.
    if p.at(T![if]) {
        if_expr(p);
    } else {
        primary_expr(p);
    }
    m.complete(p, TEST);
}

/// A **primary expression**. In Starlark, **primary expressions** are
/// **syntax nodes** which represent the components of **expressions** (e.g. operand `x` in `x + y`).
///
/// See [`expr::expression`] and [`expr::test`] for
/// usage examples.
///
/// ## Ungrammar
///
/// ```
/// PrimaryExpr =
///     Operand
///  | PrimaryExpr DotSuffix
///  | PrimaryExpr CallSuffix
///  | PrimaryExpr SliceSuffix
/// ```
///
/// ## Examples
///
/// ```starlark
/// x
/// x.y
/// x.foo()
/// ```
pub(super) fn primary_expr(p: &mut Parser) {
    // TODO: This is not correct.
    let m = p.start();
    operand(p);
    loop {
        if p.at(T![.]) {
            dot_suffix(p);
        } else if p.at(T!['(']) {
            call_suffix(p);
        } else if p.at(T!['[']) {
            slice_suffix(p);
        } else {
            break;
        }
    }

    m.complete(p, PRIMARY_EXPR);
}

/// An _inline_ **if expression**.
///
/// See [`expr::expression`] and [`expr::test`] for
/// more usage examples.
///
/// ## Ungrammar
///
/// ```
/// IfExpr = Test 'if' Test 'else' Test
/// ```
///
/// ## Examples
///
/// ```starlark
/// x if x > 0 else -x
/// ```
pub(super) fn if_expr(p: &mut Parser) {
    let m = p.start();

    test(p);
    p.expect(T![if]);
    test(p);
    p.expect(T![else]);
    test(p);

    m.complete(p, IF_EXPR);
}

/// An operand in an expression.
///
/// See [`expr::expression`] and [`expr::test`] for
/// usage examples.
///
/// ## Ungrammar
///
/// ```
/// Operand =
///     'identifier'
///   | 'int' | 'float' | 'string' | 'bytes'
///   | ListExpr | ListComp
///   | DictExpr | DictComp
///   | '(' (Expression (',')?)? ')'
/// ```
///
/// ## Examples
///
/// ```starlark
/// x
/// 1
/// [1, 2, 3]
/// ```
pub(super) fn operand(p: &mut Parser) {
    let m = p.start();
    match p.current() {
        T![identifier] | T![int] | T![float] | T![string] | T![bytes] => p.bump_any(),
        T!['('] => {
            p.bump(T!['(']);
            if !p.at(T![')']) {
                expression(p);
                p.eat(T![,]);
            }
            p.expect(T![')']);
        }

        // Add cases for ListExpr, ListComp, DictExpr, and DictComp when implemented
        // _ => p.err_and_bump("expected an operand"),
        _ => {
            todo!("Add cases for ListExpr, ListComp, DictExpr, and DictComp when implemented")
        }
    }
    m.complete(p, OPERAND);
}

/// A **dot suffix node** in a **primary expression**.
///
/// See [`expr::expression`] and [`expr::test`] for
/// usage examples.
///
/// ## Ungrammar
///
/// ```
/// DotSuffix = '.' 'identifier'
/// ```
///
/// ## Examples
///
/// ```starlark
/// x.foo
/// ```
pub(super) fn dot_suffix(p: &mut Parser) {
    todo!()
}

/// A slice suffix node in a primary expression.
///
/// See [`expr::expression`] and [`expr::test`] for
/// usage examples.
///
/// ## Ungrammar
///
/// ```
/// SliceSuffix =
///     '[' Expression? ':' Test? (':' (Test)?)? ']'
///   | '[' Expression ']'
/// ```
///
/// ## Examples
///
/// ```starlark
/// x[1:3]
/// x[1:]
/// ```
pub(super) fn slice_suffix(p: &mut Parser) {
    todo!()
}

/// A call suffix node in a primary expression.
///
/// See [`expr::expression`] and [`expr::test`] for
/// usage examples.
///
/// ## Ungrammar
///
/// ```
/// CallSuffix = '(' (Arguments (',')?)? ')'
/// ```
///
/// ## Examples
///
/// ```starlark
/// x.foo(1, 2, 3)
/// ```
pub(super) fn call_suffix(p: &mut Parser) {
    todo!()
}

/// A list of arguments for a function call.
///
/// See [`expr::expression`] and [`expr::test`] for
/// usage examples.
///
/// ## Ungrammar
///
/// ```
/// Arguments = Argument (',' Argument)*
/// ```
///
/// ## Examples
///
/// ```starlark
/// 1, 2, 3
/// ```
pub(super) fn arguments(p: &mut Parser) {
    todo!()
}

/// A single argument in a function call.
///
/// See [`expr::expression`] and [`expr::test`] for
/// usage examples.
///
/// ## Ungrammar
///
/// ```
/// Argument = Test | 'identifier' '=' Test | '*' Test | '**' Test
/// ```
///
/// ## Examples
///
/// ```starlark
/// x
/// x=1
/// args
/// **kwargs
/// ```
pub(super) fn argument(p: &mut Parser) {
    todo!()
}
