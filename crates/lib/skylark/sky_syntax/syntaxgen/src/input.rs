/// The **effective input** for the **code generation process** of the
/// **syntax tree**.
///
/// Specifies all the _different kinds_ of **syntax nodes** and **tokens** in
/// the **language**.
pub struct KindsSrc<'a> {
    pub punct: &'a [(&'a str, &'a str)],
    pub keywords: &'a [&'a str],
    pub contextual_keywords: &'a [&'a str],
    pub literals: &'a [&'a str],
    pub tokens: &'a [&'a str],
    pub nodes: &'a [&'a str],
}

#[derive(Default, Debug)]
pub(crate) struct AstSrc {
    pub tokens: Vec<String>,
    pub nodes: Vec<AstNodeSrc>,
    pub enums: Vec<AstEnumSrc>,
}

#[derive(Debug)]
pub struct AstNodeSrc {
    pub doc: Vec<String>,
    pub name: String,
    pub traits: Vec<String>,
    pub fields: Vec<Field>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum Field {
    Token(String),
    Node {
        name: String,
        ty: String,
        cardinality: Cardinality,
    },
}

#[derive(Debug, Eq, PartialEq)]
pub enum Cardinality {
    Optional,
    Many,
}

#[derive(Debug)]
pub struct AstEnumSrc {
    pub doc: Vec<String>,
    pub name: String,
    pub traits: Vec<String>,
    pub variants: Vec<String>,
}

pub const STARLARK_KINDS_SRC: KindsSrc = KindsSrc {
    // Punctuation
    //     +    -    *    //   %    **
    // ~    &    |    ^    <<   >>
    // .    ,    =    ;    :
    // (    )    [    ]    {    }
    // <    >    >=   <=   ==   !=
    // +=   -=   *=   //=  %=
    // &=   |=   ^=   <<=  >>=
    punct: &[
        ("+", "PLUS"),
        ("-", "MINUS"),
        ("*", "STAR"),
        ("/", "SLASH"),
        ("//", "DSLASH"),
        ("%", "PERCENT"),
        ("**", "DSTAR"),
        ("~", "TILDE"),
        ("&", "AMP"),
        ("|", "PIPE"),
        ("^", "CARET"),
        ("<<", "LSHIFT"),
        (">>", "RSHIFT"),
        (".", "DOT"),
        (",", "COMMA"),
        ("=", "EQ"),
        (";", "SEMICOLON"),
        (":", "COLON"),
        ("(", "LPAREN"),
        (")", "RPAREN"),
        ("[", "LBRACKET"),
        ("]", "RBRACKET"),
        ("{", "LBRACE"),
        ("}", "RBRACE"),
        ("<", "LT"),
        (">", "GT"),
        (">=", "GE"),
        ("<=", "LE"),
        ("==", "EQEQ"),
        ("!=", "NE"),
        ("+=", "PLUSEQ"),
        ("-=", "MINUSEQ"),
        ("*=", "STAREQ"),
        ("/=", "SLASHEQ"),
        // ("//=", "DSLASHEQ"),
        ("%=", "PERCENTEQ"),
        ("&=", "AMPEQ"),
        ("|=", "PIPEEQ"),
        ("^=", "CARETEQ"),
        ("<<=", "LSHIFTEQ"),
        (">>=", "RSHIFTEQ"),
    ],
    keywords: &[
        "and", "else", "load", "break", "for", "not", "continue", "if", "or", "def", "in", "pass",
        "elif", "lambda", "return",
    ],
    contextual_keywords: &[],
    literals: &["identifier", "int", "float", "string", "bytes"],
    tokens: &[
        "WHITESPACE",
        "COMMENT",
        "NEWLINE",
        "INDENT",
        "OUTDENT",
        "UNKNOWN",
    ],
    nodes: &[
        "File",
        "Statement",
        "DefStmt",
        "Parameters",
        "Parameter",
        "IfStmt",
        "ElifClauses",
        "ElseClause",
        "ForStmt",
        "Suite",
        "SimpleStmt",
        "SmallStmt",
        "ReturnStmt",
        "BreakStmt",
        "ContinueStmt",
        "PassStmt",
        "AssignStmt",
        "ExprStmt",
        "LoadStmt",
        "Test",
        "BinOp",
        "IfExpr",
        "PrimaryExpr",
        "Operand",
        "DotSuffix",
        "SliceSuffix",
        "CallSuffix",
        "Arguments",
        "Argument",
        "ListExpr",
        "ListComp",
        "DictExpr",
        "DictComp",
        "Entries",
        "Entry",
        "CompClause",
        "UnaryExpr",
        "BinaryExpr",
        "Binop",
        "LambdaExpr",
        "Expression",
        "LoopVariables",
    ],
};
