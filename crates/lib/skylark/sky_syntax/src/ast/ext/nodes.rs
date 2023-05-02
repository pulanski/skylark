use std::{marker::PhantomData, sync::Arc};

use crate::{
    ast::{AstNode, Expression, File},
    parsing, Parse, SyntaxKind, SyntaxNode,
};

impl Parse<SyntaxNode> {
    pub fn cast<N: AstNode>(self) -> Option<Parse<N>> {
        if N::cast(self.syntax_node()).is_some() {
            Some(Parse {
                green: self.green,
                errors: self.errors,
                _ty: PhantomData,
            })
        } else {
            None
        }
    }
}

// impl Parse<File> {
//     pub fn debug_dump(&self) -> String {
//         let mut buf = format!("{:#?}", self.tree().syntax());
//         for err in self.errors.iter() {
//             writeln!(buf, "error {:?}: {}", err.location(), err.kind()).unwrap();
//         }
//         buf
//     }
//     /// Parses the `SourceFile` again but with the given modification applied.
//     pub fn reparse(&self, indel: &Indel) -> Parse<SourceFile> {
//         // TODO: Implement something smarter here.
//         self.full_reparse(indel)
//     }
//     /// Performs a "reparse" of the `SourceFile` after applying the specified modification by
//     /// simply parsing the entire thing again.
//     fn full_reparse(&self, indel: &Indel) -> Parse<SourceFile> {
//         let mut text = self.tree().syntax().text().to_string();
//         indel.apply(&mut text);
//         File::parse(&text)
//     }
// }

impl File {
    pub fn parse(text: &str) -> Parse<File> {
        // todo:
        let (green, mut errors) = parsing::parse_text(text);
        let root = SyntaxNode::new_root(green.clone());

        // // errors.extend(validation::validate(&root));

        assert!(
            root.kind() == SyntaxKind::FILE,
            "Expected to parse a file, but instead got {:?}. This is a bug in the parser and should be reported.",
            root.kind()
        );

        Parse {
            green,
            errors: Arc::from(errors),
            _ty: PhantomData,
        }
    }
}
