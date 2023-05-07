# TODO

## Syntax

- [ ] Add the generation logic for adding the following additional `SyntaxKind` enum variants to `syntaxgen`.

```rust
[string] => { $ crate :: SyntaxKind :: STRING } ; [bytes] => { $ crate :: SyntaxKind :: BYTES } ; [int] => { $ crate :: SyntaxKind :: INT } ; [float] => { $ crate :: SyntaxKind :: FLOAT } ; [comment] => { $ crate :: SyntaxKind :: COMMENT } ; [whitespace] => { $ crate :: SyntaxKind :: WHITESPACE } ; [error] => { $ crate :: SyntaxKind :: ERROR } ; }
```