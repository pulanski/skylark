use std::collections::BTreeSet;

use quote::{format_ident, quote};
use ungrammar::{Grammar, Rule};

use crate::input::{AstEnumSrc, AstNodeSrc, Cardinality, STARLARK_KINDS_SRC};

use super::input::{AstSrc, Field};

pub(crate) fn lower(grammar: &Grammar) -> AstSrc {
    let mut res = AstSrc {
        tokens: STARLARK_KINDS_SRC
            .tokens
            .iter()
            .chain(STARLARK_KINDS_SRC.literals)
            .chain(STARLARK_KINDS_SRC.keywords)
            .chain(STARLARK_KINDS_SRC.punct.iter().map(|it| &it.1))
            .chain(STARLARK_KINDS_SRC.contextual_keywords)
            .map(|it| to_upper_snake_case(it))
            .collect::<Vec<_>>(),
        ..Default::default()
    };
    // tracing::debug!!("Initialized AstSrc with tokens");

    let nodes = grammar.iter().collect::<Vec<_>>();

    for &node in &nodes {
        let name = grammar[node].name.clone();
        let rule = &grammar[node].rule;
        // tracing::debug!!("Lowering node: {} = {:?}", name, rule);
        match lower_enum(grammar, rule) {
            Some(variants) => {
                // tracing::debug!!("Lowered enum: {} = {:?}", name, variants);
                let enum_src = AstEnumSrc {
                    doc: Vec::new(),
                    name,
                    traits: Vec::new(),
                    variants,
                };
                res.enums.push(enum_src);
            }
            None => {
                // tracing::debug!!("Lowering rule: {} = {:?}", name, rule);
                let mut fields = Vec::new();
                lower_rule(&mut fields, grammar, None, rule);

                // tracing::debug!!(
                //     "Lowered rule: {} = {:?} with fields {:?}",
                //     name,
                //     rule,
                //     fields
                // );
                res.nodes.push(AstNodeSrc {
                    doc: Vec::new(),
                    name,
                    traits: Vec::new(),
                    fields,
                });
            }
        }
    }

    deduplicate_fields(&mut res);
    extract_enums(&mut res);
    extract_struct_traits(&mut res);
    extract_enum_traits(&mut res);
    res
}

fn extract_enums(ast: &mut AstSrc) {
    for node in &mut ast.nodes {
        for enm in &ast.enums {
            let mut to_remove = Vec::new();
            for (i, field) in node.fields.iter().enumerate() {
                let ty = field.ty().to_string();
                if enm.variants.iter().any(|it| it == &ty) {
                    to_remove.push(i);
                }
            }
            if to_remove.len() == enm.variants.len() {
                node.remove_field(to_remove);
                let ty = enm.name.clone();
                let name = to_lower_snake_case(&ty);
                node.fields.push(Field::Node {
                    name,
                    ty,
                    cardinality: Cardinality::Optional,
                });
            }
        }
    }
}

fn extract_struct_traits(ast: &mut AstSrc) {
    let traits: &[(&str, &[&str])] = &[
        ("HasParameters", &["parameters"]),
        ("HasBody", &["suite"]),
        ("HasLoopVariables", &["loop_variables"]),
        ("HasExpression", &["expression"]),
        ("HasTest", &["test"]),
        ("HasEntries", &["entries"]),
    ];

    // for node in &mut ast.nodes {
    //     for (name, methods) in traits {
    //         extract_struct_trait(node, name, methods);
    //     }
    // }

    let nodes_with_doc_comments = [
        "File",
        "DefStmt",
        "IfStmt",
        "ForStmt",
        "SimpleStmt",
        "Suite",
        "SmallStmt",
        "ExprStmt",
        "Test",
        "IfExpr",
        "PrimaryExpr",
        "UnaryExpr",
        "BinaryExpr",
        "LambdaExpr",
        "Expression",
    ];

    // for node in &mut ast.nodes {
    //     if nodes_with_doc_comments.contains(&&*node.name) {
    //         node.traits.push("HasDocComments".into());
    //     }
    // }
}

fn extract_struct_trait(node: &mut AstNodeSrc, trait_name: &str, methods: &[&str]) {
    let mut to_remove = Vec::new();
    for (i, field) in node.fields.iter().enumerate() {
        let method_name = field.method_name().to_string();
        if methods.iter().any(|&it| it == method_name) {
            to_remove.push(i);
        }
    }
    if to_remove.len() == methods.len() {
        node.traits.push(trait_name.to_string());
        node.remove_field(to_remove);
    }
}

impl AstNodeSrc {
    fn remove_field(&mut self, to_remove: Vec<usize>) {
        to_remove.into_iter().rev().for_each(|idx| {
            self.fields.remove(idx);
        });
    }
}

impl Field {
    pub(crate) fn is_many(&self) -> bool {
        matches!(
            self,
            Field::Node {
                cardinality: Cardinality::Many,
                ..
            }
        )
    }

    pub(crate) fn token_kind(&self) -> Option<proc_macro2::TokenStream> {
        match self {
            Field::Token(token) => {
                let token: proc_macro2::TokenStream = token.parse().expect("Invalid token");

                tracing::debug!("Generating token for field: {}", token);
                // if the token is empty, then replace with dslash <-- this is a hack
                if token.is_empty() {
                    return Some(quote! { T![dslash] });
                }
                Some(quote! { T![#token] })
            }
            _ => None,
        }
    }

    pub(crate) fn method_name(&self) -> proc_macro2::Ident {
        match self {
            Field::Token(name) => {
                let name = match name.as_str() {
                    ";" => "semicolon",
                    "->" => "thin_arrow",
                    "'{'" => "l_curly",
                    "'}'" => "r_curly",
                    "'('" => "l_paren",
                    "')'" => "r_paren",
                    "'['" => "l_brack",
                    "']'" => "r_brack",
                    "<" => "l_angle",
                    ">" => "r_angle",
                    "=" => "eq",
                    "==" => "eqeq",
                    "!=" => "noteq",
                    "!" => "excl",
                    "*" => "star",
                    "&" => "amp",
                    "_" => "underscore",
                    "." => "dot",
                    ".." => "dotdot",
                    "..." => "dotdotdot",
                    "..=" => "dotdoteq",
                    "=>" => "fat_arrow",
                    "@" => "at",
                    ":" => "colon",
                    "::" => "coloncolon",
                    "#" => "pound",
                    "?" => "question_mark",
                    "," => "comma",
                    "|" => "pipe",
                    "~" => "tilde",
                    "+" => "plus",
                    "-" => "minus",
                    "/" => "slash",
                    "%" => "percent",
                    "^" => "caret",
                    "<<" => "shl",
                    ">>" => "shr",
                    "//" => "dslash",
                    "**" => "starstar",
                    "+=" => "pluseq",
                    "-=" => "minuseq",
                    "*=" => "stareq",
                    "/=" => "slasheq",
                    "%=" => "percenteq",
                    "&=" => "ampeq",
                    "|=" => "pipeeq",
                    "^=" => "careteq",
                    "<<=" => "shl_eq",
                    ">>=" => "shr_eq",
                    "<=" => "le",
                    ">=" => "ge",
                    "//=" => "slashslasheq",
                    _ => name,
                };
                format_ident!("{}_token", name)
            }
            Field::Node { name, .. } => {
                if name == "type" {
                    format_ident!("ty")
                } else {
                    format_ident!("{}", name)
                }
            }
        }
    }

    pub(crate) fn ty(&self) -> proc_macro2::Ident {
        match self {
            Field::Token(_) => format_ident!("SyntaxToken"),
            Field::Node { ty, .. } => format_ident!("{}", ty),
        }
    }
}

fn deduplicate_fields(ast: &mut AstSrc) {
    for node in &mut ast.nodes {
        let mut i = 0;
        'outer: while i < node.fields.len() {
            for j in 0..i {
                let f1 = &node.fields[i];
                let f2 = &node.fields[j];
                if f1 == f2 {
                    node.fields.remove(i);
                    continue 'outer;
                }
            }
            i += 1;
        }
    }
}

fn lower_enum(grammar: &Grammar, rule: &Rule) -> Option<Vec<String>> {
    let alternatives = match rule {
        Rule::Alt(it) => it,
        _ => return None,
    };
    let mut variants = Vec::new();
    for alternative in alternatives {
        match alternative {
            Rule::Node(it) => variants.push(grammar[*it].name.clone()),
            Rule::Token(it) if grammar[*it].name == ";" => (),
            _ => return None,
        }
    }
    Some(variants)
}

fn lower_rule(acc: &mut Vec<Field>, grammar: &Grammar, label: Option<&String>, rule: &Rule) {
    if lower_comma_list(acc, grammar, label, rule) {
        return;
    }

    // tracing::debug!!("Not a comma list, lowering {:?}", rule);

    match rule {
        Rule::Node(node) => {
            let ty = grammar[*node].name.clone();
            let name = label.cloned().unwrap_or_else(|| to_lower_snake_case(&ty));
            let field = Field::Node {
                name,
                ty,
                cardinality: Cardinality::Optional,
            };
            acc.push(field);
        }
        Rule::Token(token) => {
            assert!(label.is_none());
            let mut name = grammar[*token].name.clone();
            if name != "int" && name != "float" && name != "string" && name != "bytes" {
                if "[]{}()".contains(&name) {
                    name = format!("'{name}'");
                }
                let field = Field::Token(name);
                acc.push(field);
            }
        }
        Rule::Rep(inner) => {
            // tracing::debug!!("Lowering repeated rule {:?}", inner);

            if let Rule::Node(node) = &**inner {
                let ty = grammar[*node].name.clone();
                let name = label
                    .cloned()
                    .unwrap_or_else(|| pluralize(&to_lower_snake_case(&ty)));

                // tracing::debug!!(
                //     "Lowering repeated node {:?} with name {} and type {}",
                //     inner,
                //     name,
                //     ty
                // );

                let field = Field::Node {
                    name,
                    ty,
                    cardinality: Cardinality::Many,
                };
                acc.push(field);
                return;
            }

            if let Rule::Alt(alts) = &**inner {
                // tracing::debug!!("Lowering repeated alt {:?}", alts);

                let mut variants = Vec::new();
                for alt in alts {
                    // tracing::debug!!("Lowering repeated alt {:?}", alt);
                    if let Rule::Node(node) = alt {
                        variants.push(grammar[*node].name.clone());
                    } else if let Rule::Token(token) = alt {
                        if grammar[*token].name == ";" {
                            continue;
                        }

                        variants.push(grammar[*token].name.clone());
                    } else {
                        panic!("unhandled rule: {rule:?}");
                    }
                }
                let name = label
                    .cloned()
                    .unwrap_or_else(|| pluralize(&to_lower_snake_case(&variants[0])));
                let field = Field::Node {
                    name,
                    ty: "SyntaxNode".to_string(),
                    cardinality: Cardinality::Many,
                };
                acc.push(field);
                return;
            }

            if let Rule::Seq(seq) = &**inner {
                // tracing::debug!!("Lowering repeated seq {:?}", seq);

                let mut fields = Vec::new();
                for rule in seq {
                    lower_rule(&mut fields, grammar, None, rule);
                }
                let name = label.cloned().unwrap_or_else(|| {
                    pluralize(&to_lower_snake_case(&fields[0].method_name().to_string()))
                });
                let field = Field::Node {
                    name,
                    ty: "SyntaxNode".to_string(),
                    cardinality: Cardinality::Many,
                };
                acc.push(field);
                return;
            }

            panic!("unhandled rule: {rule:?}");
        }
        Rule::Labeled { label: l, rule } => {
            assert!(label.is_none());
            let manually_implemented = matches!(
                l.as_str(),
                "lhs" | "rhs" | "condition" | "body" | "key" | "value" | "args" | "elements"
            );
            if manually_implemented {
                return;
            }
            lower_rule(acc, grammar, Some(l), rule);
        }
        Rule::Seq(rules) | Rule::Alt(rules) => {
            for rule in rules {
                lower_rule(acc, grammar, label, rule);
            }
        }
        Rule::Opt(rule) => lower_rule(acc, grammar, label, rule),
    }
}

fn lower_comma_list(
    acc: &mut Vec<Field>,
    grammar: &Grammar,
    label: Option<&String>,
    rule: &Rule,
) -> bool {
    // tracing::debug!!("Lowering comma list: {:?}", rule);

    let rule = match rule {
        Rule::Seq(it) => {
            // tracing::debug!!("Found sequence: {:?}", it);
            it
        }
        _ => {
            // tracing::debug!!("Not a sequence");
            return false;
        }
    };

    let (node, repeat, trailing_comma) = match rule.as_slice() {
        [Rule::Node(node), Rule::Rep(repeat), Rule::Opt(trailing_comma)] => {
            // tracing::debug!!("Found node, repeat, trailing_comma sequence");
            (node, repeat, trailing_comma)
        }
        _ => {
            // tracing::debug!!("Not a node, repeat, trailing_comma sequence");
            return false;
        }
    };
    let repeat = match &**repeat {
        Rule::Seq(it) => it,
        _ => return false,
    };
    match repeat.as_slice() {
        [comma, Rule::Node(n)] if comma == &**trailing_comma && n == node => (),
        _ => return false,
    }
    let ty = grammar[*node].name.clone();
    let name = label
        .cloned()
        .unwrap_or_else(|| pluralize(&to_lower_snake_case(&ty)));
    let field = Field::Node {
        name,
        ty,
        cardinality: Cardinality::Many,
    };
    acc.push(field);
    true
}

fn to_lower_snake_case(name: &str) -> String {
    let mut res = String::new();
    let mut prev = '_';
    for c in name.chars() {
        if c.is_uppercase() && prev != '_' {
            res.push('_');
        }
        res.push(c.to_ascii_lowercase());
        prev = c;
    }
    res
}

fn pluralize(name: &str) -> String {
    if name.ends_with('s') {
        format!("{name}es")
    } else {
        format!("{name}s")
    }
}

fn extract_enum_traits(ast: &mut AstSrc) {
    for enm in &mut ast.enums {
        if enm.name == "Stmt" {
            continue;
        }
        let nodes = &ast.nodes;
        let mut variant_traits = enm
            .variants
            .iter()
            .map(|var| nodes.iter().find(|it| &it.name == var).unwrap())
            .map(|node| node.traits.iter().cloned().collect::<BTreeSet<_>>());

        let mut enum_traits = match variant_traits.next() {
            Some(it) => it,
            None => continue,
        };
        for traits in variant_traits {
            enum_traits = enum_traits.intersection(&traits).cloned().collect();
        }
        enm.traits = enum_traits.into_iter().collect();
    }
}

pub fn to_upper_snake_case(name: &str) -> String {
    let mut res = String::new();
    let mut prev = '_';

    for c in name.chars() {
        if c.is_uppercase() && prev.is_lowercase() {
            res.push('_');
        }
        res.push(c.to_ascii_uppercase());
        prev = c;
    }

    res
}

pub fn to_pascal_case(name: &str) -> String {
    let mut res = String::new();
    let mut prev = '_';

    for c in name.chars() {
        if c == '_' {
            prev = '_';
            continue;
        }
        if prev == '_' {
            res.push(c.to_ascii_uppercase());
        } else {
            res.push(c.to_ascii_lowercase());
        }
        prev = c;
    }

    res
}

pub fn normalize_newlines(s: &str) -> String {
    s.replace("\r\n", "\n")
}

/// The **kind** of source code to **generate**. This is used to generate
/// different code for different contexts (e.g. `SyntaxKind` vs `ast::Name`).
pub enum GeneratorKind {
    /// The various **syntax kinds** (e.g. `SyntaxKind::NAME`).
    Kind,
    /// The various **syntax nodes** (e.g. `ast::Name`).
    Node,
    /// The various **syntax tokens** (e.g. `ast::NameRef`).
    Token,
}

impl GeneratorKind {
    fn build_sources_string(path_prefix: &str, file_names: &[&str]) -> String {
        file_names
            .iter()
            .map(|file_name| format!("{path_prefix}{file_name}"))
            .collect::<Vec<_>>()
            .join("`,\n//! `")
    }

    /// Returns the **source files** relevant to the given `GeneratorKind`.
    pub fn relevant_sources(&self) -> String {
        let path_prefix = "syntaxgen/";

        match self {
            GeneratorKind::Kind => {
                let kinds = ["kinds.rs", "input.rs", "sourcegen.rs"];
                Self::build_sources_string(path_prefix, &kinds)
            }
            GeneratorKind::Node => {
                let nodes = ["nodes.rs", "input.rs", "sourcegen.rs"];
                Self::build_sources_string(path_prefix, &nodes)
            }
            GeneratorKind::Token => {
                let tokens = ["tokens.rs", "input.rs", "sourcegen.rs"];
                Self::build_sources_string(path_prefix, &tokens)
            }
        }
    }

    /// Returns the **doc comment** for the given `GeneratorKind`.
    /// This is used to generate the top-level doc comment for the generated
    /// file.
    pub fn mod_doc_comment(&self) -> &'static str {
        match self {
            GeneratorKind::Kind => {
                "//! Defines the `SyntaxKind` enumeration, which represents various syntax elements in the source code.
//! It includes literals, tokens, keywords, punctuation, and nodes. Each variant of the enumeration represents
//! a specific syntax element, which can be used to classify and process parts of the source code during
//! parsing and analysis.
//!
//! The `SyntaxKind` enumeration is used to represent the type of a syntax node in the AST (Abstract Syntax Tree)
//! and is a bridge between the lexer and the parser."
            }
            GeneratorKind::Node => {"//! Provides a set of various nodes in the AST (Abstract Syntax Tree) and their corresponding behavior.
//! The purpose of these nodes is to facilitate easier navigation and manipulation of the AST
//! by providing a set of methods to access various tokens and related nodes.
//!
//! Each node contains a `syntax` field representing the syntax node and provides methods to access
//! various tokens and related nodes."
        }
            GeneratorKind::Token => "/// A syntax token.",
        }
    }
}
