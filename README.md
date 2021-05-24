# deprive

![https://docs.rs/deprive/badge.svg](Docs)

Tired:

```rust
struct X {}
impl !Send for X {}
impl !Sync for X {}
```

Wired:

```rust
#[deprive(Send, Sync)]
struct X {}
```