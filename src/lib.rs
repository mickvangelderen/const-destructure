#[doc(hidden)]
/// Provides access to the inner value of a ManuallyDrop<T>.
pub const fn __manually_drop_inner_ref<T>(slot: &core::mem::ManuallyDrop<T>) -> &T {
    // SAFETY: ManuallyDrop<T> and T are guaranteed to have the same layout
    unsafe { core::mem::transmute(slot) }
}

#[macro_export]
macro_rules! const_destructure_struct {
    (let $S:ident { $($field:ident: $var:ident),* } = $value:expr) => {
        let value = $value;
        let __destructures_all_fields_and_fields_are_unique = || {
            let $S { $($field: _),* } = &value;
        };
        let value = ::core::mem::ManuallyDrop::new($value);
        let value = $crate::__manually_drop_inner_ref(&value);
        // SAFETY: We avoid double free by 1) only reading each field once (since they're unique) and 2) the original is wrapped in ManuallyDrop.
        $(
            let $var = unsafe { ::core::ptr::addr_of!(value.$field).read() };
        )*
    }
}

#[macro_export]
macro_rules! const_destructure_tuple {
    (let ($($var:ident),*) = $value:expr) => {
        $crate::const_destructure_tuple!(@impl ($($var),*); (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11) => (); let () = $value);
    };
    (@impl (); ($($index_rest:tt),*) => ($($ty:tt),*); let ($($index:tt: $var:ident),*) = $value:expr) => {
        let value: ($($ty,)*) = $value; // asserts correct arity
        let value = ::core::mem::ManuallyDrop::new(value);
        let value = $crate::__manually_drop_inner_ref(&value);
        // SAFETY: We avoid double free by 1) only reading each field once (since they're unique) and 2) the original is wrapped in ManuallyDrop.
        $(
            let $var = unsafe { ::core::ptr::addr_of!(value.$index).read() };
        )*
    };
    (@impl ($var_head:ident $(,$var_tail:ident)*); () => ($($ty:tt),*); let ($($index:tt: $var:ident),*) = $value:expr) => {
        compile_error!("tuple arity is larger than the maximum supported arity 12")
    };
    (@impl ($var_head:ident $(,$var_tail:ident)*); ($index_head:tt $(,$index_tail:tt)*) => ($($ty:tt),*); let ($($index:tt: $var:ident),*) = $value:expr) => {
        $crate::const_destructure_tuple!(@impl ($($var_tail),*); ($($index_tail),*) => ($($ty,)* _); let ($($index: $var,)* $index_head: $var_head) = $value);
    };
}

// const fn s() {
//     struct S<A, B>{ a: A, b: B}
//     const_destructure_struct!(let S { a: da, b: db } = S { a: 1, b: [0u8; 1] });
// }

// const fn x() {
//     let v = (1, 2);
//     const_destructure_tuple!(let (t0, t1) = v);
//     if t0 != 1 || t1 != 2 {
//         panic!()
//     }
// }
// const X: () = x();

// const fn x13() {
//     let v = (0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 11);
//     const_destructure_tuple!(let (t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12) = v);
// }

// const X13: () = x13();

// const fn too_many() {
//     let v = (0, 1);
//     const_destructure_tuple!(let (t0, t1, t2) = v);
// }

// const fn too_few() {
//     let v = (0, 1);
//     const_destructure_tuple!(let (t0) = v);
// }

// macro_rules! const_map {
//     ($N:tt => ($($n:ident in $a:expr),+ $(,)?) -> ($($o:ident $ot:ty => $oa:ident),+ $(,)?) $body:expr) => {{
//         $(
//             let mut $o = array_uninit::<$ot, $N>();
//         )*

//         $(
//             #[allow(clippy::redundant_locals)]
//             let $n: [_; $N] = $a;
//             let $n = core::mem::ManuallyDrop::new($n);
//             let $n = manually_drop_inner_ref(&$n);
//         )*

//         const_for!{index in 0..$N => {
//             $(
//                 let $n = unsafe { core::ptr::addr_of!($n[index]).read() };
//             )*
//             let item: ($($ot),*) = $body;
//             const_destructure!(let $($o: ))
//             $(
//                 $o[index].write()
//             )
//             out[index].write(item);
//         }};

//         unsafe { array_assume_init(out) }
//     }};
// }
// pub(crate) use const_map;
