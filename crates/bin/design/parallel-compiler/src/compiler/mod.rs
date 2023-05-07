mod chunk;
mod lexer;
mod parser;
mod task;

use codespan_reporting::diagnostic::{Diagnostic, Label};
use codespan_reporting::files::SimpleFiles;
use codespan_reporting::term::termcolor::{ColorChoice, StandardStream};
use indicatif::ProgressBar;
use parser::AST;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use std::{any::Any, path::Path};
use tracing::{debug, info, warn};

use crate::NUM_ITERATIONS;

// use self::task::{ChunkTask, LexTask, ParseTask, Task};

// tasks: Vec<Box<dyn Task<dyn Any, dyn Any>>>,
// Vec<Box<dyn Task<Input = Box<dyn Any + Send + Sync>, Output = Box<dyn Any + Send + Sync>>>>,
// task_tx: Sender<Box<dyn Any + Send>>,
// task_rx: Receiver<Box<dyn Any + Send>>,
// // task_tx: Sender<Task>,
// // task_rx: Receiver<Task>,
// // task_tx: UnboundedSender<Box<dyn Any + Send>>,
// // task_rx: UnboundedReceiver<Box<dyn Any + Send>>,

// tasks: Vec::new(),

pub struct Compiler {
    file_db: SimpleFiles<String, String>,
    pb: ProgressBar,
}

// pub async fn execute_tasks(&mut self) {
//     let task_count = self.tasks.len();

//     debug!("Executing {} tasks", task_count);

//     if task_count == 0 {
//         return;
//     }

//     let tasks = std::mem::take(&mut self.tasks);
//     let mut futures = Vec::new();

//     for task in tasks {
//         futures.push(task.execute(task.input));
//     }

//     futures::future::join_all(futures).await;
// }

// fn handle_success(&self, path: &Path, ast: &AST) -> Result<String, Box<dyn std::error::Error>> {
//     let ast_string = format!("AST for file {}: {:?}", path.display(), ast);
//     println!("{ast_string}");
//     Ok(ast_string)
// }

// fn handle_error(
//     &self,
//     file_id: usize,
//     source_code: &str,
// ) -> Result<(), Box<dyn std::error::Error>> {
//     let writer = StandardStream::stderr(ColorChoice::Always);
//     let config = codespan_reporting::term::Config::default();
//     let diagnostic = Diagnostic::error()
//         .with_message("Invalid output type for Parser")
//         .with_labels(vec![
//             Label::primary(file_id, 0..source_code.len()).with_message("Parsing failed here")
//         ]);

//     codespan_reporting::term::emit(&mut writer.lock(), &config, &self.file_db, &diagnostic)
//         .expect("Failed to write to stderr");

//     Ok(())
// }

impl Compiler {
    pub fn new(pb: ProgressBar) -> Self {
        Compiler {
            file_db: SimpleFiles::new(),
            pb,
        }
    }

    pub async fn compile(
        &mut self,
        dir_path: PathBuf,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let paths = get_paths_recursively(&dir_path);

        let message = format!("Compiling {}...", dir_path.display());
        self.pb.set_message(message);
        for path in paths {
            info!("Reading file: {}", path.display());
            if let Ok(source_code) = read_file_to_string(&path) {
                let file_id = self
                    .file_db
                    .add(path.to_string_lossy().to_string(), source_code.clone());

                let lexing_message = format!("Lexing {}", path.display());
                self.pb.set_message(lexing_message);

                lex();

                let message = format!("Parsing {}", path.display());
                self.pb.set_message(message);

                parse();

                let message = format!("Typechecking {}", path.display());
                self.pb.set_message(message);

                typecheck();

                let message = format!("Lowering to HIR {}", path.display());
                self.pb.set_message(message);

                lower_to_hir();

                let message = format!("Lowering to MIR {}", path.display());
                self.pb.set_message(message);

                lower_to_mir();

                let message = format!("Codegening {}", path.display());
                self.pb.set_message(message);

                codegen();
            } else {
                warn!("Failed to read file: {}", path.display());
            }
        }
        self.pb.set_message("Done.");
        Ok("".to_string())
    }
}

#[tracing::instrument]
fn lex() {
    for i in 0..NUM_ITERATIONS {
        if i % 10 == 0 {
            debug!("lexed {} tokens", i);
        }

        // sleep for 1ms to simulate work
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

#[tracing::instrument]
fn parse() {
    for i in 0..NUM_ITERATIONS {
        if i % 10 == 0 {
            debug!("parsed {} AST nodes", i);
        }

        // sleep for 1ms to simulate work
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

#[tracing::instrument]
fn typecheck() {
    for i in 0..NUM_ITERATIONS {
        if i % 10 == 0 {
            debug!("typechecked {} AST nodes", i);
        }

        // sleep for 1ms to simulate work
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

#[tracing::instrument]
fn lower_to_hir() {
    for i in 0..NUM_ITERATIONS {
        if i % 10 == 0 {
            debug!("lowered {} AST nodes", i);
        }

        // sleep for 1ms to simulate work
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

#[tracing::instrument]
fn lower_to_mir() {
    for i in 0..NUM_ITERATIONS {
        if i % 10 == 0 {
            debug!("lowered {} hir nodes", i);
        }

        // sleep for 1ms to simulate work
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}

#[tracing::instrument]
fn codegen() {
    for i in 0..NUM_ITERATIONS {
        if i % 10 == 0 {
            debug!("codegened {} mir nodes", i);
        }

        // sleep for 1ms to simulate work
        std::thread::sleep(std::time::Duration::from_millis(1));
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
