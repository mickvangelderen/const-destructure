use const_destructure::const_destructure;

struct Wrap<T> {
    value: T,
}

impl<T> Wrap<T> {
    const fn test(self) {
        const_destructure!(let Wrap { .. } = self);
    }
}

fn main() {}
