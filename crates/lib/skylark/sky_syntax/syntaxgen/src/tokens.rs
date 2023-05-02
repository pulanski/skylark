use std::ascii::AsciiExt;

use proc_macro2::Ident;
use quote::{format_ident, quote};

use crate::{
    add_preamble, reformat,
    sourcegen::{to_pascal_case, to_upper_snake_case, GeneratorKind},
};

use super::input::AstSrc;

fn is_keyword(token: &str) -> bool {
    matches!(
        token.to_ascii_lowercase().as_str(),
        "and"
            | "else"
            | "load"
            | "break"
            | "for"
            | "not"
            | "continue"
            | "if"
            | "or"
            | "def"
            | "in"
            | "pass"
            | "elif"
            | "lambda"
            | "return"
    )
}

fn format_kind(token: &str) -> Ident {
    tracing::info!("format_kind: {}", token);
    // if the token is a keyword, then we need to append _kw to the end of the
    // token name
    if is_keyword(token) {
        tracing::info!("format_kind: {} is a keyword", token);
        return format_ident!("{}_KW", to_upper_snake_case(token));
    }

    format_ident!("{}", to_upper_snake_case(token))
}

pub(crate) fn generate_tokens(grammar: &AstSrc) -> String {
    let tokens = grammar.tokens.iter().map(|token| {
        let name = format_ident!("{}", to_pascal_case(token));
        let kind = format_kind(token);

        tracing::debug!("Generating token: {} with kind: {}", name, kind);

        quote! {
            #[derive(Debug, Clone, PartialEq, Eq, Hash)]
            pub struct #name {
                pub(crate) syntax: SyntaxToken,
            }
            impl std::fmt::Display for #name {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    std::fmt::Display::fmt(&self.syntax, f)
                }
            }
            impl AstToken for #name {
                fn can_cast(kind: SyntaxKind) -> bool { kind == #kind }
                fn cast(syntax: SyntaxToken) -> Option<Self> {
                    if Self::can_cast(syntax.kind()) { Some(Self { syntax }) } else { None }
                }
                fn syntax(&self) -> &SyntaxToken { &self.syntax }
            }
        }
    });

    tracing::debug!(
        "Generating tokens {:?}",
        tokens.clone().map(|t| t.to_string())
    );

    add_preamble(
        reformat(
            quote! {
                use crate::{SyntaxKind::{self, *}, SyntaxToken, ast::AstToken};
                #(#tokens)*
            }
            .to_string(),
        ),
        GeneratorKind::Token,
    )
    .replace("#[derive", "\n#[derive")
}
