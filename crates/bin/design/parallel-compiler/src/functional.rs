//! A purely functional implementation of a parallel compiler.

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use indicatif::{ProgressBar, ProgressStyle};
use rayon::prelude::*;
use std::any::Any;
use std::fs;
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;

pub type FileId = usize;

// The SourceChunk represents a chunk of source code.
#[derive(Debug, Clone)]
pub struct SourceChunk {
    code: String,
}

// Function to partition the source code into syntax-aware chunks.
// This is a simplified version; a real implementation would use more sophisticated logic.
fn partition_source(source_code: &str) -> Vec<SourceChunk> {
    // In this example, we just create a single chunk containing the entire source code.
    vec![SourceChunk {
        code: source_code.to_string(),
    }]
}

// The various stages of the compilation process.
#[derive(Debug, Clone, Copy)]
pub enum CompilationStage {
    Lexer,
    Parser,
    SemanticAnalyzer,
    CodeGenerator,
}

// The Task represents a unit of work for a specific compilation stage.
#[derive(Debug, Clone)]
pub struct Task {
    stage: CompilationStage,
    input: Arc<dyn Any + Send + Sync>,
    output: Arc<Mutex<Option<Arc<dyn Any + Send + Sync>>>>,
    dependencies: Option<Vec<Arc<Task>>>,
}

impl Task {
    // Create a new Task with the given stage and input.
    pub fn new(stage: CompilationStage, input: Arc<dyn Any + Send + Sync>) -> Self {
        Self {
            stage,
            input,
            output: Arc::new(Mutex::new(None)),
            dependencies: None,
        }
    }

    // Create a new Task with the given stage, input, and dependencies.
    pub fn new_with_deps(
        stage: CompilationStage,
        input: Arc<dyn Any + Send + Sync>,
        dependencies: Vec<Arc<Task>>,
    ) -> Self {
        Self {
            stage,
            input,
            output: Arc::new(Mutex::new(None)),
            dependencies: Some(dependencies),
        }
    }

    // Run the task, waiting for dependencies to complete if necessary.
    pub fn run(&self) {
        if let Some(dependencies) = &self.dependencies {
            dependencies.par_iter().for_each(|task| task.run());
        }

        let result: Arc<dyn Any + Send + Sync> = match self.stage {
            CompilationStage::Lexer => {
                let source_chunk = self.input.downcast_ref::<SourceChunk>();

                if let Some(source_chunk) = source_chunk {
                    debug!("Lexing source chunk");
                    let tokens = lex(source_chunk);

                    debug!("Lexed tokens: {:?}", tokens);
                    Arc::new(tokens)
                } else {
                    panic!("Invalid input type for Lexer");
                }
            }
            CompilationStage::Parser => {
                let output = self
                    .input
                    .downcast_ref::<Arc<Mutex<Option<Arc<dyn Any + Send + Sync>>>>>();

                if let Some(output) = output {
                    let locked_output = output.lock().expect("Failed to lock lexer output");
                    if let Some(lexer_output) = locked_output.as_ref() {
                        let tokens = lexer_output
                            .downcast_ref::<Vec<Token>>()
                            .expect("Invalid input type for Parser");
                        debug!("Parsing tokens");
                        let ast = parse(tokens);
                        Arc::new(ast)
                    } else {
                        panic!("Lexer output is not set");
                    }
                } else {
                    panic!("Invalid input type for Parser");
                }
            }
            CompilationStage::SemanticAnalyzer => {
                // ... Implement semantic analysis logic
                todo!()
            }
            CompilationStage::CodeGenerator => {
                // ... Implement code generation logic
                todo!()
            }
        };

        if let Ok(mut output) = self.output.lock() {
            *output = Some(result);
        } else {
            panic!("Failed to acquire lock on task output");
        }
    }
}

// The Compiler is responsible for coordinating and executing tasks.
pub struct Compiler {
    tasks: Vec<Arc<Task>>,
    file_db: SimpleFiles<String, String>,
    progress_bar: ProgressBar,
}

impl Default for Compiler {
    fn default() -> Self {
        Self::new()
    }
}

impl Compiler {
    pub fn new() -> Self {
        // Set up indicatif progress bar
        let progress_bar = ProgressBar::new_spinner();
        progress_bar.enable_steady_tick(Duration::from_millis(100));
        progress_bar.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} [{elapsed_precise}] {msg}")
                .expect("Failed to set progress bar style"),
        );

        Self {
            tasks: Vec::new(),
            file_db: SimpleFiles::new(),
            progress_bar,
        }
    }

    // Add a task to the compiler.
    pub fn add_task(&mut self, task: Arc<Task>) {
        self.tasks.push(task);
    }

    // Execute all tasks in parallel.
    pub fn execute_tasks(&self) {
        self.tasks.par_iter().for_each(|task| task.run());
    }

    // Compile the given source code.
    pub fn compile(&mut self, source_code: &str, source_path: &str) {
        let source_chunks = partition_source(source_code);

        // Add the source code to the file database, if it doesn't already exist
        let file_id = self
            .file_db
            .add(source_path.to_owned(), source_code.to_owned());

        // Create tasks for lexing the source chunks
        let lex_tasks: Vec<_> = source_chunks
            .into_iter()
            .map(|chunk| Task::new(CompilationStage::Lexer, Arc::new(chunk)))
            .collect();

        // Add lex tasks to the compiler
        for task in &lex_tasks {
            self.add_task(task.clone().into());
        }

        // Create tasks for parsing the token streams generated by the lex tasks
        let parse_tasks: Vec<_> = lex_tasks
            .into_iter()
            .map(|lex_task| {
                debug!("Creating parse task for {:?}", lex_task);

                Task::new_with_deps(
                    CompilationStage::Parser,
                    lex_task.output.clone(),
                    vec![lex_task.into()],
                )
            })
            .collect();

        // Add parse tasks to the compiler
        for task in &parse_tasks {
            self.add_task(task.clone().into());
        }

        // Execute tasks
        self.execute_tasks();

        // sleep(Duration::from_millis(1000));

        // Collect the results and print the generated ASTs
        for task in &parse_tasks {
            let output = task.output.lock().unwrap();
            if let Some(ast) = output.as_ref().and_then(|arc| arc.downcast_ref::<AST>()) {
                println!("AST: {ast:?}");
            } else {
                self.emit_error(file_id, source_code);
            }
        }
    }

    pub fn compile_file(&mut self, file_id: FileId) {
        let source_file = self.file_db.get(file_id).expect("Invalid file ID").clone();
        let source_code = source_file.source().clone();

        self.compile(&source_code, source_file.name());
    }

    pub fn compile_all(&mut self, dir_path: PathBuf) {
        let paths = get_paths_recursively(&dir_path);

        let message = format!("Compiling //{}/...", dir_path.display());
        self.progress_bar.set_message(message);

        for path in paths {
            info!("Ingesting file: {}", path.display());
            if let Ok(source_code) = read_file_to_string(&path) {
                let file_id = self
                    .file_db
                    .add(path.to_string_lossy().into_owned(), source_code.clone());
                debug!("Added file to in-memory database: `{}`", path.display());

                self.compile_file(file_id);
            } else {
                eprintln!("Failed to read file: {}", path.display());
            }
        }
    }

    fn emit_error(&self, file_id: usize, source_code: &str) {
        let writer = StandardStream::stderr(ColorChoice::Always);
        let config = codespan_reporting::term::Config::default();
        let diagnostic = Diagnostic::error()
            .with_message("Invalid output type for Parser")
            .with_labels(vec![
                Label::primary(file_id, 0..source_code.len()).with_message("Parsing failed here")
            ]);

        codespan_reporting::term::emit(&mut writer.lock(), &config, &self.file_db, &diagnostic)
            .expect("Failed to write to stderr");
    }
}

// Read the file at the given path and return its content as a string.
fn read_file_to_string(path: &Path) -> std::io::Result<String> {
    let mut file = fs::File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

// Traverse the directory and return a Vec<PathBuf> containing all file paths.
fn get_paths_recursively(dir_path: &Path) -> Vec<PathBuf> {
    let mut paths = Vec::new();

    let allowed_file_names = allowed_filenames();
    let allowed_extensions = allowed_file_extensions();

    if dir_path.is_dir() {
        for entry in fs::read_dir(dir_path).expect("Failed to read directory") {
            let entry = entry.expect("Failed to read entry");
            let path = entry.path();
            if path.is_dir() {
                paths.extend(get_paths_recursively(&path));
            } else {
                let file_name = path.file_name().and_then(|os_str| os_str.to_str());
                let extension = path.extension().and_then(|os_str| os_str.to_str());

                if let (Some(file_name), Some(extension)) = (file_name, extension) {
                    if allowed_file_names.contains(&file_name)
                        || allowed_extensions.contains(&extension)
                    {
                        paths.push(path);
                    }
                }
            }
        }
    }
    paths
}

fn allowed_filenames() -> Vec<&'static str> {
    vec![
        "BUCK",
        "BUILD",
        "BUILD.bazel",
        "WORKSPACE.bazel",
        "WORKSPACE",
    ]
}

fn allowed_file_extensions() -> Vec<&'static str> {
    vec!["bzl", "star"]
}

// Define tokens for our simple example language.
#[derive(Debug, Clone)]
pub enum Token {
    Def,
    Identifier(String),
    Integer(i64),
    Colon,
    Comma,
    OpenParen,
    CloseParen,
    Plus,
    Minus,
    Star,
    Slash,
}

// Implement a simple lexer for the example language.
fn lex(source_chunk: &SourceChunk) -> Vec<Token> {
    let mut tokens = Vec::new();

    let input = source_chunk.code.as_str();
    let mut iter = input.chars().peekable();

    while let Some(&c) = iter.peek() {
        match c {
            ' ' | '\t' | '\n' | '\r' => {
                iter.next();
            }
            'a'..='z' | 'A'..='Z' => {
                let mut ident = String::new();
                while let Some(&c) = iter.peek() {
                    if c.is_alphabetic() {
                        ident.push(c);
                        iter.next();
                    } else {
                        break;
                    }
                }
                if ident == "def" {
                    tokens.push(Token::Def);
                } else {
                    tokens.push(Token::Identifier(ident));
                }
            }
            '0'..='9' => {
                let mut num = 0;
                while let Some(&c) = iter.peek() {
                    if c.is_ascii_digit() {
                        num = num * 10 + c.to_digit(10).unwrap() as i64;
                        iter.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Integer(num));
            }
            '(' => {
                tokens.push(Token::OpenParen);
                iter.next();
            }
            ')' => {
                tokens.push(Token::CloseParen);
                iter.next();
            }
            ':' => {
                tokens.push(Token::Colon);
                iter.next();
            }
            ',' => {
                tokens.push(Token::Comma);
                iter.next();
            }
            '+' => {
                tokens.push(Token::Plus);
                iter.next();
            }
            '-' => {
                tokens.push(Token::Minus);
                iter.next();
            }
            '*' => {
                tokens.push(Token::Star);
                iter.next();
            }
            '/' => {
                tokens.push(Token::Slash);
                iter.next();
            }
            _ => {
                panic!("Unexpected character: {c}");
            }
        }
    }

    debug!(
        "Lexed tokens: {:?} from source chunk: {:?}",
        tokens, source_chunk
    );

    tokens
}

// Define an AST for the simple example language.
#[derive(Debug)]
pub enum Expr {
    Function(String, Vec<Expr>),
    Integer(i64),
}

#[derive(Debug)]
pub struct AST {
    pub root: Vec<Expr>,
}

// Implement a simple parser for the example language.
fn parse(tokens: &[Token]) -> AST {
    let mut ast = AST { root: Vec::new() };
    let mut iter = tokens.iter().peekable();

    while let Some(token) = iter.peek() {
        match token {
            Token::Def => {
                iter.next();
                if let Some(Token::Identifier(name)) = iter.next() {
                    if let Some(Token::OpenParen) = iter.next() {
                        let mut args = Vec::new();
                        loop {
                            match iter.peek() {
                                Some(Token::CloseParen) => {
                                    iter.next();
                                    break;
                                }
                                Some(Token::Integer(num)) => {
                                    args.push(Expr::Integer(*num));
                                    iter.next();
                                }
                                _ => panic!("Unexpected token in function arguments"),
                            }
                        }
                        ast.root.push(Expr::Function(name.clone(), args));
                    } else {
                        panic!("Function name is missing");
                    }
                }
            }
            _ => panic!("Unexpected token: {token:?}"),
        }
    }

    ast
}

pub(crate) fn main() {
    // Set up tracing subscriber
    let subscriber = FmtSubscriber::builder()
        // .with_env_filter("compiler=info")
        .with_max_level(Level::TRACE)
        .finish();
    tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

    let mut compiler = Compiler::new();

    let source_root = PathBuf::from("test_data");
    compiler.compile_all(source_root);
}
