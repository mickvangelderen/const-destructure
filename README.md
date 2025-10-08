This crate provides a macro that allows you to destructure types in a `const` context:

```rust
// A wrapper that does not implement `Copy` to enforce values are moved.
struct NoCopy<T>(T);

// Some struct we'd like to break up in a const fn.
struct Wrap<A, B> {
    a: A,
    b: B,
}

const fn test() {
    const_destructure!(let Wrap { a: a, b: b } = Wrap { a: NoCopy(1), b: NoCopy(2) });

    assert!(matches!(a, NoCopy(1)));
    assert!(matches!(b, NoCopy(2)));
}
```

Unlike normal destructuring, the macro enforces that you assign all fields.
If it didn't, it would be possible to accidentally leak unassigned fields because of how the macro is implemented. 
This is still a big win over writing the implementation by hand, which requires `unsafe`.

## Motivation

The following doesn't compile on rustc 1.90.0 (2025) due to https://github.com/rust-lang/rust/issues/86897:

```rust
pub struct Wrap<T> {
    value: T,
}

impl<T> Wrap<T> {
    pub const fn into_inner(self) -> T {
        let Self { value } = self;
        value
    }
}
```

```txt
error[E0493]: destructor of `Wrap<T>` cannot be evaluated at compile-time
 --> test.rs:6:29
  |
6 |     pub const fn into_inner(self) -> T {
  |                             ^^^^ the destructor for this type cannot be evaluated in constant functions
7 |         self.value
8 |     }
  |     - value is dropped here

error: aborting due to 1 previous error

For more information about this error, try `rustc --explain E0493`.
```

You get the same error without destructuring:

```rust
impl<T> Wrap<T> {
    pub const fn into_inner(self) -> T {
        self.value
    }
}
```
