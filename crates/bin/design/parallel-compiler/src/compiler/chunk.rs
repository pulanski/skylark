use std::sync::Arc;

use super::task::FileId;
// use super::task::{FileId, TaskInput, TaskOutput};

// The SourceChunk represents a chunk of source code.
#[derive(Debug, Clone)]
pub struct SourceChunk {
    pub(crate) code: String,
}

// Function to partition the source code into syntax-aware chunks.
// This is a simplified version; a real implementation would use more sophisticated logic.
fn partition_source(source_code: &str) -> Vec<SourceChunk> {
    // In this example, we just create a single chunk containing the entire source code.
    vec![SourceChunk {
        code: source_code.to_string(),
    }]
}

pub(crate) async fn chunk(input: String) -> (FileId, SourceChunk) {
    todo!("chunk")
}
