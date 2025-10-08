This crate provides a macro that allows you to destructure types in a `const` context:

```rust
struct Wrap<T> {
    value: T
}

impl<T> Wrap<T> {
    const fn into_inner(self) -> T {
        const_destructure!(let Self { value } = self);
        value
    }
}
```

Unlike normal destructuring, the macro enforces that you assign all fields.
If it didn't, it would be possible to accidentally leak unassigned fields because of how the macro is implemented. 
This is still a big win over writing the implementation by hand, which requires `unsafe`.

## Motivation

The following doesn't compile on rustc 1.90.0 (2025) due to https://github.com/rust-lang/rust/issues/86897:

```rust
struct Wrap<T> {
    value: T
}

impl Wrap<T> {
    const fn into_inner(self) -> T {
        let Self { value } = self;
        value
    }
}
```

```txt
error[E0493]: destructor of `Wrap<T>` cannot be evaluated at compile-time
 --> test.rs:6:25
  |
6 |     const fn into_inner(self) -> T {
  |                         ^^^^ the destructor for this type cannot be evaluated in constant functions
...
9 |     }
  |     - value is dropped here
```

You get the same error without destructuring:

```rust
impl<T> Wrap<T> {
    pub const fn into_inner(self) -> T {
        self.value
    }
}
```

```txt
error[E0493]: destructor of `Wrap<T>` cannot be evaluated at compile-time
 --> test.rs:6:25
  |
6 |     const fn into_inner(self) -> T {
  |                         ^^^^ the destructor for this type cannot be evaluated in constant functions
7 |         self.value
8 |     }
  |     - value is dropped here
```
