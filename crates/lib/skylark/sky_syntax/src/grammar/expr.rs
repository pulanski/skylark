use crate::{parser::Parser, SyntaxKind::*, T};

/// An **expression**. In Starlark, **expressions** are
/// **comma-separated lists** of [`test`]'s. **Expressions** are the
/// **basis** of **statements** and **comprehensions**.
///
/// See [`test`] for more information.
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
/// See [`if_expr`], [`primary_expr`], [`unary_expr`], [`binary_expr`],
/// and [`lambda_expr`] for usage examples and more information.
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
    // TODO: Add `UnaryExpr`, `BinaryExpr`, and `LambdaExpr`.
    let m = p.start();

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

/// An **operand** in an **expression**. In Starlark, **operands** are
/// **syntax nodes** which represent the components of **expressions** (e.g. `x` in `x + y`).
///
/// See [`expression`], [`list_expr`], [`list_comp`], [`dict_expr`], and [`dict_comp`] for more information.
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

/// A **dot suffix** node in a **primary expression**. In Starlark, **dot suffixes** are
/// **syntax nodes** that represent the **access** of a **primary expression**.
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
    let m = p.start();

    p.expect(T![.]);
    p.expect(T![identifier]);

    m.complete(p, DOT_SUFFIX);
}

/// A **slice suffix** node in a **primary expression**. In Starlark, **slice suffixes** are
/// **syntax nodes** that represent the **slicing** of a **primary expression**.
/// Slicing is a way to _extract a subset_ of a **list**, **tuple,** or **string**.
///
/// See [`expression`] and [`test`] for more information.
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
    let m = p.start();

    p.expect(T!['[']);
    if p.eat(T![:]) {
        test(p);
        if p.eat(T![:]) {
            test(p);
        }
    } else {
        expression(p);
        if p.eat(T![:]) {
            test(p);
            if p.eat(T![:]) {
                test(p);
            }
        }
    }
    p.expect(T![']']);

    m.complete(p, SLICE_SUFFIX);
}

/// A **call suffix** node in a **primary expression**.
///
/// See [`arguments`] for more information.
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
    let m = p.start();

    p.expect(T!['(']);
    if !p.at(T![')']) {
        arguments(p);
        p.eat(T![,]);
    }
    p.expect(T![')']);

    m.complete(p, CALL_SUFFIX);
}

/// A comma-separated list of **arguments** in a **function call**. This can be a
/// **positional argument**, a **keyword argument**, a **star argument**,
/// or a **double star argument**.
///
/// See [`argument`] for more information.
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
    let m = p.start();

    argument(p);
    while p.eat(T![,]) {
        argument(p);
    }

    m.complete(p, ARGUMENTS);
}

/// A **single argument** in a **function call**. This can be a
/// **positional argument**, a **keyword argument**, a **star argument**,
/// or a **double star argument**.
///
/// See [`test`] for usage examples.
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
    let m = p.start();

    if p.at(T![identifier]) && p.nth_at(1, T![=]) {
        p.bump(T![identifier]);
        p.expect(T![=]);
        test(p);
    } else if p.at(T![*]) {
        p.bump(T![*]);
        test(p);
    } else if p.at(T![**]) {
        p.bump(T![**]);
        test(p);
    } else {
        test(p);
    }

    m.complete(p, ARGUMENT);
}

/// A **list expression**. In Starlark, **list expressions** are
/// **comma-separated lists** of [`expression`]'s.
///
/// See [`expression`] for usage examples.
///
/// ## Ungrammar
///
/// ```
/// ListExpr = '[' (Expression (',')?)? ']'
/// ```
///
/// ## Examples
///
/// ```starlark
/// [1, 2, 3]
/// ```
pub(super) fn list_expr(p: &mut Parser) {
    let m = p.start();
    p.expect(T!['[']);

    if !p.at(T![']']) {
        expression(p);
        while p.eat(T![,]) {
            if p.at(T![']']) {
                break;
            }
            expression(p);
        }
    }

    p.expect(T![']']);
    m.complete(p, LIST_EXPR);
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
