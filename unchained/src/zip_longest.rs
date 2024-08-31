//! Tools to handle zipping two iterators
//! and continuing until both are exhausted.
//! Inspired by [itertools::zip_longest](https://doc.servo.org/itertools/zip_longest/index.html)

/// Struct where the next element is an option tuple
/// with two optional elements.
pub struct ZipLongestIter<T, U> {
    a: T,
    b: U,
}

impl<T, U> Iterator for ZipLongestIter<T, U>
where
    T: Iterator,
    U: Iterator,
{
    type Item = (Option<T::Item>, Option<U::Item>);

    /// Returns the next element in the iterator.
    /// Returns Some as long as one element is available.
    /// If both iterators are exhausted, returns None.
    fn next(&mut self) -> Option<Self::Item> {
        match (self.a.next(), self.b.next()) {
            (None, None) => None,
            (a, b) => Some((a, b)),
        }
    }
}

pub trait ZipLongest {
    fn zip_longest<U>(self, other: U) -> ZipLongestIter<Self, U>
    where
        Self: Sized,
        U: IntoIterator;
}
impl<T> ZipLongest for T
where
    T: Iterator,
{
    fn zip_longest<U>(self, other: U) -> ZipLongestIter<Self, U>
    where
        U: IntoIterator,
    {
        ZipLongestIter { a: self, b: other }
    }
}
