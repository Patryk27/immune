pub enum Poll<T> {
    Ready(T),
    Pending,
}

impl<T> Poll<T> {
    pub fn map<U>(self, f: impl FnOnce(T) -> U) -> Poll<U> {
        match self {
            Poll::Ready(val) => Poll::Ready(f(val)),
            Poll::Pending => Poll::Pending,
        }
    }
}
