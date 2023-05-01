pub(crate) mod ext;
pub(crate) mod generated;
pub(crate) mod traits;

use crate::lang::{SyntaxNode, SyntaxNodeChildren, SyntaxToken};
use dyn_clone::DynClone;
use either::Either;
pub use generated::{kinds::*, nodes::*, tokens::*};
use std::{fmt::Debug, marker::PhantomData};

/// A **typed** AST node.
///
/// Defines the behavior for converting between an **untyped** `SyntaxNode` and
/// a **typed** `AstNode`.
///
/// The conversion itself has no runtime cost since both `AstNode`s and
/// `SyntaxNode`s have exactly the same representation. That is, they both
/// contain a **pointer** to the **tree root** and a **pointer** to the **node
/// itself**.
///
/// The `AstNode` trait is implemented for all the AST nodes and enforces
/// the invariant that the specific `SyntaxNode` has the specific `SyntaxKind`.
pub trait AstNode: DynClone + Debug {
    /// Returns `true` if the syntax node has the **correct kind** for this AST
    /// node and as such can be converted to an `AstNode`. Otherwise,
    /// returns `false`.
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized;

    /// Casts the given syntax node to an [`AstNode`], converting the **untyped**
    /// [`SyntaxNode`] to a **typed** [`AstNode`], if the syntax node _has_ the
    /// **correct kind**. Otherwise, returns `None`.
    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized;

    /// Returns the underlying [`SyntaxNode`] that this [`AstNode`] wraps. This is
    /// the symmetric or inverse operation of [`AstNode::cast`].
    fn syntax(&self) -> &SyntaxNode;

    /// **Clones** the [`AstNode`] for _updating its properties_, **preserving the original
    /// node**. The resulting AstNode will have the **same SyntaxNode** as the
    /// original, but will be **mutable**.
    ///
    /// Returns a **new** [`AstNode`] with the **same** [`SyntaxNode`].
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Create a new node.
    /// let node: SomeAstNode = ...;
    ///
    /// // Clone the node for update.
    /// let updated_node = node.clone_for_update();
    ///
    /// // Now, you can update properties on the `updated_node` without affecting
    /// // the original `node`.
    ///
    /// // For example, you can update the text of the node.
    /// updated_node.set_text("new text");
    ///
    /// // Or, you can add a child to the node.
    /// updated_node.add_child(SomeAstNode::new());
    ///
    /// // This is the cornerstone for the immutability of the AST. You can
    /// // update the AST without affecting the original AST.
    ///
    /// assert!(!original_node.text().contains("new text"));
    /// ```
    fn clone_for_update(&self) -> Self
    where
        Self: Sized,
    {
        Self::cast(self.syntax().clone_for_update()).expect("Failed to clone for update")
    }

    /// Clones the [`AstNode`] and its **entire subtree**, creating a **deep copy** of the
    /// original node and all its children.
    ///
    /// Returns a **new** [`AstNode`] with a **cloned subtree**.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// let node: SomeAstNode = ...;
    /// let cloned_subtree = node.clone_subtree();
    /// // Now, you can modify the `cloned_subtree` without affecting the original
    /// // `node` or any of its children.
    /// ```
    fn clone_subtree(&self) -> Self
    where
        Self: Sized,
    {
        Self::cast(self.syntax().clone_subtree()).expect("Failed to clone subtree")
    }
}

dyn_clone::clone_trait_object!(AstNode);

// impl Debug for dyn AstNode {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_struct("AstNode")
//             .field("kind", &self.syntax().kind())
//             .finish()
//     }
// }

/// A **typed** AST token.
///
/// Defines the behavior for converting between an **untyped** `SyntaxToken` and
/// a **typed** `AstToken`. The conversion itself has no runtime cost since both
/// `AstToken`s and `SyntaxToken`s have exactly the same representation. That
/// is, they both contain a **pointer** to the **token itself**.
///
/// The `AstToken` trait is implemented for all the AST tokens and enforces
/// the invariant that the specific `SyntaxToken` has the specific `SyntaxKind`.
pub trait AstToken {
    /// Returns `true` if the syntax token has the **correct kind** for this AST
    /// token and as such can be converted to an `AstToken`. Otherwise,
    /// returns `false`.
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized;

    /// Casts the given syntax token to an `AstToken`, converting the
    /// **untyped** `SyntaxToken` to a **typed** `AstToken`, if the syntax
    /// token has the **correct kind**. Otherwise, returns `None`.
    fn cast(syntax: SyntaxToken) -> Option<Self>
    where
        Self: Sized;

    /// Returns the underlying `SyntaxToken` that this `AstToken` wraps. This is
    /// the symmetric or inverse operation of `AstToken::cast`.
    fn syntax(&self) -> &SyntaxToken;

    /// TODO: Document
    fn text(&self) -> &str {
        self.syntax().text()
    }
}

/// An **iterator** over [`SyntaxNode`] children of a **particular AST type**.
///
/// [`AstChildren`] is an **iterator** that yields the **ONLY** the children of the
/// given type, **`N`**, found within the syntax tree of the **parent** [`AstNode`].
///
/// # Example
///
/// ```rust,ignore
/// // Create a new node.
/// let node: SomeAstNode = ...;
///
/// // Iterate over the children of the node.
/// for child in node.children() {
///    // Do something with the child.
/// }
/// ```
#[derive(Debug, Clone)]
pub struct AstChildren<N> {
    inner: SyntaxNodeChildren,
    ph: PhantomData<N>,
}

/// TODO: Document
impl<N> AstChildren<N> {
    /// Initializes a new [`AstChildren`] iterator for a given parent [`SyntaxNode`].
    ///
    /// The [`AstChildren`] iterator will yield **only** the children of the given
    /// type, **`N`**, found within the syntax tree of the **parent** [`AstNode`].
    fn new(parent: &SyntaxNode) -> Self {
        AstChildren {
            inner: parent.children(),
            ph: PhantomData,
        }
    }
}

impl<N: AstNode> Iterator for AstChildren<N> {
    type Item = N;
    fn next(&mut self) -> Option<N> {
        self.inner.find_map(N::cast)
    }
}

impl<L, R> AstNode for Either<L, R>
where
    L: AstNode + DynClone + Clone,
    R: AstNode + DynClone + Clone,
{
    fn can_cast(kind: SyntaxKind) -> bool
    where
        Self: Sized,
    {
        L::can_cast(kind) || R::can_cast(kind)
    }

    fn cast(syntax: SyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if L::can_cast(syntax.kind()) {
            L::cast(syntax).map(Either::Left)
        } else {
            R::cast(syntax).map(Either::Right)
        }
    }

    fn syntax(&self) -> &SyntaxNode {
        self.as_ref().either(L::syntax, R::syntax)
    }

    fn clone_for_update(&self) -> Self
    where
        Self: Sized,
    {
        Self::cast(self.syntax().clone_for_update()).expect("Failed to clone for update")
    }

    fn clone_subtree(&self) -> Self
    where
        Self: Sized,
    {
        Self::cast(self.syntax().clone_subtree()).expect("Failed to clone subtree")
    }
}

/// Helper functions for working with [`SyntaxNode`] and [`SyntaxToken`] instances.
mod support {
    use super::{AstChildren, AstNode, SyntaxKind, SyntaxNode, SyntaxToken};

    /// Returns the **first child** from given parent [`SyntaxNode`] with the **specified
    /// AST type `N`**. If **no such child** exists, returns [`None`].
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use crate::support;
    ///
    /// let parent: SyntaxNode = ...;
    /// let first_child: Option<SomeAstNode> = support::child::<SomeAstNode>(&parent);
    /// ```
    pub(super) fn child<N: AstNode>(parent: &SyntaxNode) -> Option<N> {
        parent.children().find_map(N::cast)
    }

    /// Returns an **iterator** over the **children** from given parent [`SyntaxNode`]
    /// with the **specified AST type `N`**.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// use crate::support;
    /// let parent: SyntaxNode = File::parse(r"
    /// def foo():
    ///     [i ** 2 for i in range(10)]
    ///
    /// def bar():
    ///     "hello"
    ///
    /// def baz():
    ///     2 + 2
    /// ");
    ///
    /// // Get an iterator over the children of the specified type `Statement`.
    /// let children: AstChildren<Statement> = support::children::<Statement>(&parent);
    ///
    /// for child in children {
    ///     // Process each statement within the file...
    /// }
    /// ```
    pub(super) fn children<N: AstNode>(parent: &SyntaxNode) -> AstChildren<N> {
        AstChildren::new(parent)
    }

    /// Returns the first SyntaxToken child of a given parent SyntaxNode with
    /// the specified SyntaxKind. If no such child exists, returns None.
    ///
    /// # Example
    ///
    /// /// use crate::support; /// use crate::SyntaxKind; /// /// let parent: SyntaxNode = ...; /// let kind: SyntaxKind = ...; /// let first_token: Option<SyntaxToken> = support::token(&parent, kind); /// /// // Process the `SyntaxToken` with the specified `SyntaxKind` if it exists. ///
    pub(super) fn token(parent: &SyntaxNode, kind: SyntaxKind) -> Option<SyntaxToken> {
        parent
            .children_with_tokens()
            .filter_map(|it| it.into_token())
            .find(|it| it.kind() == kind)
    }
}
