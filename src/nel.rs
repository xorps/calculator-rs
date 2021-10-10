#[derive(Eq, PartialEq, Debug)]
pub struct Nel<T> {
    pub head: T,
    pub tail: Vec<T>,
}

impl<T> Nel<T> {
    pub fn new(head: T) -> Self {
        let tail = Vec::new();
        Self {head, tail}
    }
    pub fn push(mut self, c: T) -> Self {
        self.tail.push(c);
        self
    }
}