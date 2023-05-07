// use super::{
//     chunk::{chunk, SourceChunk},
//     lexer::{lex, TokenStream},
//     parser::{parse, AST},
// };
// use futures::Future;
// use std::{any::Any, pin::Pin, sync::Arc};
// use tokio::sync::Mutex as AsyncMutex;

pub type FileId = usize;

// pub type TaskInput<T> = (FileId, T);
// pub type TaskOutput<T> = Arc<AsyncMutex<Option<Arc<(FileId, T)>>>>;

// pub trait Task<I, O>: Send + Sync + 'static
// where
//     I: Send + Sync + 'static,
//     O: Send + Sync + 'static,
// {
//     fn execute(&self, input: I) -> Pin<Box<dyn Future<Output = O> + Send>>;
// }

// #[derive(Debug)]
// pub struct ChunkTask {
//     input: String,
//     output: SourceChunk,
// }

// impl Task<String, Arc<(FileId, SourceChunk)>> for ChunkTask {
//     fn execute(
//         &self,
//         input: String,
//     ) -> Pin<Box<dyn Future<Output = Arc<(FileId, SourceChunk)>> + Send>> {
//         Box::pin(async move {
//             let (file_id, chunk) = chunk(input).await;
//             Arc::new((file_id, chunk))
//         })
//     }
// }

// #[derive(Debug)]
// pub struct LexTask {
//     input: SourceChunk,
//     output: TokenStream,
// }

// #[derive(Debug)]
// pub struct ParseTask {
//     input: TokenStream,
//     output: AST,
// }
