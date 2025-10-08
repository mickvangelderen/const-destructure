use const_destructure::const_destructure;

struct S<A> {
    a: A,
}

fn main() {
    const_destructure!(let S { a: _, b: _ } = S { a: 1u8 });
}
