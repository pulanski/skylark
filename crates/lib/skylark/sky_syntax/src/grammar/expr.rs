use crate::{parser::Parser, T};

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
    
}

/// A **test**. In Starlark, **tests** are **syntax nodes** that represent
/// the **basis** of **expressions**.
///
/// See [`expr::expression`] for more information.
/// See [`expr::test`] for more information.
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
pub(super) fn test(p: &mut Parser) {}
