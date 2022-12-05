pub fn lines(input: &str) -> impl Iterator<Item = &str> {
    input
        .lines()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
}

pub fn full_lines(input: &str) -> impl Iterator<Item = &str> {
    lines(input).filter(|line| !line.is_empty())
}

pub trait SelfExt {
    fn after<R>(self, body: impl FnOnce(Self) -> R) -> R
    where
        Self: Sized,
    {
        body(self)
    }
}

impl<T> SelfExt for T {}
