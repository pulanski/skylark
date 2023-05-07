use crate::{
    grammar::decl,
    lexer::{Span, Token},
    parser::{CompletedMarker, Marker, ParseError, Parser},
    SyntaxKind::*,
    TokenSet, T,
};

use super::expr;

pub(super) const STATEMENT_RECOVERY_SET: TokenSet =
    TokenSet::new(&[T![def], T![load], T![if], T![for], T![in], T![return]]);

pub(super) const ASSIGNMENT_OPERATOR: TokenSet = TokenSet::new(&[
    T![=],
    T![+=],
    T![-=],
    T![*=],
    T![/=],
    // T![slash_slash_eq], TODO: add support for operator
    T![%=],
    T![&=],
    T![|=],
    T![^=],
    T![<<=],
    T![>>=],
]);

/// A **statement** in a **Starlark file**. Statements are **syntax
/// nodes** that represent the **basis** of **Starlark files**. They
/// can **declare functions**, **define variables**, and more (_see below_).
///
/// ## Ungrammar
///
/// ```
/// Statement =
///    DefStmt
///  | IfStmt
///  | ForStmt
///  | SimpleStmt
/// ```
///
/// ## Examples
///
/// ```starlark
/// def area(x, y):
///    return x * y
/// ```
pub(super) fn statement(p: &mut Parser) {
    let m = p.start();

    match p.current() {
        T![def] => def_stmt(p),
        T![if] => if_stmt(p),
        T![for] => for_stmt(p),
        _ => simple_stmt(p),
    };

    m.complete(p, STATEMENT);

    // Some(m.complete(p, STATEMENT))
    // p.accept
}

/// A **function definition statement**.
///
/// ## Ungrammar
///
/// ```
/// DefStmt =
/// 'def' 'identifier' '('
///   (Parameters (',')?)?
/// ')' ':' Suite
/// ```
///
/// ## Examples
///
/// ```starlark
/// def foo(x):
///     return x * 2
/// ```
pub(super) fn def_stmt(p: &mut Parser) {
    assert!(p.at(T![def])); // precondition (enforced by caller)
    let m = p.start();

    p.bump(T![def]);
    p.expect(T![identifier]);
    p.expect(T!['(']);
    if !p.at(T![')']) {
        parameters(p);
        p.eat(T![,]);
    }
    p.expect(T![')']);
    p.expect(T![:]);
    decl::suite(p);

    m.complete(p, DEF_STMT);
}

/// A **comma-separated list** of **function parameters**.
///
/// ## Ungrammar
///
/// ```
/// Parameters = Parameter (',' Parameter)*
/// ```
///
/// ## Examples
///
/// `x`, `y`, `z` in
///
/// ```starlark
/// def sum(x, y, z):
///    return x + y + z
/// ```
fn parameters(p: &mut Parser) {
    let m = p.start();

    parameter(p);
    while p.eat(T![,]) {
        parameter(p);
    }
    m.complete(p, PARAMETERS);
}

/// A **function parameter**.
///
/// ## Ungrammar
///
/// ```
/// Parameter = 'identifier'
///   | ('identifier' '=' Test)
///  | '*'
///  | ('*' 'identifier')
///  | ('**' 'identifier')
/// ```
///
/// ## Examples
///
/// `x`, `y`, `z` in
///
/// ```starlark
/// def sum(x, y, z):
///   return x + y + z
/// ```
///
/// or `*args` in
///
/// ```starlark
/// def sum(*args):
///  return sum(args)
/// ```
pub(super) fn parameter(p: &mut Parser) {
    let m = p.start();
    match p.current() {
        T![identifier] => {
            p.bump(T![identifier]);
            if p.eat(T![=]) {
                expr::test(p);
            }
        }
        T![*] => {
            p.bump(T![*]);
            if p.at(T![identifier]) {
                p.bump(T![identifier]);
            }
        }
        T![**] => {
            p.bump(T![**]);
            p.expect(T![identifier]);
        }
        _ => p.error(ParseError::UnexpectedToken {
            expected: TokenSet::from(vec![T![identifier].tk()]),
            found: Token::new(
                p.current().tk(),
                String::from("TODO: expected parameter"),
                Span::default(),
                // p.current().text().to_string(),
                // p.current().span(),
            ),
        }),
        // _ => p.error("expected parameter"),
    }
    m.complete(p, PARAMETER);
}

/// An **elif clause**. Elif clauses are used to **conditionally execute
/// statements** within an **if statement**.
///
/// ## Ungrammar
///
/// ```
/// ElifClause = 'elif' Test ':' Suite
/// ```
///
/// ## Examples
///
/// `elif x < 0:
///    print("x is negative")` in
///
/// ```starlark
/// if x > 0:
///     print("x is positive")
/// elif x < 0:
///     print("x is negative")
/// else:
///     print("x is zero")
/// ```
pub(super) fn elif_clauses(p: &mut Parser) {
    assert!(p.at(T![elif])); // precondition (enforced by caller)
    let m = p.start();

    p.bump(T![elif]);
    expr::test(p);
    p.expect(T![:]);
    decl::suite(p);

    m.complete(p, ELIF_CLAUSES);
}

/// An **else clause**. Else clauses are used to **conditionally execute
/// statements** within an **if statement**.
///
/// ## Ungrammar
///
/// ```
/// ElseClause = 'else' ':' Suite
/// ```
///
/// ## Examples
///
/// `else:
///   print("x is zero")` in
///
/// ```starlark
/// if x > 0:
///     print("x is positive")
/// elif x < 0:
///     print("x is negative")
/// else:
///     print("x is zero")
/// ```
pub(super) fn else_clause(p: &mut Parser) {
    assert!(p.at(T![else])); // precondition (enforced by caller)
    let m = p.start();

    p.bump(T![else]);
    p.expect(T![:]);
    decl::suite(p);

    m.complete(p, ELSE_CLAUSE);
}

/// A **for statement**. For statements are used to **iterate over a
/// sequence**.
///
/// ## Ungrammar
///
/// ```
/// ForStmt = 'for' LoopVariables 'in' Expression ':' Suite
/// ```
///
/// ## Examples
///
/// ```starlark
/// # Iterate over a list
/// for x in range(10):
///     print(x)
/// ```
pub(super) fn for_stmt(p: &mut Parser) {
    assert!(p.at(T![for])); // precondition (enforced by caller)
    let m = p.start();

    p.bump(T![for]);
    decl::loop_variables(p);
    p.expect(T![in]);
    expr::expression(p);
    p.expect(T![:]);
    decl::suite(p);

    m.complete(p, FOR_STMT);
}

/// A **simple statement**, which can be executed on a single line.
///
/// ## Ungrammar
///
/// ```
/// SimpleStmt = SmallStmt (';' SmallStmt)* ';'?
/// ```
///
/// ## Examples
///
/// ```starlark
/// x = 1 # a simple statement
/// return x # another simple statement
/// ```
pub(super) fn simple_stmt(p: &mut Parser) {
    // TODO: may need to fix
    let m = p.start();

    small_stmt(p);
    while p.eat(T![;]) {
        if p.at(T![newline]) || p.at(EOF) {
            break;
        }
        small_stmt(p);
    }

    m.complete(p, SIMPLE_STMT);
}

/// A **small statement**. A small statement is a statement that **does
/// not** contain other statements. This includes:
///
/// - [`return_stmt`]
/// - [`break_stmt`]
/// - [`continue_stmt`]
/// - [`pass_stmt`]
/// - [`assign_stmt`]
/// - [`expr_stmt`]
/// - [`load_stmt`]
///
/// ## Ungrammar
///
/// ```
/// SmallStmt =
///  ReturnStmt
/// | BreakStmt
/// | ContinueStmt
/// | PassStmt
/// | AssignStmt
/// | ExprStmt
/// | LoadStmt
/// ```
///
/// ## Examples
///
/// ```starlark
/// return x
/// break
/// continue
/// pass
/// x = 1
/// ```
pub(super) fn small_stmt(p: &mut Parser) {
    let m = p.start();

    match p.current() {
        T![return] => {
            return_stmt(p);
        }
        T![break] => {
            break_stmt(p);
        }
        T![continue] => {
            continue_stmt(p);
        }
        T![pass] => {
            pass_stmt(p);
        }
        _ => {
            if p.at(T![load]) {
                load_stmt(p);
            } else if p.at(T![identifier]) && p.nth_at(1, T![=]) {
                assign_stmt(p);
            } else {
                expr_stmt(p);
            }
        }
    }

    m.complete(p, SMALL_STMT);
}

/// A **return statement**, which can include an **expression**.
///
/// See [`expr::expression`] for more information.
///
/// ## Ungrammar
///
/// ```
/// ReturnStmt = 'return' Expression?
/// ```
///
/// ## Examples
///
/// ```starlark
/// return x
/// return
/// ```
pub(super) fn return_stmt(p: &mut Parser) {
    assert!(p.at(T![return])); // precondition (enforced by caller)
    let m = p.start();

    p.bump(T![return]);
    if !p.at(T![newline]) {
        expr::expression(p);
    }

    m.complete(p, RETURN_STMT);
}

/// A **break statement**. Break statements are **used to exit loops**.
/// They can **only** be used _inside a loop_.
///
/// ## Ungrammar
///
/// ```
/// BreakStmt = 'break'
/// ```
///
/// ## Examples
///
/// ```starlark
/// for x in range(10):
///    if x == 5:
///      break
///   print(x)
/// ```
fn break_stmt(p: &mut Parser) {
    assert!(p.at(T![break])); // precondition (enforced by caller)
    let m = p.start();

    p.bump(T![break]);

    m.complete(p, BREAK_STMT);
}

/// A **continue statement**. Continue statements are used to **skip to the
/// next iteration** of a loop. They can **only** be used _inside a loop_.
///
/// ## Ungrammar
///
/// ```
/// ContinueStmt = 'continue'
/// ```
///
/// ## Examples
///
/// ```starlark
/// for x in range(10):
///  if x == 5:
///   continue
/// print(x)
/// ```
fn continue_stmt(p: &mut Parser) -> CompletedMarker {
    assert!(p.at(T![continue])); // precondition (enforced by caller)
    let m = p.start();

    p.bump(T![continue]);

    m.complete(p, CONTINUE_STMT)
}

/// A **pass statement**. Pass statements are used to **do nothing**.
/// They are **useful as placeholders** when a statement is required
/// syntactically, but no code needs to be executed.
/// They can be used _anywhere_.
///
/// ## Ungrammar
///
/// ```
/// PassStmt = 'pass'
/// ```
///
/// ## Examples
///
/// ```starlark
/// if x > 0:
///     pass
/// else:
///     print(x)
/// ```
fn pass_stmt(p: &mut Parser) {
    assert!(p.at(T![pass])); // precondition (enforced by caller)
    let m = p.start();

    p.bump(T![pass]);

    m.complete(p, PASS_STMT);
}

/// An **assignment statement**. An assignment statement is a statement
/// that assigns a value to a variable or other target usign an **assignment
/// operator**.
///
/// ## Ungrammar
///
/// ```
/// AssignStmt = Expression ('=' | '+=' | '-=' | '*=' | '/=' | '//=' | '%=' | '&=' | '|=' | '^=' | '<<=' | '>>=') Expression
/// ```
fn assign_stmt(p: &mut Parser) -> CompletedMarker {
    let m = p.start();

    expr::expression(p);
    if !p.at_ts(ASSIGNMENT_OPERATOR) {
        p.error(ParseError::UnexpectedToken {
            expected: ASSIGNMENT_OPERATOR,
            found: Token::new(
                p.current().tk(),
                String::from("TODO: assignment operator"),
                Span::new(0, 0),
                // p.current().text().to_owned(),
                // p.current().span(), TODO: refactor to use this for better error messages
            ),
        });
    } else {
        assert!(p.at_ts(ASSIGNMENT_OPERATOR));
        p.bump_any();
    }
    expr::expression(p);

    m.complete(p, ASSIGN_STMT)
}

/// An **expression statement**. An expression statement is a statement
/// that evaluates an [`expr::expression`] and discards the result. This is
/// typically used for function calls and assignments.
///
/// ## Ungrammar
///
/// ```
/// ExprStmt = Expression
/// ```
///
/// ## Examples
///
/// ```starlark
/// x = 1 # an expression statement
/// foo() # another expression statement
/// ```
fn expr_stmt(p: &mut Parser) -> CompletedMarker {
    let m = p.start();

    expr::expression(p);

    m.complete(p, EXPR_STMT)
}

/// A **load statement**. Load statements are used to **import** other
/// Starlark files. Typically this means bringing other **symbols** (e.g.
/// _functions_, _variables_, etc.) into the scope of the current file.
///
/// ## Ungrammar
///
/// ```
/// LoadStmt = 'load' '(' 'string' (',' ('identifier' '=')? 'string')* ','? ')'
/// ```
///
/// ## Examples
///
/// ```starlark
/// load("@bazel_tools//tools/build_defs/repo:git.bzl",
/// _git_repository = "git_repository") # usage with alias
/// load("@stdlib//strings.bzl", "strncpy") # regular usage
/// ```
fn load_stmt(p: &mut Parser) {
    let m = p.start();

    p.bump(T![load]);
    p.expect(T!['(']);
    p.expect(T![string]);
    while p.eat(T![,]) {
        if p.at(T![identifier]) {
            p.bump(T![identifier]);
            p.expect(T![=]);
        }
        p.expect(T![string]);
    }
    p.eat(T![,]);
    p.expect(T![')']);

    m.complete(p, LOAD_STMT);
}

/// An **if statement**. If statements are used to **conditionally execute
/// statements**. They can **optionally** include **else clauses** and
/// **elif clauses**.
///
/// ## Ungrammar
///
/// ```
/// IfStmt =
///  'if' Test ':' Suite
/// ElifClauses*
///  ElseClause?
/// ```
///
/// ## Examples
///
/// ```starlark
/// if x > 0:
///     print("x is positive")
/// elif x < 0:
///     print("x is negative")
/// else:
///     print("x is zero")
/// ```
pub(super) fn if_stmt(p: &mut Parser) {
    assert!(p.at(T![if])); // precondition (enforced by caller)
    let m = p.start();

    p.bump(T![if]);
    expr::test(p);
    p.expect(T![:]);
    decl::suite(p);
    while p.at(T![elif]) {
        p.bump(T![elif]);
        expr::test(p);
        p.expect(T![:]);
        decl::suite(p);
    }
    if p.at(T![else]) {
        p.bump(T![else]);
        p.expect(T![:]);
        decl::suite(p);
    }

    m.complete(p, IF_STMT);
}
