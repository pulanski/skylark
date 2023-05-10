use super::{
    comprehension::{dict_comp, dict_expr, list_comp},
    statements::{self, PARAM_START},
};
use crate::{
    lexer::{Span, Token},
    parser::{ParseError, Parser},
    SyntaxKind::*,
    TokenSet, T,
};

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
#[tracing::instrument(level = "trace", skip(p))]
pub(super) fn expression(p: &mut Parser) {
    tracing::trace!("Parsing expression");
    let m = p.start();

    test(p);
    while p.eat(T![,]) {
        test(p);
    }

    tracing::trace!("Completed expression");
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
#[tracing::instrument(level = "debug", skip(p))]
pub(super) fn test(p: &mut Parser) {
    let m = p.start();
    tracing::debug!("Parsing test. Current token: {:?}", p.current());

    if p.at(T![if]) {
        if_expr(p);
    } else if p.at(T![lambda]) {
        lambda_expr(p);
    } else if p.at_ts(UNARY_OP) {
        unary_expr(p);
    } else {
        primary_expr(p);
    }

    // TODO: Add support for binary expressions

    tracing::debug!("Completed test");
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
/// See [`expression`], [`list_expr`], [`comprehension::list_comp`], [`comprehension::dict_expr`],
/// and [`comprehension::dict_comp`] for more information.
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
#[tracing::instrument(level = "debug", skip(p))]
pub(super) fn operand(p: &mut Parser) {
    let m = p.start();
    tracing::debug!("Parsing operand. Current token: {:?}", p.current());

    match p.current() {
        T![identifier] | T![int] | T![float] | T![string] | T![bytes] => {
            tracing::debug!("Parsing literal. Literal token: {:?}", p.current());
            p.bump_any()
        }
        T!['('] => {
            p.bump(T!['(']);
            tracing::debug!(
                "Parsing parenthesized expression. Current token: {:?}",
                p.current()
            );
            if !p.at(T![')']) {
                expression(p);
                p.eat(T![,]);
            }
            p.expect(T![')']);
        }
        T!['['] => {
            tracing::debug!(
                "Parsing list expression or comprehension. Current token: {:?}",
                p.current()
            );
            if p.nth_at(1, T![for]) {
                tracing::debug!("Parsing list comprehension");
                list_comp(p);
            } else {
                tracing::debug!("Parsing list expression");
                list_expr(p);
            }
        }
        T!['{'] => {
            if p.nth_at(1, T![for]) {
                tracing::debug!("Parsing dict comprehension");
                dict_comp(p);
            } else {
                tracing::debug!("Parsing dict expression");
                dict_expr(p);
            }
        }
        _ => {
            p.error(ParseError::UnexpectedToken {
                expected: TokenSet::new(&[
                    T![identifier],
                    T![int],
                    T![float],
                    T![string],
                    T![bytes],
                    T!['('],
                    T!['['],
                    T!['{'],
                ]),
                found: Token::new(
                    p.current().tk(),
                    String::from("expected an operand"),
                    Span::new(0, 0),
                    // p.current().text.clone(),
                    // p.current().span
                ),
            });
            // p.err_recover("operand error", |p| {
            //     p.at(T![identifier])
            //         || p.at(T![int])
            //         || p.at(T![float])
            //         || p.at(T![string])
            //         || p.at(T![bytes])
            //         || p.at(T!['('])
            //         || p.at(T!['['])
            //         || p.at(T!['{'])
            // });
        }
    }

    tracing::debug!("Finished parsing operand");
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
#[tracing::instrument(level = "debug", skip(p))]
pub(super) fn dot_suffix(p: &mut Parser) {
    let m = p.start();
    tracing::debug!("Parsing dot suffix");

    p.expect(T![.]);
    p.expect(T![identifier]);

    tracing::debug!("Finished parsing dot suffix");
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
#[tracing::instrument(level = "debug", skip(p))]
pub(super) fn slice_suffix(p: &mut Parser) {
    let m = p.start();
    tracing::debug!("Parsing slice suffix");

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

    tracing::debug!("Finished parsing slice suffix");
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
#[tracing::instrument(level = "debug", skip(p))]
pub(super) fn call_suffix(p: &mut Parser) {
    let m = p.start();
    tracing::debug!("Parsing call suffix");

    p.expect(T!['(']);
    if !p.at(T![')']) {
        arguments(p);
        p.eat(T![,]);
    }
    p.expect(T![')']);

    tracing::debug!("Finished parsing call suffix");
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
#[tracing::instrument(level = "debug", skip(p))]
pub(super) fn arguments(p: &mut Parser) {
    let m = p.start();
    tracing::debug!("Parsing arguments");

    argument(p);
    while p.eat(T![,]) {
        argument(p);
    }

    tracing::debug!("Finished parsing arguments");
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
#[tracing::instrument(level = "debug", skip(p))]
pub(super) fn argument(p: &mut Parser) {
    let m = p.start();
    tracing::debug!("Parsing argument");

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

    tracing::debug!("Finished parsing argument");
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
#[tracing::instrument(level = "debug", skip(p))]
pub(super) fn list_expr(p: &mut Parser) {
    let m = p.start();
    tracing::debug!("Parsing list expression");

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

    tracing::debug!("Finished parsing list expression");
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
#[tracing::instrument(level = "debug", skip(p))]
pub(super) fn unary_expr(p: &mut Parser) {
    let m = p.start();
    tracing::debug!("Parsing unary_expr");

    if p.at_ts(UNARY_OP) {
        p.bump_any();
        test(p);
    } else {
        // TODO: add diganostic and error recovery
        // p.error("expected a unary operator");
        // p.err_recover("unary operator error", |p| {
        //     p.at(T![+]) || p.at(T![-]) || p.at(T![not]) || p.at(T![~])
        // });
    }

    tracing::debug!("Finished parsing unary_expr");
    m.complete(p, UNARY_EXPR);
}

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
#[tracing::instrument(level = "debug", skip(p))]
pub(super) fn binary_expr(p: &mut Parser) {
    let m = p.start();
    tracing::debug!("Parsing binary_expr");

    test(p);

    while p.at_ts(BIN_OP) || (p.at(T![not]) && p.nth_at(1, T![in])) {
        bin_op(p);
        test(p);
    }

    tracing::debug!("Finished parsing binary_expr");
    m.complete(p, BINARY_EXPR);
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
#[tracing::instrument(level = "debug", skip(p))]
pub(super) fn bin_op(p: &mut Parser) {
    tracing::debug!("Parsing bin_op");

    if p.at_ts(BIN_OP) {
        p.bump_any();
    } else if p.at(T![not]) && p.nth_at(1, T![in]) {
        p.bump(T![not]);
        p.bump(T![in]);
    } else {
        // TODO: add diganostic and error recovery
        // p.error("expected a binary operator");
        // p.err_recover("binary operator error", |p| {
        //     p.at_ts(&BINARY_OPERATORS) || (p.at(T![not]) && p.nth_at(1, T![in]))
        // });
    }

    tracing::debug!("Finished parsing bin_op");
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
#[tracing::instrument(level = "debug", skip(p))]
pub(super) fn lambda_expr(p: &mut Parser) {
    let m = p.start();
    tracing::debug!("Parsing lambda_expr");

    p.expect(T![lambda]);
    if p.at_ts(PARAM_START) {
        statements::parameters(p);
    }
    p.expect(T![:]);
    test(p);

    tracing::debug!("Finished parsing lambda_expr");
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
#[tracing::instrument(level = "debug", skip(p))]
pub(super) fn if_expr(p: &mut Parser) {
    let m = p.start();
    tracing::debug!("Parsing if_expr");

    test(p);
    p.expect(T![if]);
    test(p);
    p.expect(T![else]);
    test(p);

    tracing::debug!("Finished parsing if_expr");
    m.complete(p, IF_EXPR);
}
