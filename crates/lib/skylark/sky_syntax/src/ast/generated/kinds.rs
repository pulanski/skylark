//! Generated by `syntaxgen`, do not edit by hand.
//!
//! To regenerate this file, run `buck2 run //:syntaxgen`.
//!
//! Source files relevant to code generation for this file include:
//! `syntaxgen/kinds.rs`,
//! `syntaxgen/input.rs`,
//! `syntaxgen/sourcegen.rs`.
//!
//! Defines the `SyntaxKind` enumeration, which represents various syntax elements in the source code.
//! It includes literals, tokens, keywords, punctuation, and nodes. Each variant of the enumeration represents
//! a specific syntax element, which can be used to classify and process parts of the source code during
//! parsing and analysis.
//!
//! The `SyntaxKind` enumeration is used to represent the type of a syntax node in the AST (Abstract Syntax Tree)
//! and is a bridge between the lexer and the parser.

#![allow(bad_style, missing_docs, unreachable_pub)]
#[allow(clippy::manual_non_exhaustive, non_snake_case, non_camel_case_types)]
#[derive(
    Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, FromPrimitive, ToPrimitive, EnumCount,
)]
#[repr(u16)]
pub enum SyntaxKind {
    #[doc(hidden)]
    TOMBSTONE,
    #[doc(hidden)]
    EOF,
    #[doc = "Literals (e.g. IDENTIFIER, INT, FLOAT, STRING, BYTES)"]
    IDENTIFIER,
    INT,
    FLOAT,
    STRING,
    BYTES,
    #[doc = "Tokens (e.g. WHITESPACE, COMMENT, NEWLINE)"]
    WHITESPACE,
    COMMENT,
    NEWLINE,
    INDENT,
    OUTDENT,
    UNKNOWN,
    #[doc = "Keywords (e.g. BREAK, IN, LET, LOOP, etc.)"]
    AND_KW,
    ELSE_KW,
    LOAD_KW,
    BREAK_KW,
    FOR_KW,
    NOT_KW,
    CONTINUE_KW,
    IF_KW,
    OR_KW,
    DEF_KW,
    IN_KW,
    PASS_KW,
    ELIF_KW,
    LAMBDA_KW,
    RETURN_KW,
    #[doc = "Punctuation (e.g. DOT, COMMA, SEMICOLON, etc.)"]
    PLUS,
    MINUS,
    STAR,
    SLASH,
    DSLASH,
    PERCENT,
    DSTAR,
    TILDE,
    AMP,
    PIPE,
    CARET,
    LSHIFT,
    RSHIFT,
    DOT,
    COMMA,
    EQ,
    SEMICOLON,
    COLON,
    LPAREN,
    RPAREN,
    LBRACKET,
    RBRACKET,
    LBRACE,
    RBRACE,
    LT,
    GT,
    GE,
    LE,
    EQEQ,
    NE,
    PLUSEQ,
    MINUSEQ,
    STAREQ,
    SLASHEQ,
    PERCENTEQ,
    AMPEQ,
    PIPEEQ,
    CARETEQ,
    LSHIFTEQ,
    RSHIFTEQ,
    #[doc = "Nodes (e.g. FILE, MODULE, FUNCTION, etc.)"]
    FILE,
    STATEMENT,
    DEF_STMT,
    PARAMETERS,
    PARAMETER,
    IF_STMT,
    ELIF_CLAUSES,
    ELSE_CLAUSE,
    FOR_STMT,
    SUITE,
    SIMPLE_STMT,
    SMALL_STMT,
    RETURN_STMT,
    BREAK_STMT,
    CONTINUE_STMT,
    PASS_STMT,
    ASSIGN_STMT,
    EXPR_STMT,
    LOAD_STMT,
    TEST,
    BIN_OP,
    IF_EXPR,
    PRIMARY_EXPR,
    OPERAND,
    DOT_SUFFIX,
    SLICE_SUFFIX,
    CALL_SUFFIX,
    ARGUMENTS,
    ARGUMENT,
    LIST_EXPR,
    LIST_COMP,
    DICT_EXPR,
    DICT_COMP,
    ENTRIES,
    ENTRY,
    COMP_CLAUSE,
    UNARY_EXPR,
    BINARY_EXPR,
    BINOP,
    LAMBDA_EXPR,
    EXPRESSION,
    LOOP_VARIABLES,
    #[doc(hidden)]
    __LAST,
}
use self::SyntaxKind::*;
use crate::lexer::TokenKind;
use num_derive::{FromPrimitive, ToPrimitive};
use strum_macros::EnumCount;
impl SyntaxKind {
    pub fn is_keyword(self) -> bool {
        matches!(
            self,
            AND_KW
                | ELSE_KW
                | LOAD_KW
                | BREAK_KW
                | FOR_KW
                | NOT_KW
                | CONTINUE_KW
                | IF_KW
                | OR_KW
                | DEF_KW
                | IN_KW
                | PASS_KW
                | ELIF_KW
                | LAMBDA_KW
                | RETURN_KW
        )
    }
    pub fn is_punct(self) -> bool {
        matches!(
            self,
            PLUS | MINUS
                | STAR
                | SLASH
                | DSLASH
                | PERCENT
                | DSTAR
                | TILDE
                | AMP
                | PIPE
                | CARET
                | LSHIFT
                | RSHIFT
                | DOT
                | COMMA
                | EQ
                | SEMICOLON
                | COLON
                | LPAREN
                | RPAREN
                | LBRACKET
                | RBRACKET
                | LBRACE
                | RBRACE
                | LT
                | GT
                | GE
                | LE
                | EQEQ
                | NE
                | PLUSEQ
                | MINUSEQ
                | STAREQ
                | SLASHEQ
                | PERCENTEQ
                | AMPEQ
                | PIPEEQ
                | CARETEQ
                | LSHIFTEQ
                | RSHIFTEQ
        )
    }
    pub fn is_literal(self) -> bool {
        matches!(self, IDENTIFIER | INT | FLOAT | STRING | BYTES)
    }
    pub fn from_keyword(ident: &str) -> Option<SyntaxKind> {
        let kw = match ident {
            "and" => AND_KW,
            "else" => ELSE_KW,
            "load" => LOAD_KW,
            "break" => BREAK_KW,
            "for" => FOR_KW,
            "not" => NOT_KW,
            "continue" => CONTINUE_KW,
            "if" => IF_KW,
            "or" => OR_KW,
            "def" => DEF_KW,
            "in" => IN_KW,
            "pass" => PASS_KW,
            "elif" => ELIF_KW,
            "lambda" => LAMBDA_KW,
            "return" => RETURN_KW,
            _ => return None,
        };
        Some(kw)
    }
    pub fn from_contextual_keyword(ident: &str) -> Option<SyntaxKind> {
        let kw = match ident {
            _ => return None,
        };
        Some(kw)
    }
    pub fn from_char(c: char) -> Option<SyntaxKind> {
        let tok = match c {
            '+' => PLUS,
            '-' => MINUS,
            '*' => STAR,
            '/' => SLASH,
            '%' => PERCENT,
            '~' => TILDE,
            '&' => AMP,
            '|' => PIPE,
            '^' => CARET,
            '.' => DOT,
            ',' => COMMA,
            '=' => EQ,
            ';' => SEMICOLON,
            ':' => COLON,
            '(' => LPAREN,
            ')' => RPAREN,
            '[' => LBRACKET,
            ']' => RBRACKET,
            '{' => LBRACE,
            '}' => RBRACE,
            '<' => LT,
            '>' => GT,
            _ => return None,
        };
        Some(tok)
    }
}
#[doc = "Converts a `TokenKind` generated by the **Lexer** to a into a `SyntaxKind` for use via the parser and other tools."]
impl From<TokenKind> for SyntaxKind {
    fn from(kind: TokenKind) -> SyntaxKind {
        match kind {
            TokenKind::PLUS => PLUS,
            TokenKind::MINUS => MINUS,
            TokenKind::STAR => STAR,
            TokenKind::SLASH => SLASH,
            TokenKind::DSLASH => DSLASH,
            TokenKind::PERCENT => PERCENT,
            TokenKind::DSTAR => DSTAR,
            TokenKind::TILDE => TILDE,
            TokenKind::AMP => AMP,
            TokenKind::PIPE => PIPE,
            TokenKind::CARET => CARET,
            TokenKind::LSHIFT => LSHIFT,
            TokenKind::RSHIFT => RSHIFT,
            TokenKind::DOT => DOT,
            TokenKind::COMMA => COMMA,
            TokenKind::EQ => EQ,
            TokenKind::SEMICOLON => SEMICOLON,
            TokenKind::COLON => COLON,
            TokenKind::LPAREN => LPAREN,
            TokenKind::RPAREN => RPAREN,
            TokenKind::LBRACKET => LBRACKET,
            TokenKind::RBRACKET => RBRACKET,
            TokenKind::LBRACE => LBRACE,
            TokenKind::RBRACE => RBRACE,
            TokenKind::LT => LT,
            TokenKind::GT => GT,
            TokenKind::GE => GE,
            TokenKind::LE => LE,
            TokenKind::EQEQ => EQEQ,
            TokenKind::NE => NE,
            TokenKind::PLUSEQ => PLUSEQ,
            TokenKind::MINUSEQ => MINUSEQ,
            TokenKind::STAREQ => STAREQ,
            TokenKind::SLASHEQ => SLASHEQ,
            TokenKind::PERCENTEQ => PERCENTEQ,
            TokenKind::AMPEQ => AMPEQ,
            TokenKind::PIPEEQ => PIPEEQ,
            TokenKind::CARETEQ => CARETEQ,
            TokenKind::LSHIFTEQ => LSHIFTEQ,
            TokenKind::RSHIFTEQ => RSHIFTEQ,
            TokenKind::AND_KW => AND_KW,
            TokenKind::ELSE_KW => ELSE_KW,
            TokenKind::LOAD_KW => LOAD_KW,
            TokenKind::BREAK_KW => BREAK_KW,
            TokenKind::FOR_KW => FOR_KW,
            TokenKind::NOT_KW => NOT_KW,
            TokenKind::CONTINUE_KW => CONTINUE_KW,
            TokenKind::IF_KW => IF_KW,
            TokenKind::OR_KW => OR_KW,
            TokenKind::DEF_KW => DEF_KW,
            TokenKind::IN_KW => IN_KW,
            TokenKind::PASS_KW => PASS_KW,
            TokenKind::ELIF_KW => ELIF_KW,
            TokenKind::LAMBDA_KW => LAMBDA_KW,
            TokenKind::RETURN_KW => RETURN_KW,
            TokenKind::IDENTIFIER => IDENTIFIER,
            TokenKind::INT => INT,
            TokenKind::FLOAT => FLOAT,
            TokenKind::STRING => STRING,
            TokenKind::BYTES => BYTES,
            TokenKind::NEWLINE => NEWLINE,
            TokenKind::INDENT => INDENT,
            TokenKind::OUTDENT => OUTDENT,
            TokenKind::NumericLiteral => INT,
            TokenKind::COMMENT => COMMENT,
            TokenKind::WHITESPACE => WHITESPACE,
            TokenKind::UNKNOWN => UNKNOWN,
            TokenKind::EOF => EOF,
            TokenKind::DSLASHEQ => todo!(),
            // TokenKind::DSLASHEQ => DSLASHEQ,
        }
    }
}
#[macro_export]
macro_rules ! T { [+] => { $ crate :: SyntaxKind :: PLUS } ; [-] => { $ crate :: SyntaxKind :: MINUS } ; [*] => { $ crate :: SyntaxKind :: STAR } ; [/] => { $ crate :: SyntaxKind :: SLASH } ; [dslash] => { $ crate :: SyntaxKind :: DSLASH } ; [%] => { $ crate :: SyntaxKind :: PERCENT } ; [**] => { $ crate :: SyntaxKind :: DSTAR } ; [~] => { $ crate :: SyntaxKind :: TILDE } ; [&] => { $ crate :: SyntaxKind :: AMP } ; [|] => { $ crate :: SyntaxKind :: PIPE } ; [^] => { $ crate :: SyntaxKind :: CARET } ; [<<] => { $ crate :: SyntaxKind :: LSHIFT } ; [>>] => { $ crate :: SyntaxKind :: RSHIFT } ; [.] => { $ crate :: SyntaxKind :: DOT } ; [,] => { $ crate :: SyntaxKind :: COMMA } ; [=] => { $ crate :: SyntaxKind :: EQ } ; [;] => { $ crate :: SyntaxKind :: SEMICOLON } ; [:] => { $ crate :: SyntaxKind :: COLON } ; ['('] => { $ crate :: SyntaxKind :: LPAREN } ; [')'] => { $ crate :: SyntaxKind :: RPAREN } ; ['['] => { $ crate :: SyntaxKind :: LBRACKET } ; [']'] => { $ crate :: SyntaxKind :: RBRACKET } ; ['{'] => { $ crate :: SyntaxKind :: LBRACE } ; ['}'] => { $ crate :: SyntaxKind :: RBRACE } ; [<] => { $ crate :: SyntaxKind :: LT } ; [>] => { $ crate :: SyntaxKind :: GT } ; [>=] => { $ crate :: SyntaxKind :: GE } ; [<=] => { $ crate :: SyntaxKind :: LE } ; [==] => { $ crate :: SyntaxKind :: EQEQ } ; [!=] => { $ crate :: SyntaxKind :: NE } ; [+=] => { $ crate :: SyntaxKind :: PLUSEQ } ; [-=] => { $ crate :: SyntaxKind :: MINUSEQ } ; [*=] => { $ crate :: SyntaxKind :: STAREQ } ; [/=] => { $ crate :: SyntaxKind :: SLASHEQ } ; [%=] => { $ crate :: SyntaxKind :: PERCENTEQ } ; [&=] => { $ crate :: SyntaxKind :: AMPEQ } ; [|=] => { $ crate :: SyntaxKind :: PIPEEQ } ; [^=] => { $ crate :: SyntaxKind :: CARETEQ } ; [<<=] => { $ crate :: SyntaxKind :: LSHIFTEQ } ; [>>=] => { $ crate :: SyntaxKind :: RSHIFTEQ } ; [and] => { $ crate :: SyntaxKind :: AND_KW } ; [else] => { $ crate :: SyntaxKind :: ELSE_KW } ; [load] => { $ crate :: SyntaxKind :: LOAD_KW } ; [break] => { $ crate :: SyntaxKind :: BREAK_KW } ; [for] => { $ crate :: SyntaxKind :: FOR_KW } ; [not] => { $ crate :: SyntaxKind :: NOT_KW } ; [continue] => { $ crate :: SyntaxKind :: CONTINUE_KW } ; [if] => { $ crate :: SyntaxKind :: IF_KW } ; [or] => { $ crate :: SyntaxKind :: OR_KW } ; [def] => { $ crate :: SyntaxKind :: DEF_KW } ; [in] => { $ crate :: SyntaxKind :: IN_KW } ; [pass] => { $ crate :: SyntaxKind :: PASS_KW } ; [elif] => { $ crate :: SyntaxKind :: ELIF_KW } ; [lambda] => { $ crate :: SyntaxKind :: LAMBDA_KW } ; [return] => { $ crate :: SyntaxKind :: RETURN_KW } ; [identifier] => { $ crate :: SyntaxKind :: IDENTIFIER } ; [eof] => { $ crate :: SyntaxKind :: EOF } ; [newline] => { $ crate :: SyntaxKind :: NEWLINE } ; [indent] => { $ crate :: SyntaxKind :: INDENT } ; [outdent] => { $ crate :: SyntaxKind :: OUTDENT } ; [dslash] => { $ crate :: SyntaxKind :: DSLASH } ; [string] => { $ crate :: SyntaxKind :: STRING } ; [bytes] => { $ crate :: SyntaxKind :: BYTES } ; [int] => { $ crate :: SyntaxKind :: INT } ; [float] => { $ crate :: SyntaxKind :: FLOAT } ; [comment] => { $ crate :: SyntaxKind :: COMMENT } ; [whitespace] => { $ crate :: SyntaxKind :: WHITESPACE } ; [error] => { $ crate :: SyntaxKind :: ERROR } ; }
pub use T;
