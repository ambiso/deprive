# deprive

[![Docs](https://docs.rs/deprive/badge.svg)](https://docs.rs/deprive/latest/deprive/attr.deprive.html)

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