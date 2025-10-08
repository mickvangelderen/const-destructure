use const_destructure::const_destructure;

struct NoCopy;

struct Wrap<T> {
    value: T,
}

fn main() {
    const_destructure!(let Wrap { value: _, unknown: _ } = Wrap { value: NoCopy });
}
