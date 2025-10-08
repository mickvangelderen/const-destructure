struct Wrap<T> {
    value: T,
}

impl<T> Wrap<T> {
    const fn test(self) -> T {
        self.value
    }
}

fn main() {}
