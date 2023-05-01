// use crate::ast::generated::kinds::_IMPL_NUM_FromPrimitive_FOR_SyntaxKind::_num_traits::FromPrimitive;
use crate::ast::SyntaxKind;
use rowan::Language;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Starlark {}

/// [`skylark`]'s [`Language`][crate::lang::Language] type, a wrapper around
/// [`rowan`]'s language-agnostic [`Language`][rowan::Language] type.
///
/// [`Language`] is the interface [`skylark`] uses to bridge the gap between
/// [`rowan`]'s language-agnostic [`SyntaxNode`][rowan::SyntaxNode] type and
/// [`skylark`]'s [`SyntaxNode`][crate::lang::SyntaxNode] type.
impl Language for Starlark {
    type Kind = SyntaxKind;

    /// Get the [`SyntaxKind`] from the raw [`rowan::SyntaxKind`].
    ///
    /// Users of [`skylark`] should not need to call this directly and
    /// should only need to interact with [`SyntaxKind`]s.
    fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
        SyntaxKind::from(raw.0)
    }

    /// Convert the [`SyntaxKind`] to the raw [`rowan::SyntaxKind`].
    ///
    /// Users of [`skylark`] should not need to call this directly and
    /// should only need to interact with [`SyntaxKind`]s.
    fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind {
        rowan::SyntaxKind(u16::from(kind))
    }
}

/// [`skylark`]'s [`SyntaxNode`][crate::lang::SyntaxNode] type, a wrapper around
/// [`rowan`]'s language-agnostic [`SyntaxNode`][rowan::SyntaxNode] type.
///
/// These are analogous to a _`RedNode`_ in the [**Red-Green**](https://ericlippert.com/2012/06/08/red-green-trees/) tree model seen in **Roslyn**.
/// They signify **non-terminal nodes** in the syntax tree.
///
/// [`skylark`]: https://crates.io/crates/skylark
/// [`cstree`]: https://crates.io/crates/rowan
/// [`SyntaxNode`]: crate::syntax_tree::SyntaxNode
pub type SyntaxNode = rowan::SyntaxNode<Starlark>;

/// [`skylark`]'s [`SyntaxToken`][crate::lang::SyntaxToken] type, a wrapper around
/// [`rowan`]'s language-agnostic [`SyntaxToken`][rowan::SyntaxToken] type.
///
/// These are analogous to a _`RedNode`_ in the [**Red-Green**](https://ericlippert.com/2012/06/08/red-green-trees/) tree model seen in **Roslyn**, however, they specifically signify **terminal nodes** in the syntax
/// tree aka **tokens**.
///
/// [`skylark`]: https://crates.io/crates/skylark
/// [`rowan`]: https://crates.io/crates/rowan
/// [`SyntaxToken`]: crate::syntax_tree::SyntaxToken
pub type SyntaxToken = rowan::SyntaxToken<Starlark>;

/// [`skylark`]'s [`SyntaxElement`][crate::lang::SyntaxElement] type, a wrapper around
/// [`rowan`]'s language-agnostic [`SyntaxElement`][rowan::SyntaxElement] type.
///
/// These are analogous to a _`GreenNode`_ in the [**Red-Green**](https://ericlippert.com/2012/06/08/red-green-trees/) tree model seen in **Roslyn**.
///
/// [`skylark`]: https://crates.io/crates/skylark
/// [`rowan`]: https://crates.io/crates/rowan
/// [`SyntaxElement`]: crate::syntax_tree::SyntaxElement
pub type SyntaxElement = rowan::NodeOrToken<SyntaxNode, SyntaxToken>;

/// [`skylark`]'s [`SyntaxNodeChildren`][crate::lang::SyntaxNodeChildren] type, a wrapper around
/// [`rowan`]'s language-agnostic [`SyntaxNodeChildren`][rowan::SyntaxNodeChildren] type.
///
/// These provide an **iterator** over the children of a [`SyntaxNode`][crate::lang::SyntaxNode].
///
/// [`skylark`]: https://crates.io/crates/skylark
/// [`rowan`]: https://crates.io/crates/rowan
/// [`SyntaxNodeChildren`]: crate::syntax_tree::SyntaxNodeChildren
/// [`SyntaxNode`]: crate::syntax_tree::SyntaxNode
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<Starlark>;

/// [`skylark`]'s [`SyntaxElementChildren`][crate::lang::SyntaxElementChildren] type, a wrapper around
/// [`rowan`]'s language-agnostic [`SyntaxElementChildren`][rowan::SyntaxElementChildren] type.
///
/// These provide an **iterator** over the children of a [`SyntaxElement`][crate::lang::SyntaxElement].
///
/// [`skylark`]: https://crates.io/crates/skylark
/// [`rowan`]: https://crates.io/crates/rowan
/// [`SyntaxElementChildren`]: crate::syntax_tree::SyntaxElementChildren
/// [`SyntaxElement`]: crate::syntax_tree::SyntaxElement
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<Starlark>;

/// [`skylark`]'s [`PreorderWithTokens`][crate::lang::PreorderWithTokens] type, a wrapper around
/// [`rowan`]'s language-agnostic [`PreorderWithTokens`][rowan::api::PreorderWithTokens] type.
///
/// These provide an **iterator** over the children of a [`SyntaxNode`][crate::lang::SyntaxNode] in **pre-order**.
/// This means that the iterator will first yield the node itself, then all of its children, then all of its grandchildren, etc.
/// The iterator will also yield all of the tokens in the tree, in the order they appear in the source code.
/// This is useful for **syntax highlighting** and **pretty-printing** as well as general **tree traversal**.
///
/// [`skylark`]: https://crates.io/crates/skylark
/// [`rowan`]: https://crates.io/crates/rowan
/// [`PreorderWithTokens`]: crate::lang::PreorderWithTokens
/// [`SyntaxNode`]: crate::lang::SyntaxNode
pub type PreorderWithTokens = rowan::api::PreorderWithTokens<SyntaxNode>;
