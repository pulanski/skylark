use std::mem;

use crate::{parser::ParseError, parsing::TreeSink, SyntaxKind, T};

/// `Parser` produces a flat list of `Events`'s. They are converted to a tree structure in a
/// separate pass via a `TreeSink`.
#[derive(Debug)]
pub enum Event {
    /// This event signifies the start of a node.
    /// It should be either abandoned (in which case the `kind` is `TOMBSTONE`, and the event is
    /// ignored), or completed via a `Finish` event.
    ///
    /// All tokens between a `Start` and a `Finish` become the children of the respective node.
    Start {
        kind: SyntaxKind,
        forward_parent: Option<u32>,
    },

    /// Completes the previous `Start` event
    Finish,

    StartError,

    FinishError(ParseError),

    /// Produce a single leaf-element.
    /// `n_raw_tokens` is used to glue complex contextual tokens.
    /// For example, the lexer tokenizes `>>` as `>`, `>`. `n_raw_tokens = 2` is used to produce
    /// a single `>>`.
    Token {
        kind: SyntaxKind,
        n_raw_tokens: u8,
    },

    Error(ParseError),
}

impl Event {
    pub(crate) fn tombstone() -> Event {
        Event::Start {
            kind: SyntaxKind::TOMBSTONE,
            forward_parent: None,
        }
    }
}

pub(super) fn process(mut sink: &mut dyn TreeSink, mut events: Vec<Event>) {
    let mut forward_parents = Vec::new();

    for i in 0..events.len() {
        match mem::replace(&mut events[i], Event::tombstone()) {
            Event::Start {
                kind: SyntaxKind::WHITESPACE,
                ..
            } => {}
            Event::Start {
                kind,
                forward_parent,
            } => {
                // For events[A, B, C], B is A's forward_parent, C is B's forward_parent,
                // in the normal control flow, the parent-child relation: `A -> B -> C`,
                // while with the magic forward_parent, it writes: `C <- B <- A`.

                // append `A` into parents.
                forward_parents.push(kind);
                let mut idx = i;
                let mut fp = forward_parent;
                while let Some(fwd) = fp {
                    idx += fwd as usize;
                    // append `A`'s forward_parent `B`
                    fp = match mem::replace(&mut events[idx], Event::tombstone()) {
                        Event::Start {
                            kind,
                            forward_parent,
                        } => {
                            if kind != SyntaxKind::TOMBSTONE {
                                forward_parents.push(kind);
                            }
                            forward_parent
                        }
                        _ => unreachable!(),
                    };
                    // append `B`'s forward_parent `C` in the next stage.
                }

                for kind in forward_parents.drain(..).rev() {
                    sink.start_node(kind);
                }
            }
            Event::Error(e) => sink.error(e),
            Event::Finish => sink.finish_node(),
            Event::Token { kind, n_raw_tokens } => sink.token(kind, n_raw_tokens),
            Event::StartError => unimplemented!(),
            Event::FinishError(_) => unimplemented!(),
        }
    }

    sink.finish_node()
}
