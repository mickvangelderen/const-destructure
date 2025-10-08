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

compile_test!(struct_destructure_0 {
    struct Wrap {}
    const_destructure!(let Wrap { } = Wrap { });
});

compile_test!(struct_destructure_1 {
    struct Wrap<T> {
        value: T,
    }

    const_destructure!(let Wrap { value } = Wrap { value: NoCopy(1) });
    assert!(matches!(value, NoCopy(1)));

    const_destructure!(let Wrap { value, } = Wrap { value: NoCopy(1) });
    assert!(matches!(value, NoCopy(1)));

    const_destructure!(let Wrap { value: v } = Wrap { value: NoCopy(1) });
    assert!(matches!(v, NoCopy(1)));

    const_destructure!(let Wrap { value: v, } = Wrap { value: NoCopy(1) });
    assert!(matches!(v, NoCopy(1)));

    #[allow(unused_mut)]
    {
        const_destructure!(let Wrap { value: mut v } = Wrap { value: NoCopy(1) });
        assert!(matches!(v, NoCopy(1)));
        v.0 = 2
    }

    #[allow(unused_mut)]
    {
        const_destructure!(let Wrap { value: mut v, } = Wrap { value: NoCopy(1) });
        assert!(matches!(v, NoCopy(1)));
        v.0 = 2
    }
});

compile_test!(struct_destructure_2 {
    struct Wrap<A, B> {
        a: A,
        b: B,
    }

    const_destructure!(let Wrap { a: a_bound, b: b_bound } = Wrap { a: NoCopy(1), b: NoCopy(2) });
    assert!(matches!(a_bound, NoCopy(1)));
    assert!(matches!(b_bound, NoCopy(2)));

    const_destructure!(let Wrap { a, b: b_bound } = Wrap { a: NoCopy(1), b: NoCopy(2) });
    assert!(matches!(a, NoCopy(1)));
    assert!(matches!(b_bound, NoCopy(2)));

    const_destructure!(let Wrap { a: a_bound, b } = Wrap { a: NoCopy(1), b: NoCopy(2) });
    assert!(matches!(a_bound, NoCopy(1)));
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
