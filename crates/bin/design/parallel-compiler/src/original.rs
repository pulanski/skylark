// use rayon::prelude::*;
// use std::{
//     any::Any,
//     sync::{Arc, Mutex},
// };

// #[derive(Debug, Clone)]
// pub enum Token {
//     Function,
//     Identifier(String),
//     Integer(i64),
//     OpenParen,
//     CloseParen,
// }

// fn lex(source_chunk: &SourceChunk) -> Vec<Token> {
//     let mut tokens = Vec::new();

//     let input = source_chunk.code.as_str();
//     let mut iter = input.chars().peekable();

//     while let Some(&c) = iter.peek() {
//         match c {
//             ' ' | '\t' | '\n' | '\r' => {
//                 iter.next();
//             }
//             'a'..='z' | 'A'..='Z' => {
//                 let mut ident = String::new();
//                 while let Some(&c) = iter.peek() {
//                     if c.is_alphabetic() {
//                         ident.push(c);
//                         iter.next();
//                     } else {
//                         break;
//                     }
//                 }
//                 if ident == "fn" {
//                     tokens.push(Token::Function);
//                 } else {
//                     tokens.push(Token::Identifier(ident));
//                 }
//             }
//             '0'..='9' => {
//                 let mut num = 0;
//                 while let Some(&c) = iter.peek() {
//                     if c.is_ascii_digit() {
//                         num = num * 10 + c.to_digit(10).unwrap() as i64;
//                         iter.next();
//                     } else {
//                         break;
//                     }
//                 }
//                 tokens.push(Token::Integer(num));
//             }
//             '(' => {
//                 tokens.push(Token::OpenParen);
//                 iter.next();
//             }
//             ')' => {
//                 tokens.push(Token::CloseParen);
//                 iter.next();
//             }
//             _ => {
//                 panic!("Unexpected character: {c}");
//             }
//         }
//     }

//     tokens
// }

// #[derive(Debug)]
// pub enum Expr {
//     Function(String, Vec<Expr>),
//     Integer(i64),
// }

// #[derive(Debug)]
// pub struct AST {
//     pub root: Vec<Expr>,
// }

// fn parse(tokens: &[Token]) -> AST {
//     let mut ast = AST { root: Vec::new() };
//     let mut iter = tokens.iter().peekable();

//     while let Some(token) = iter.peek() {
//         match token {
//             Token::Function => {
//                 iter.next();
//                 if let Some(Token::Identifier(name)) = iter.next() {
//                     if let Some(Token::OpenParen) = iter.next() {
//                         let mut args = Vec::new();
//                         while let Some(token) = iter.peek() {
//                             match token {
//                                 Token::Integer(value) => {
//                                     args.push(Expr::Integer(*value));
//                                     iter.next();
//                                 }
//                                 Token::CloseParen => {
//                                     iter.next();
//                                     break;
//                                 }
//                                 _ => {
//                                     panic!("Unexpected token: {:?}", token);
//                                 }
//                             }
//                         }
//                         ast.root.push(Expr::Function(name.clone(), args));
//                     } else {
//                         panic!("Expected '(', found {:?}", iter.next());
//                     }
//                 } else {
//                     panic!("Expected identifier, found {:?}", iter.next());
//                 }
//             }
//             _ => {
//                 panic!("Unexpected token: {:?}", token);
//             }
//         }
//     }

//     ast
// }

// /// Stores a **chunk** of source code (e.g. a **file**, a **function**, a **class**, etc.)
// /// that can be _processed in parallel_. Typically, a chunk is a **contiguous
// /// sequence of characters** which amount to a **given syntactic construct**.
// pub struct SourceChunk {
//     code: String,
// }

// /// Partitions a given source code into **chunks** that can be _processed in parallel_.
// /// In this model, chunks use a fairly naive chunking algorithm.
// fn partition_source(source_code: &str) -> Vec<SourceChunk> {
//     // ... Implement the syntax-aware chunking logic
//     todo!()
// }

// /// A **stage** within the compilation pipeline
// /// (e.g. _lexical analysis_, _parsing_, _semantic analysis_, _code generation_, etc.)
// #[derive(Clone, Copy)]
// pub enum CompilationStage {
//     Lexer,
//     Parser,
//     SemanticAnalyzer,
//     CodeGenerator,
// }

// /// A **task** within the compilation pipeline (e.g. _lex_ a character stream, _parse_ a token stream, etc.)
// #[derive(Clone)]
// pub struct Task {
//     stage: CompilationStage,
//     input: Arc<dyn Any + Send + Sync>,
//     output: Arc<Mutex<Option<Arc<dyn Any + Send + Sync>>>>,
//     dependencies: Option<Vec<Arc<Task>>>,
// }

// impl Task {
//     pub fn new(stage: CompilationStage, input: Arc<dyn Any + Send + Sync>) -> Self {
//         Self {
//             stage,
//             input,
//             output: Arc::new(Mutex::new(None)),
//             dependencies: None,
//         }
//     }

//     pub fn new_with_deps(
//         stage: CompilationStage,
//         input: Arc<dyn Any + Send + Sync>,
//         dependencies: Vec<Arc<Task>>,
//     ) -> Self {
//         Self {
//             stage,
//             input,
//             output: Arc::new(Mutex::new(None)),
//             dependencies: Some(dependencies),
//         }
//     }

//     pub fn run(&self) {
//         if let Some(dependencies) = &self.dependencies {
//             dependencies.par_iter().for_each(|task| task.run());
//         }

//         let result: Arc<dyn Any + Send + Sync> = match self.stage {
//             CompilationStage::Lexer => {
//                 // ... Implement lexer logic
//                 todo!()
//             }
//             CompilationStage::Parser => {
//                 // ... Implement parser logic
//                 todo!()
//             }
//             CompilationStage::SemanticAnalyzer => {
//                 // ... Implement semantic analysis logic
//                 todo!()
//             }
//             CompilationStage::CodeGenerator => {
//                 // ... Implement code generation logic
//                 todo!()
//             }
//         };

//         let mut output = self.output.lock().unwrap();
//         *output = Some(result);
//     }
// }

// pub struct Compiler {
//     tasks: Vec<Arc<Task>>,
// }

// impl Compiler {
//     pub fn new() -> Self {
//         Self { tasks: Vec::new() }
//     }

//     pub fn add_task(&mut self, task: Task) {
//         self.tasks.push(Arc::new(task));
//     }

//     pub fn execute_tasks(&self) {
//         self.tasks.par_iter().for_each(|task| task.run());
//     }

//     pub fn compile(&mut self, source_code: &str) {
//         // ... Create tasks based on the input source code
//         // ... Add tasks to the compiler

//         let source_chunks = partition_source(source_code);

//         // Create tasks for lexing the source chunks
//         let lex_tasks: Vec<_> = source_chunks
//             .into_iter()
//             .map(|chunk| Task::new(CompilationStage::Lexer, Arc::new(chunk)))
//             .collect();

//         // Add lex tasks to the compiler
//         for task in &lex_tasks {
//             self.add_task(task.clone());
//         }

//         // Create tasks for parsing the token streams generated by the lex tasks
//         let parse_tasks: Vec<_> = lex_tasks
//             .into_iter()
//             .map(|lex_task| {
//                 Task::new_with_deps(
//                     CompilationStage::Parser,
//                     lex_task.output.clone(),
//                     vec![lex_task.into()],
//                 )
//             })
//             .collect();

//         // Add parse tasks to the compiler
//         for task in &parse_tasks {
//             self.add_task(task.clone());
//         }

//         // Execute tasks
//         self.execute_tasks();
//     }
// }

// fn main() {
//     let source_code = "...";
//     let mut compiler = Compiler::new();
//     compiler.compile(source_code);
// }
