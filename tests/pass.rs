use const_destructure::const_destructure;

const fn const_destructure_struct_generic() {
    struct S<A, B> {
        a: A,
        b: B,
    }
    const_destructure!(let S { a: a_var, b: b_var } = S { a: 1u8, b: 2i16 });
    assert!(matches!(a_var, 1u8));
    assert!(matches!(b_var, 2i16));
}

const _: () = const_destructure_struct_generic();

const fn const_destructure_tuple() {
    const_destructure!(let (a, b) = (1u8, 2i16));
    assert!(matches!(a, 1u8));
    assert!(matches!(b, 2i16));
}

const _: () = const_destructure_tuple();
