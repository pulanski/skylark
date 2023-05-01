//! **Extensions** to the AST. These are additional functionality that is not
//! generated from the grammar, but is instead manually implemented.

pub(crate) mod kinds;
pub(crate) mod nodes;

// /// `Parse` is the result of the parsing: a syntax tree and a collection of errors.
// ///
// /// Note that we always produce a syntax tree, event for completely invalid files.
// #[derive(Debug, PartialEq, Eq)]
// pub struct Parse<T> {
//     green: GreenNode,
//     errors: Arc<[SyntaxError]>,
//     _ty: PhantomData<fn() -> T>,
// }
