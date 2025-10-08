use const_destructure::const_destructure;

struct S<A, B> {
    a: A,
    b: B,
}

fn main() {
    const_destructure!(let S { a: _ } = S { a: 1u8, b: 2i16 });
}
