use const_destructure::const_destructure;

struct NoCopy<T>(T);

macro_rules! compile_test {
    ($ident:ident $body:block) => {
        const _: () = {
            const fn $ident() {
                $body
            }
            $ident()
        };
    };
}

compile_test!(struct_destructure_1 {
    struct Wrap<T> {
        value: T,
    }
    const_destructure!(let Wrap { value: v } = Wrap { value: NoCopy(1) });
    assert!(matches!(v, NoCopy(1)));
});

compile_test!(struct_destructure_2 {
    struct Wrap<A, B> {
        a: A,
        b: B,
    }
    const_destructure!(let Wrap { a: a, b: b } = Wrap { a: NoCopy(1), b: NoCopy(2) });
    assert!(matches!(a, NoCopy(1)));
    assert!(matches!(b, NoCopy(2)));
});

compile_test!(tuple_destructure_1 {
    const fn f() {
        const_destructure!(let (v,) = (NoCopy(1),));
        assert!(matches!(v, NoCopy(1)));
    }
    f()
});

compile_test!(tuple_destructure_2 {
    const_destructure!(let (a, b) = (NoCopy(1), NoCopy(2)));
    assert!(matches!(a, NoCopy(1)));
    assert!(matches!(b, NoCopy(2)));
});
