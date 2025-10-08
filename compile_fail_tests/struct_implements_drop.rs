use const_destructure::const_destructure;

struct Wrap<T> {
    value: T,
}

impl<T> Drop for Wrap<T> {
    fn drop(&mut self) {}
}

impl<T> Wrap<T> {
    const fn test(self) -> T {
        const_destructure!(let Wrap { value } = self);
        value
    }
}

fn main() {}
