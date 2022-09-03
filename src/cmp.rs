use crate::{traits::Container, GenericVec};
use core::cmp::Ordering;

impl<T, C: Container<T> + ?Sized> PartialOrd for GenericVec<T, C>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl<T, C: Container<T> + ?Sized> Ord for GenericVec<T, C>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl<T, C: Container<T> + ?Sized> PartialEq for GenericVec<T, C>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl<T, C: Container<T> + ?Sized> Eq for GenericVec<T, C> where T: Eq {}

impl<T, const M: usize, C: Container<T> + ?Sized> PartialEq<[T; M]> for GenericVec<T, C>
where
    T: PartialEq,
{
    fn eq(&self, other: &[T; M]) -> bool {
        self.as_slice().eq(&other[..])
    }
}

impl<T, C: Container<T> + ?Sized> PartialEq<[T]> for GenericVec<T, C>
where
    T: PartialEq,
{
    fn eq(&self, other: &[T]) -> bool {
        self.as_slice().eq(other)
    }
}

impl<T, C: Container<T> + ?Sized> PartialEq<&[T]> for GenericVec<T, C>
where
    T: PartialEq,
{
    fn eq(&self, other: &&[T]) -> bool {
        self.as_slice().eq(*other)
    }
}

impl<T, C: Container<T> + ?Sized> PartialEq<&mut [T]> for GenericVec<T, C>
where
    T: PartialEq,
{
    fn eq(&self, other: &&mut [T]) -> bool {
        self.as_slice().eq(*other)
    }
}

#[cfg(feature = "std")]
impl<T, C: Container<T> + ?Sized> PartialEq<std::vec::Vec<T>> for GenericVec<T, C>
where
    T: PartialEq,
{
    fn eq(&self, other: &std::vec::Vec<T>) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}
