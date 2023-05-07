use std::io::{self, Write};

use ir::{Diagnostics, SourceProgram};

// ANCHOR: jar_struct
#[salsa::jar(db = Db)]
pub struct Jar(
    crate::compile::compile,
    crate::ir::SourceProgram,
    crate::ir::Program,
    crate::ir::VariableId,
    crate::ir::FunctionId,
    crate::ir::Function,
    crate::ir::Diagnostics,
    crate::ir::Span,
    crate::parser::parse_statements,
    crate::type_check::type_check_program,
    crate::type_check::type_check_function,
    crate::type_check::find_function,
);
// ANCHOR_END: jar_struct

// ANCHOR: jar_db
pub trait Db: salsa::DbWithJar<Jar> {}
// ANCHOR_END: jar_db

// ANCHOR: jar_db_impl
impl<DB> Db for DB where DB: ?Sized + salsa::DbWithJar<Jar> {}
// ANCHOR_END: jar_db_impl

mod compile;
mod db;
mod ir;
mod parser;
mod type_check;

// pub fn main() {
//     let mut db = db::Database::default();
//     let mut source_code = String::new();

//     loop {
//         print!("> ");
//         io::stdout().flush().unwrap();

//         let mut input = String::new();
//         match io::stdin().read_line(&mut input) {
//             Ok(_) => {
//                 source_code.push_str(&input);
//                 let source_program = SourceProgram::new(&db, source_code.clone());
//                 compile::compile(&db, source_program);
//                 let diagnostics = compile::compile::accumulated::<Diagnostics>(&db, source_program);
//                 if diagnostics.is_empty() {
//                     println!("Compilation successful");
//                 } else {
//                     eprintln!("Diagnostics: {:#?}", diagnostics);
//                 }
//             }
//             Err(error) => {
//                 eprintln!("Error reading input: {}", error);
//                 break;
//             }
//         }
//     }
// }
// pub fn main() {
//     let db = db::Database::default();

//     let source_program = SourceProgram::new(&db, String::new());
//     compile::compile(&db, source_program);
//     let diagnostics = compile::compile::accumulated::<Diagnostics>(&db, source_program);
//     eprintln!("{diagnostics:?}");
// }
fn main() {
    let source = r"def foo():
        pass $$
    ";

    let ast = sky_syntax::File::parse(source);

    println!("ast: {:#?}", ast.debug_dump());

    //     let mut lexer = sky_syntax::StarlarkLexer::new();

    //     let token_sink = lexer.tokenize(source);

    //     if token_sink.has_errors() {
    //         println!("has errors");
    //         lexer.emit_errors();
    //     }

    // println!("tokens: {:#?}", token_sink.tokens());

    // let parser = sky_syntax::StarlarkParser::new();

    // let ast = parser.parse(source);

    // println!("ast: {:#?}", ast.debug_dump());
}
