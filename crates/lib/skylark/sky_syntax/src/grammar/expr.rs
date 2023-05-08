use once_cell::sync::Lazy;

use crate::{parser::Parser, SyntaxKind::*, TokenSet, T};

use super::statements::{self, PARAM_START};

pub static UNARY_OP: TokenSet = TokenSet::new(&[T![not], T![-], T![+], T![~]]);

pub const BIN_OP: TokenSet = TokenSet::new(&[
    T![+],
    T![-],
    T![*],
    T![/],
    T![dslash],
    T![%],
    T![**],
    T![<<],
    T![>>],
    T![&],
    T![|],
    T![^],
    T![<],
    T![>],
    T![<=],
    T![>=],
    T![==],
    T![!=],
    T![in],
    T![not],
]);

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

/// A **unary expression**. In Starlark, **unary expressions** are
/// **expressions** that are prefixed with a **unary operator**.
///
/// Unary operators are `+`, `-`, `not`, and `~`.
///
/// See [`expression`] for more usage examples.
///
/// ## Ungrammar
///
/// ```
/// UnaryExpr = ('+' | '-' | 'not' | '~') Test
/// ```
///
/// ## Examples
///
/// ```starlark
/// -1
///
/// not x
/// ```
pub(super) fn unary_expr(p: &mut Parser) {
    let m = p.start();

    if p.at_ts(UNARY_OP) {
        p.bump_any();
        test(p);
    } else {
        // TODO: add diganostic and error recovery
        // p.error_recover("expected a unary operator", &[T![+], T![-], T![not], T![~]]);
    }

    m.complete(p, UNARY_EXPR);
}

// FIXME tmp
// let m = p.start();

//     test(p);
//     while p.at_ts(*BIN_OP) {
//         if p.at(T![in]) && !p.nth_at(1, T![not]) {
//             p.bump(T![in]);
//             // TODO: need to figure out how to deal with error handling and recovery here
//             // p.expect(T![not]);
//         } else {
//             p.bump_any();
//         }
//         p.bump_any();
//         test(p);
//     }

//     m.complete(p, BINARY_EXPR);

/// A **binary expression**. In Starlark, **binary expressions** are
/// **two expressions** separated by a **binary operator**.
/// Binary expressions are left-associative.
/// The precedence of binary operators is as follows:
///
/// 1. `*`, `/`, `%`, `//`
/// 2. `+`, `-`
/// 3. `<<`, `>>`
/// 4. `&`
/// 5. `^`
/// 6. `|`
///
/// See [`expression`] for more usage examples.
/// See [`bin_op`] for more information on binary operators.
/// // TODO: precedence table
/// See [`precedence`] for more information on operator precedence.
///
/// ## Ungrammar
///
/// ```
/// BinaryExpr = Test (BinOp Test)*
/// ```
///
/// ## Examples
///
/// ```starlark
/// 1 + 2
/// x * y
/// a and b
/// ```
pub(super) fn binary_expr(p: &mut Parser) {
    todo!("binary_expr")
}

/// A **binary operator**. In Starlark, **binary operators** are
/// `+`, `-`, `*`, `/`, `%`, `//`, `<<`, `>>`, `&`, `^`, `|`, `in`, `not in`.
///
/// See [`binary_expr`] for more information on binary expressions.
///
/// ## Ungrammar
///
/// ```
/// Binop =
///    'or'
///  | 'and'
///  | '==' | '!=' | '<' | '>' | '<=' | '>=' | 'in' | 'not' 'in'
///  | '|'
///  | '^'
///  | '&'
///  | '<<' | '>>'
///  | '-' | '+'
///  | '*' | '%' | '/' | '//'
/// ```
///
/// ## Examples
///
/// ```starlark
/// x + y
/// a * b
/// ```
pub(super) fn bin_op(p: &mut Parser) {
    todo!("bin_op")
}

/// A **lambda expression**. In Starlark, **lambda expressions** are
/// **anonymous functions**.
///
/// See [`statements::parameters`] and [`expr::test`] for
/// more information.
///
/// ## Ungrammar
///
/// ```
/// LambdaExpr = 'lambda' Parameters? ':' Test
/// ```
///
/// ## Examples
///
/// ```starlark
/// lambda x: x + 1
/// ```
pub(super) fn lambda_expr(p: &mut Parser) {
    let m = p.start();

    p.expect(T![lambda]);
    if p.at_ts(PARAM_START) {
        statements::parameters(p);
    }
    p.expect(T![:]);
    test(p);

    m.complete(p, LAMBDA_EXPR);
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
