struct Wrap<T> {
    value: T,
}

const fn test<T>(wrap: Wrap<T>) -> T {
    wrap.value
}

fn main() {}
