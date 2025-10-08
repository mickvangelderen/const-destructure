pub struct Wrap<T> {
    value: T,
}

impl<T> Wrap<T> {
    pub const fn into_inner(self) -> T {
        let Self { value } = self;
        value
    }
}

fn main() {}
