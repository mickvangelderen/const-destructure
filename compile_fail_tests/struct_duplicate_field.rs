use const_destructure::const_destructure;

struct Wrap<T> {
    value: T,
}

impl<T> Wrap<T> {
    const fn test(self) -> T {
        const_destructure!(let Wrap { value: v1, value: v2 } = self);
        v1
    }
}

fn main() {}
