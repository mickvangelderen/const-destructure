use const_destructure::const_destructure;

struct S<A> {
    a: A,
}

fn main() {
    const_destructure!(let S { a: a1, a: a2 } = S { a: 1u8 });
}
