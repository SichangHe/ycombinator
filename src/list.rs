#[derive(Debug, Clone)]
pub struct Cons<T> {
    pub car: T,
    pub cdr: Option<Box<Cons<T>>>,
}

impl<T> Cons<T> {
    pub fn from<I>(value: I) -> Option<Self>
    where
        I: IntoIterator<Item = T>,
        <I as IntoIterator>::IntoIter: DoubleEndedIterator,
    {
        value.into_iter().rev().fold(None, |acc, x| {
            Some(Self {
                car: x,
                cdr: acc.map(Box::new),
            })
        })
    }
}
