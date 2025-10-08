use const_destructure::const_destructure;

struct NoCopy;

struct Wrap<T> {
    value: T,
}

fn main() {
    const_destructure!(let Wrap { value: v1, value: v2 } = Wrap { value: NoCopy });
}
