// mod compiler;
// mod functional;

// use compiler::Compiler;
// use indicatif::{ProgressBar, ProgressStyle};
// use std::{path::PathBuf, time::Duration};
// use tracing::Level;
// use tracing_subscriber::FmtSubscriber;

// pub static NUM_ITERATIONS: usize = 1000;

// #[tokio::main]
// async fn main() {
//     // Set up tracing subscriber
//     let subscriber = FmtSubscriber::builder()
//         // .with_env_filter("compiler=info")
//         .with_max_level(Level::TRACE)
//         .finish();
//     tracing::subscriber::set_global_default(subscriber).expect("Failed to set subscriber");

//     // Set up indicatif progress bar
//     let pb = ProgressBar::new_spinner();
//     pb.enable_steady_tick(Duration::from_millis(100));
//     pb.set_style(
//         ProgressStyle::default_spinner()
//             .template("{spinner:.green} [{elapsed_precise}] {msg}")
//             .expect("Failed to set progress bar style"),
//     );

//     // Compile directory
//     let dir_path = PathBuf::from("test_data");
//     let mut compiler = Compiler::new(pb.clone());
//     if let Err(e) = compiler.compile(dir_path).await {
//         eprintln!("Error: {e}");
//     }

//     pb.finish_and_clear();
// }

// ------------------------------------------------------------------------------------------------

use derive_more::Display;
use derive_new::new;
use futures::stream::{self, StreamExt};
use getset::Getters;
use getset::MutGetters;
use getset::Setters;
use indicatif::ProgressState;
use indicatif::ProgressStyle;
use lazy_static::lazy_static;
use once_cell::sync::Lazy;
use owo_colors::OwoColorize;
use owo_colors::Rgb;
use parking_lot::RwLock;
use rand::thread_rng;
use rand::Rng;
use shrinkwraprs::Shrinkwrap;
use smartstring::alias::String;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::time::Instant;
use tracing::info;
use tracing::info_span;
use tracing::instrument;
use tracing_indicatif::span_ext::IndicatifSpanExt;
use tracing_indicatif::IndicatifLayer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use typed_builder::TypedBuilder;
use ulid::Ulid;
use walkdir::WalkDir;

macro_rules! italicize {
    ($msg:expr) => {
        $msg.italic().to_string().into()
    };
}

macro_rules! bold {
    ($msg:expr) => {
        $msg.bold().to_string().into()
    };
}

macro_rules! color {
    ($msg:expr, $color:expr) => {
        $msg.color($color).to_string().into()
    };
}

// Durations for different progress indicators
static SHORT_DURATION_START_TIME: Duration = Duration::from_secs(1);
static MEDIUM_DURATION_START_TIME: Duration = Duration::from_secs(4);
static LONG_DURATION_START_TIME: Duration = Duration::from_secs(8);
static VERY_LONG_DURATION_START_TIME: Duration = Duration::from_secs(12);

// Track in-progress and completed tasks as well as cache hits
static TASK_COUNTER: Lazy<TaskTracker> = Lazy::new(|| TaskTracker::new(0, 0));

lazy_static! {
    // Colors
    static ref RED: Rgb = Rgb(255, 0, 0);
    static ref ORANGE: Rgb = Rgb(255, 165, 0);
    static ref YELLOW: Rgb = Rgb(255, 255, 0);
    static ref GREEN: Rgb = Rgb(0, 255, 0);
    static ref DARK_GREEN: Rgb = Rgb(0, 100, 0);
    static ref BLUE: Rgb = Rgb(0, 0, 255);
    static ref INDIGO: Rgb = Rgb(75, 0, 130);
    static ref VIOLET: Rgb = Rgb(238, 130, 238);
    static ref PURPLE: Rgb = Rgb(255, 0, 255);
    static ref CYAN: Rgb = Rgb(0, 255, 255);
    static ref WHITE: Rgb = Rgb(255, 255, 255);
    static ref BLACK: Rgb = Rgb(0, 0, 0);

    // Symbols
    static ref RIGHT_ARROW: String = italicize!("↳ ");
    static ref RIGHT_ARROW_SYMBOL: String = color!(&*RIGHT_ARROW, Rgb(0, 0, 0));
}

#[derive(
    Debug,
    Clone,
    Copy,
    Display,
    PartialEq,
    Eq,
    Hash,
    TypedBuilder,
    Getters,
    MutGetters,
    Setters,
    new,
)]
#[display(fmt = "In progress {in_progress}, completed {completed}")]
#[getset(get = "pub", get_mut = "pub", set = "pub")]
struct TaskTrackerData {
    in_progress: usize,
    completed: usize,
}

#[derive(Debug, Clone, Shrinkwrap)]
struct TaskTracker(Arc<RwLock<TaskTrackerData>>);

impl TaskTracker {
    pub fn new(in_progress: usize, completed: usize) -> Self {
        Self(Arc::new(RwLock::new(TaskTrackerData::new(
            in_progress,
            completed,
        ))))
    }
}

fn elapsed_subsec(state: &ProgressState, writer: &mut dyn std::fmt::Write) {
    let seconds = state.elapsed().as_secs();
    let sub_seconds = (state.elapsed().as_millis() % 1000) / 100;
    let duration = state.elapsed();
    let elapsed_secs = duration.as_secs_f64();

    let elapsed = state.elapsed();

    if elapsed > VERY_LONG_DURATION_START_TIME {
        let gradient = interpolate_color(
            &ORANGE,
            &RED,
            (elapsed_secs - VERY_LONG_DURATION_START_TIME.as_secs_f64()) / 3.0,
        );
        let _ = write!(
            writer,
            "{}",
            format!("{seconds}.{sub_seconds}s").color(gradient)
        );
    } else if elapsed > LONG_DURATION_START_TIME {
        let gradient = interpolate_color(
            &YELLOW,
            &ORANGE,
            (elapsed_secs - LONG_DURATION_START_TIME.as_secs_f64()) / 3.0,
        );
        let _ = write!(
            writer,
            "{}",
            format!("{seconds}.{sub_seconds}s").color(gradient)
        );
    } else if elapsed > MEDIUM_DURATION_START_TIME {
        let gradient = interpolate_color(
            &GREEN,
            &YELLOW,
            (elapsed_secs - MEDIUM_DURATION_START_TIME.as_secs_f64()) / 3.0,
        );
        let _ = write!(
            writer,
            "{}",
            format!("{seconds}.{sub_seconds}s").color(gradient).italic()
        );
    } else {
        let gradient = interpolate_color(&DARK_GREEN, &GREEN, elapsed_secs / 3.0);
        let _ = write!(
            writer,
            "{}",
            format!("{seconds}.{sub_seconds}s").color(gradient).italic()
        );
    }
}

pub fn increment_in_progress_task() {
    let mut task_counter = TASK_COUNTER.write();
    *task_counter.in_progress_mut() += 1;
    tracing::trace!(
        "Started task, in progress: {}, completed: {}",
        task_counter.in_progress(),
        task_counter.completed()
    );
}

pub fn increment_completed_task() {
    let mut task_counter = TASK_COUNTER.write();
    *task_counter.completed_mut() += 1;
    if task_counter.in_progress() > &0 {
        *task_counter.in_progress_mut() -= 1;
    }
    tracing::trace!(
        "Completed task, in progress: {}, completed: {}",
        task_counter.in_progress(),
        task_counter.completed()
    );
}

#[instrument(level = "trace", skip_all)]
// #[cached(size = 100)]
async fn build_sub_unit(sub_unit: u64) {
    increment_in_progress_task(); // Increment the in progress task count
    let sleep_time =
        thread_rng().gen_range(Duration::from_millis(5000)..Duration::from_millis(10000));
    tokio::time::sleep(sleep_time).await;

    if thread_rng().gen_bool(0.9) {
        info!("sub_unit {} built successfully", sub_unit);
    }

    increment_completed_task(); // Increment the completed task count
}

pub fn short_running_task_msg(duration: Duration) -> String {
    let elapsed_secs = duration.as_secs_f64();
    let gradient = interpolate_color(&DARK_GREEN, &GREEN, elapsed_secs / 3.0);
    format!(
        "{}{}{}",
        "[".black(),
        "Short".color(gradient).italic(),
        "]".black()
    )
    .into()
}

pub fn medium_running_task_msg(duration: Duration) -> String {
    let elapsed_secs = duration.as_secs_f64();
    let gradient = interpolate_color(&GREEN, &YELLOW, elapsed_secs / 3.0);
    format!(
        "{}{}{}",
        "[".black(),
        "Medium".color(gradient).italic(),
        "]".black()
    )
    .into()
}

pub fn long_running_task_msg(duration: Duration) -> String {
    let elapsed_secs = duration.as_secs_f64();
    let gradient = interpolate_color(&YELLOW, &ORANGE, elapsed_secs / 6.0);
    format!(
        "{}{}{}",
        "[".black(),
        "Long".color(gradient).italic(),
        "]".black()
    )
    .into()
}

pub fn very_long_running_task_msg(duration: Duration) -> String {
    let elapsed_secs = duration.as_secs_f64();
    let gradient = interpolate_color(&ORANGE, &RED, elapsed_secs / 6.0);
    format!(
        "{}{}{}",
        "[".black(),
        "Very Long".color(gradient).italic(),
        "]".black()
    )
    .into()
}

fn interpolate_color(from: &Rgb, to: &Rgb, t: f64) -> Rgb {
    let r = interpolate(from.0, to.0, t);
    let g = interpolate(from.1, to.1, t);
    let b = interpolate(from.2, to.2, t);
    Rgb(r, g, b)
}

fn interpolate(a: u8, b: u8, t: f64) -> u8 {
    let result = a as f64 * (1.0 - t) + b as f64 * t;
    result.round() as u8
}

fn task_msg_display(state: &ProgressState, writer: &mut dyn std::fmt::Write) {
    let elapsed = state.elapsed();

    if elapsed > VERY_LONG_DURATION_START_TIME {
        let _ = write!(
            writer,
            " {} ",
            very_long_running_task_msg(elapsed - VERY_LONG_DURATION_START_TIME)
        );
    } else if elapsed > LONG_DURATION_START_TIME {
        let _ = write!(
            writer,
            " {} ",
            long_running_task_msg(elapsed - LONG_DURATION_START_TIME)
        );
    } else if elapsed > MEDIUM_DURATION_START_TIME {
        let _ = write!(
            writer,
            " {} ",
            medium_running_task_msg(elapsed - MEDIUM_DURATION_START_TIME)
        );
    } else if elapsed > SHORT_DURATION_START_TIME {
        let _ = write!(
            writer,
            " {} ",
            short_running_task_msg(elapsed - SHORT_DURATION_START_TIME)
        );
    }
}

// ------------------------------------------------------------------------------------------------

// #[instrument(level = "trace", skip_all)]
// async fn build(unit: u64) {

// }

static CHUNK_DURATION: Duration = Duration::from_secs(1);
static LEX_DURATION: Duration = Duration::from_secs(2);
static PARSE_DURATION: Duration = Duration::from_secs(3);
static TYPECHECK_DURATION: Duration = Duration::from_secs(4);
static LOWER_TO_HIR_DURATION: Duration = Duration::from_secs(4);
static HIR_TO_MIR_DURATION: Duration = Duration::from_secs(3);
static MIR_TO_LLVM_DURATION: Duration = Duration::from_secs(2);
static LLVM_TO_OBJ_DURATION: Duration = Duration::from_secs(1);
static OBJ_TO_EXE_DURATION: Duration = Duration::from_secs(5);

#[tokio::main]
async fn main() {
    let start_time = Instant::now();
    let task = "build";

    let indicatif_layer = IndicatifLayer::new()
        .with_progress_style(
            ProgressStyle::with_template(
                r"{spinner:.green}{color_start}{span_child_prefix}{span_fields} -- {span_name}{wide_msg}{elapsed_subsec}{color_end}",
            )
            .expect("Failed to initialize TUI")
            .tick_strings(&[
            "    ",
            "◐   ",
            "◓   ",
			"=◑  ",
			"==◒ ",
			"===◐",
			" ===",
			"  ==",
			"   =",
			"    ",
			"   ◓",
			"  ◑=",
			" ◒==",
			"◐===",
			"◓== ",
			"◑=  ",
			"◒   ",
            "    ",])
            .with_key("elapsed_subsec", elapsed_subsec)
            .with_key("color_start", |state: &ProgressState, writer: &mut dyn std::fmt::Write| {
                task_msg_display(state, writer)
            })
            .with_key(
                "color_end",
                |state: &ProgressState, writer: &mut dyn std::fmt::Write| {
                    if state.elapsed() > Duration::from_secs(4) {
                        let _ = write!(writer, "\x1b[0m");
                    }
                },
            )
        )
        .with_span_child_prefix_symbol(&RIGHT_ARROW_SYMBOL)
        .with_span_child_prefix_indent(" ");

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_writer(indicatif_layer.get_stderr_writer())
                .fmt_fields(tracing_subscriber::fmt::format::DefaultFields::new())
                .with_line_number(true)
                // .without_time()
                // .with_thread_names(true)
                .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
                .with_ansi(true)
                .with_timer(tracing_subscriber::fmt::time::Uptime::default()),
        )
        .with(indicatif_layer)
        .init();

    let ulid = Ulid::new();
    let task_id = ulid.to_string()[..10].to_string();
    tracing::info!("{} ID: {}", task, task_id.cyan().italic());

    let task_display = format!(
        "{}{}{} {}{}{}",
        "`".red(),
        task.green(),
        "`".red(),
        "[".black(),
        task_id[..5].to_string().cyan().italic(),
        "]".black()
    );
    // .replace("{task_display}", &task_display)

    let template = "Executing tasks for command: {task_display}. {wide_msg} Jobs: In progress: \
                    {in_progress}. Finished: {completed}. Time elapsed: {elapsed_subsec}
\n{wide_bar}"
        .replace('.', &format!("{}", ".".black()))
        .replace(':', &format!("{}", ":".black()))
        .replace("{task_display}", &task_display)
        .replace("In progress", &format!("{}", "In progress".bright_yellow()))
        .replace("Finished", &format!("{}", "Finished".green()));

    let header_span = info_span!("header");
    header_span.pb_set_style(
        &ProgressStyle::with_template(&template)
            .unwrap()
            .with_key("elapsed_subsec", elapsed_subsec)
            .with_key(
                "in_progress",
                |state: &ProgressState, writer: &mut dyn std::fmt::Write| {
                    let task_counter = TASK_COUNTER.read();

                    let _ = write!(writer, "{}", task_counter.in_progress);
                },
            )
            .with_key(
                "completed",
                |state: &ProgressState, writer: &mut dyn std::fmt::Write| {
                    let task_counter = TASK_COUNTER.read();

                    let _ = write!(writer, "{}", task_counter.completed);
                },
            )
            .progress_chars("---"),
    );
    header_span.pb_start();

    // Display full "-----" line underneath the header.
    header_span.pb_set_length(1);
    header_span.pb_set_position(1);

    let files = WalkDir::new("test_data");
    let files = files
        .into_iter()
        .filter(|e| e.is_ok() && e.as_ref().unwrap().path().is_file())
        .map(|e| e.unwrap().into_path());

    stream::iter(files.map(compile))
        .buffer_unordered(7)
        .collect::<Vec<()>>()
        .await;

    // Display the resulting diagnostics from the task tracker
    let task_counter = TASK_COUNTER.read();

    tracing::info!("Finished executing tasks for command: {}", task_display);
    tracing::info!(
        " Jobs Finished{} {}",
        ":".black(),
        task_counter.completed.green().bold().italic()
    );
    tracing::info!(
        " Time elapsed{} {}{}{}s",
        ":".black(),
        start_time.elapsed().as_secs().to_string().cyan().italic(),
        ".".black(),
        start_time
            .elapsed()
            .subsec_millis()
            .to_string()
            .cyan()
            .italic()
    );
}

// dummy compile function which simulates a compiler pipeline on a file
// e.g. `chunk -> lex -> parse -> typecheck -> lower to hir -> lower to mir ->
// lower to llvm ir -> emit object file -> link`)
async fn compile(path: PathBuf) {
    increment_in_progress_task(); // Increment in-progress tasks when a new task starts

    let task_id = Ulid::new().to_string()[..10].to_string();
    let task_display = format!(
        "{}{}{} {}{}{}",
        "`".red(),
        "compile".green(),
        "`".red(),
        "[".black(),
        task_id[..5].to_string().cyan().italic(),
        "]".black()
    );

    let template = "{task_display} {wide_msg} {msg} {spinner} {wide_bar}"
        .replace('.', &format!("{}", ".".black()))
        .replace(':', &format!("{}", ":".black()))
        .replace("{task_display}", &task_display);

    let task_span = info_span!("compile", task_id = %task_id);

    let source_code = fs::read_to_string(&path)
        .expect("Unable to read file")
        .into();

    let chunk_map = chunk(source_code).await;

    let chunk_map = chunk_map.chunks.into_iter();

    eprintln!("got chunk map");

    let mut tokens = vec![];
    for chunk in chunk_map {
        let token = lex(chunk).await;
        tokens.push(token);
    }
    let tokens = tokens.into_iter();

    eprintln!("got tokens");

    let mut ast_nodes = vec![];
    for token in tokens {
        let ast_node = parse(token).await;
        ast_nodes.push(ast_node);
    }

    eprintln!("got ast nodes");
}

pub struct ChunkMap {
    pub chunks: Vec<SourceChunk>,
}

impl ChunkMap {
    pub fn new() -> Self {
        Self { chunks: vec![] }
    }

    pub fn add_chunk(&mut self, chunk: SourceChunk) {
        self.chunks.push(chunk);
    }
}

impl Default for ChunkMap {
    fn default() -> Self {
        Self::new()
    }
}

pub struct SourceChunk {
    pub source: String,
    pub start: usize,
    pub end: usize,
}

#[tracing::instrument(name = "Chunking source code...", skip(source))]
async fn chunk(source: String) -> ChunkMap {
    increment_in_progress_task(); // Increment in-progress tasks when a new task starts
    info!("Chunking source: {}", source);

    thread::sleep(CHUNK_DURATION);

    increment_completed_task(); // Increment completed tasks when a task finishes
    ChunkMap::new()
}

#[derive(Debug)]
pub struct TokenStream {
    pub tokens: Vec<Token>,
}

impl TokenStream {
    pub fn new() -> Self {
        Self { tokens: vec![] }
    }

    pub fn add_token(&mut self, token: Token) {
        self.tokens.push(token);
    }
}

impl Default for TokenStream {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
}

#[derive(Debug)]
pub enum TokenKind {
    Identifier,
    Keyword,
    Literal,
    Punctuation,
    Operator,
}

#[derive(Debug)]
pub struct Span {
    pub start: usize,
    pub end: usize,
}

#[tracing::instrument(name = "Lexing source chunk...", skip(chunk))]
async fn lex(chunk: SourceChunk) -> TokenStream {
    increment_in_progress_task(); // Increment in-progress tasks when a new task starts
    info!("Lexing chunk: {}", chunk.source);

    thread::sleep(LEX_DURATION);

    increment_completed_task(); // Increment completed tasks when a task finishes
    TokenStream::new()
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

#[tracing::instrument(name = "Parsing token stream...", skip(tokens))]
async fn parse(tokens: TokenStream) -> AST {
    increment_in_progress_task(); // Increment in-progress tasks when a new task starts
    info!("Parsing tokens: {:?}", tokens);

    thread::sleep(PARSE_DURATION);

    increment_completed_task(); // Increment completed tasks when a task finishes
    AST { root: vec![] }
}

#[derive(Debug)]
pub struct Type {
    pub kind: TypeKind,
}

#[derive(Debug)]
pub enum TypeKind {
    Integer,
    Function,
}

#[derive(Debug)]
pub struct TypeAST {
    pub root: Vec<Type>,
}

#[tracing::instrument(name = "Typechecking AST...", skip(ast))]
async fn typecheck(ast: AST) -> TypeAST {
    increment_in_progress_task(); // Increment in-progress tasks when a new task starts
    info!("Typechecking AST: {:?}", ast);

    thread::sleep(TYPECHECK_DURATION);

    increment_completed_task(); // Increment completed tasks when a task finishes
    TypeAST { root: vec![] }
}

#[derive(Debug)]
pub struct HIR {
    pub root: Vec<HIRNode>,
}

#[derive(Debug)]
pub enum HIRNode {
    Function(String, Vec<HIRNode>),
    Integer(i64),
}

#[tracing::instrument(name = "Lowering AST to HIR...", skip(ast))]
async fn lower_to_hir(ast: AST) -> HIR {
    increment_in_progress_task(); // Increment in-progress tasks when a new task starts
    info!("Lowering AST to HIR: {:?}", ast);

    thread::sleep(LOWER_TO_HIR_DURATION);

    increment_completed_task(); // Increment completed tasks when a task finishes
    HIR { root: vec![] }
}

#[derive(Debug)]
pub struct MIR {
    pub root: Vec<MIRNode>,
}

#[derive(Debug)]
pub enum MIRNode {
    Function(String, Vec<MIRNode>),
    Integer(i64),
}

#[tracing::instrument(name = "Lowering HIR to MIR...", skip(hir))]
async fn lower_to_mir(hir: HIR) -> MIR {
    increment_in_progress_task(); // Increment in-progress tasks when a new task starts
    info!("Lowering HIR to MIR: {:?}", hir);

    thread::sleep(HIR_TO_MIR_DURATION);

    increment_completed_task(); // Increment completed tasks when a task finishes
    MIR { root: vec![] }
}

#[derive(Debug)]
pub struct LLVMIR {
    pub root: Vec<LLVMIRNode>,
}

#[derive(Debug)]
pub enum LLVMIRNode {
    Function(String, Vec<LLVMIRNode>),
    Integer(i64),
}

#[tracing::instrument(name = "Lowering MIR to LLVM IR...", skip(mir))]
async fn lower_to_llvm_ir(mir: MIR) -> LLVMIR {
    increment_in_progress_task(); // Increment in-progress tasks when a new task starts
    info!("Lowering MIR to LLVM IR: {:?}", mir);

    thread::sleep(MIR_TO_LLVM_DURATION);

    increment_completed_task(); // Increment completed tasks when a task finishes
    LLVMIR { root: vec![] }
}

#[derive(Debug)]
pub struct ObjectCode {
    pub root: Vec<ObjectCodeNode>,
}

#[derive(Debug)]
pub enum ObjectCodeNode {
    Function(String, Vec<ObjectCodeNode>),
    Integer(i64),
}

#[tracing::instrument(name = "Generating object code...", skip(llvm_ir))]
async fn generate_object_code(llvm_ir: LLVMIR) -> ObjectCode {
    increment_in_progress_task(); // Increment in-progress tasks when a new task starts
    info!("Generating object code: {:?}", llvm_ir);

    thread::sleep(LLVM_TO_OBJ_DURATION);

    increment_completed_task(); // Increment completed tasks when a task finishes
    ObjectCode { root: vec![] }
}

#[derive(Debug)]
pub struct Executable {
    pub root: Vec<ExecutableNode>,
}

#[derive(Debug)]
pub enum ExecutableNode {
    Function(String, Vec<ExecutableNode>),
    Integer(i64),
}

#[tracing::instrument(name = "Linking object code...", skip(object_code))]
async fn link_object_code(object_code: ObjectCode) -> Executable {
    increment_in_progress_task(); // Increment in-progress tasks when a new task starts
    info!("Linking object code: {:?}", object_code);

    thread::sleep(OBJ_TO_EXE_DURATION);

    increment_completed_task(); // Increment completed tasks when a task finishes
    Executable { root: vec![] }
}
