//! Generated by `syntaxgen`, do not edit by hand.
//!
//! To regenerate this file, run `buck2 run //:syntaxgen`.
//!
//! Source files relevant to code generation for this file include:
//! `syntaxgen/tokens.rs`,
//! `syntaxgen/input.rs`,
//! `syntaxgen/sourcegen.rs`.
//!
/// A syntax token.

use crate::{
    ast::AstToken,
    SyntaxKind::{self, *},
    SyntaxToken,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Whitespace {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Whitespace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Whitespace {
    fn can_cast(kind: SyntaxKind) -> bool { kind == WHITESPACE }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Comment {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Comment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Comment {
    fn can_cast(kind: SyntaxKind) -> bool { kind == COMMENT }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Newline {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Newline {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Newline {
    fn can_cast(kind: SyntaxKind) -> bool { kind == NEWLINE }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Indent {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Indent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Indent {
    fn can_cast(kind: SyntaxKind) -> bool { kind == INDENT }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Outdent {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Outdent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Outdent {
    fn can_cast(kind: SyntaxKind) -> bool { kind == OUTDENT }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Unknown {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Unknown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Unknown {
    fn can_cast(kind: SyntaxKind) -> bool { kind == UNKNOWN }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Identifier {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Identifier {
    fn can_cast(kind: SyntaxKind) -> bool { kind == IDENTIFIER }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Int {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Int {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Int {
    fn can_cast(kind: SyntaxKind) -> bool { kind == INT }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Float {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Float {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Float {
    fn can_cast(kind: SyntaxKind) -> bool { kind == FLOAT }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct String {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for String {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for String {
    fn can_cast(kind: SyntaxKind) -> bool { kind == STRING }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Bytes {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Bytes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Bytes {
    fn can_cast(kind: SyntaxKind) -> bool { kind == BYTES }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct And {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for And {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for And {
    fn can_cast(kind: SyntaxKind) -> bool { kind == AND_KW }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Else {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Else {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Else {
    fn can_cast(kind: SyntaxKind) -> bool { kind == ELSE_KW }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Load {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Load {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Load {
    fn can_cast(kind: SyntaxKind) -> bool { kind == LOAD_KW }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Break {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Break {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Break {
    fn can_cast(kind: SyntaxKind) -> bool { kind == BREAK_KW }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct For {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for For {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for For {
    fn can_cast(kind: SyntaxKind) -> bool { kind == FOR_KW }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Not {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Not {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Not {
    fn can_cast(kind: SyntaxKind) -> bool { kind == NOT_KW }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Continue {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Continue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Continue {
    fn can_cast(kind: SyntaxKind) -> bool { kind == CONTINUE_KW }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct If {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for If {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for If {
    fn can_cast(kind: SyntaxKind) -> bool { kind == IF_KW }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Or {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Or {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Or {
    fn can_cast(kind: SyntaxKind) -> bool { kind == OR_KW }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Def {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Def {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Def {
    fn can_cast(kind: SyntaxKind) -> bool { kind == DEF_KW }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct In {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for In {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for In {
    fn can_cast(kind: SyntaxKind) -> bool { kind == IN_KW }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pass {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Pass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Pass {
    fn can_cast(kind: SyntaxKind) -> bool { kind == PASS_KW }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Elif {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Elif {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Elif {
    fn can_cast(kind: SyntaxKind) -> bool { kind == ELIF_KW }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lambda {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Lambda {
    fn can_cast(kind: SyntaxKind) -> bool { kind == LAMBDA_KW }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Return {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Return {
    fn can_cast(kind: SyntaxKind) -> bool { kind == RETURN_KW }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Plus {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Plus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Plus {
    fn can_cast(kind: SyntaxKind) -> bool { kind == PLUS }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Minus {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Minus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Minus {
    fn can_cast(kind: SyntaxKind) -> bool { kind == MINUS }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Star {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Star {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Star {
    fn can_cast(kind: SyntaxKind) -> bool { kind == STAR }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Slash {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Slash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Slash {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SLASH }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dslash {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Dslash {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Dslash {
    fn can_cast(kind: SyntaxKind) -> bool { kind == DSLASH }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Percent {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Percent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Percent {
    fn can_cast(kind: SyntaxKind) -> bool { kind == PERCENT }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dstar {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Dstar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Dstar {
    fn can_cast(kind: SyntaxKind) -> bool { kind == DSTAR }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Tilde {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Tilde {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Tilde {
    fn can_cast(kind: SyntaxKind) -> bool { kind == TILDE }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Amp {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Amp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Amp {
    fn can_cast(kind: SyntaxKind) -> bool { kind == AMP }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pipe {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Pipe {
    fn can_cast(kind: SyntaxKind) -> bool { kind == PIPE }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Caret {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Caret {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Caret {
    fn can_cast(kind: SyntaxKind) -> bool { kind == CARET }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lshift {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Lshift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Lshift {
    fn can_cast(kind: SyntaxKind) -> bool { kind == LSHIFT }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rshift {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Rshift {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Rshift {
    fn can_cast(kind: SyntaxKind) -> bool { kind == RSHIFT }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Dot {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Dot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Dot {
    fn can_cast(kind: SyntaxKind) -> bool { kind == DOT }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Comma {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Comma {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Comma {
    fn can_cast(kind: SyntaxKind) -> bool { kind == COMMA }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Eq {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Eq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Eq {
    fn can_cast(kind: SyntaxKind) -> bool { kind == EQ }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Semicolon {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Semicolon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Semicolon {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SEMICOLON }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Colon {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Colon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Colon {
    fn can_cast(kind: SyntaxKind) -> bool { kind == COLON }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lparen {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Lparen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Lparen {
    fn can_cast(kind: SyntaxKind) -> bool { kind == LPAREN }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rparen {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Rparen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Rparen {
    fn can_cast(kind: SyntaxKind) -> bool { kind == RPAREN }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lbracket {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Lbracket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Lbracket {
    fn can_cast(kind: SyntaxKind) -> bool { kind == LBRACKET }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rbracket {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Rbracket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Rbracket {
    fn can_cast(kind: SyntaxKind) -> bool { kind == RBRACKET }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lbrace {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Lbrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Lbrace {
    fn can_cast(kind: SyntaxKind) -> bool { kind == LBRACE }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rbrace {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Rbrace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Rbrace {
    fn can_cast(kind: SyntaxKind) -> bool { kind == RBRACE }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lt {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Lt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Lt {
    fn can_cast(kind: SyntaxKind) -> bool { kind == LT }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Gt {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Gt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Gt {
    fn can_cast(kind: SyntaxKind) -> bool { kind == GT }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ge {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Ge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Ge {
    fn can_cast(kind: SyntaxKind) -> bool { kind == GE }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Le {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Le {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Le {
    fn can_cast(kind: SyntaxKind) -> bool { kind == LE }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Eqeq {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Eqeq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Eqeq {
    fn can_cast(kind: SyntaxKind) -> bool { kind == EQEQ }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ne {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Ne {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Ne {
    fn can_cast(kind: SyntaxKind) -> bool { kind == NE }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pluseq {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Pluseq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Pluseq {
    fn can_cast(kind: SyntaxKind) -> bool { kind == PLUSEQ }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Minuseq {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Minuseq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Minuseq {
    fn can_cast(kind: SyntaxKind) -> bool { kind == MINUSEQ }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Stareq {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Stareq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Stareq {
    fn can_cast(kind: SyntaxKind) -> bool { kind == STAREQ }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Slasheq {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Slasheq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Slasheq {
    fn can_cast(kind: SyntaxKind) -> bool { kind == SLASHEQ }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Percenteq {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Percenteq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Percenteq {
    fn can_cast(kind: SyntaxKind) -> bool { kind == PERCENTEQ }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Ampeq {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Ampeq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Ampeq {
    fn can_cast(kind: SyntaxKind) -> bool { kind == AMPEQ }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pipeeq {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Pipeeq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Pipeeq {
    fn can_cast(kind: SyntaxKind) -> bool { kind == PIPEEQ }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Careteq {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Careteq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Careteq {
    fn can_cast(kind: SyntaxKind) -> bool { kind == CARETEQ }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Lshifteq {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Lshifteq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Lshifteq {
    fn can_cast(kind: SyntaxKind) -> bool { kind == LSHIFTEQ }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Rshifteq {
    pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Rshifteq {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.syntax, f)
    }
}
impl AstToken for Rshifteq {
    fn can_cast(kind: SyntaxKind) -> bool { kind == RSHIFTEQ }
    fn cast(syntax: SyntaxToken) -> Option<Self> {
        if Self::can_cast(syntax.kind()) {
            Some(Self { syntax })
        } else {
            None
        }
    }
    fn syntax(&self) -> &SyntaxToken { &self.syntax }
}