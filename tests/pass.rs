use const_destructure::const_destructure;

struct S0 {}

impl S0 {
    #[allow(unused)]
    const fn destructure(self) {
        let Self {} = self;
    }
}

struct S1<F0> {
    f0: F0,
}

impl<F0> S1<F0> {
    #[allow(unused)]
    const fn destructure_implicit(self) -> F0 {
        const_destructure!(let Self { f0 } = self);
        f0
    }

    #[allow(unused)]
    const fn destructure_explicit(self) -> F0 {
        const_destructure!(let Self { f0: f0_bound } = self);
        f0_bound
    }

    #[allow(unused)]
    const fn destructure_explicit_mut(self) -> F0 {
        #![allow(unused_mut)]
        const_destructure!(let Self { f0: mut f0 } = self);
        f0
    }

    #[allow(unused)]
    const fn destructure_implicit_trailing_comma(self) -> F0 {
        const_destructure!(let Self { f0, } = self);
        f0
    }

    #[allow(unused)]
    const fn destructure_explicit_trailing_comma(self) -> F0 {
        const_destructure!(let Self { f0: f0_bound, } = self);
        f0_bound
    }

    #[allow(unused)]
    const fn destructure_explicit_mut_trailing_comma(self) -> F0 {
        #![allow(unused_mut)]
        const_destructure!(let Self { f0: mut f0, } = self);
        f0
    }
}

struct S2<F0, F1> {
    f0: F0,
    f1: F1,
}

impl<F0, F1> S2<F0, F1> {
    #[allow(unused)]
    const fn destructure_implicit(self) -> (F0, F1) {
        const_destructure!(let Self { f0, f1 } = self);
        (f0, f1)
    }

    #[allow(unused)]
    const fn destructure_explicit_1(self) -> (F0, F1) {
        const_destructure!(let Self { f0: f0_bound, f1 } = self);
        (f0_bound, f1)
    }

    #[allow(unused)]
    const fn destructure_explicit_2(self) -> (F0, F1) {
        const_destructure!(let Self { f0, f1: f1_bound } = self);
        (f0, f1_bound)
    }

    #[allow(unused)]
    const fn destructure_explicit_mut(self) -> (F0, F1) {
        #![allow(unused_mut)]
        const_destructure!(let Self { f0: mut f0, f1: mut f1 } = self);
        (f0, f1)
    }

    #[allow(unused)]
    const fn destructure_implicit_trailing_comma(self) -> (F0, F1) {
        const_destructure!(let Self { f0, f1, } = self);
        (f0, f1)
    }

    #[allow(unused)]
    const fn destructure_explicit_1_trailing_comma(self) -> (F0, F1) {
        const_destructure!(let Self { f0: f0_bound, f1, } = self);
        (f0_bound, f1)
    }

    #[allow(unused)]
    const fn destructure_explicit_2_trailing_comma(self) -> (F0, F1) {
        const_destructure!(let Self { f0, f1: f1_bound, } = self);
        (f0, f1_bound)
    }

    #[allow(unused)]
    const fn destructure_explicit_mut_trailing_comma(self) -> (F0, F1) {
        #![allow(unused_mut)]
        const_destructure!(let Self { f0: mut f0, f1: mut f1, } = self);
        (f0, f1)
    }
}
