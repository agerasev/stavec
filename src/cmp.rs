use crate::{
    traits::{Container, Length},
    GenericVec,
};
use core::cmp::Ordering;

impl<T: PartialOrd, C: Container<T> + ?Sized, L: Length> PartialOrd for GenericVec<T, C, L> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl<T: Ord, C: Container<T> + ?Sized, L: Length> Ord for GenericVec<T, C, L> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl<T: PartialEq, C: Container<T> + ?Sized, L: Length> PartialEq for GenericVec<T, C, L> {
    fn eq(&self, other: &Self) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl<T, C: Container<T> + ?Sized, L: Length> Eq for GenericVec<T, C, L> where T: Eq {}

impl<T: PartialEq, const M: usize, C: Container<T> + ?Sized, L: Length> PartialEq<[T; M]>
    for GenericVec<T, C, L>
{
    fn eq(&self, other: &[T; M]) -> bool {
        self.as_slice().eq(&other[..])
    }
}

impl<T: PartialEq, C: Container<T> + ?Sized, L: Length> PartialEq<[T]> for GenericVec<T, C, L> {
    fn eq(&self, other: &[T]) -> bool {
        self.as_slice().eq(other)
    }
}

impl<T: PartialEq, C: Container<T> + ?Sized, L: Length> PartialEq<&[T]> for GenericVec<T, C, L> {
    fn eq(&self, other: &&[T]) -> bool {
        self.as_slice().eq(*other)
    }
}

impl<T: PartialEq, C: Container<T> + ?Sized, L: Length> PartialEq<&mut [T]>
    for GenericVec<T, C, L>
{
    fn eq(&self, other: &&mut [T]) -> bool {
        self.as_slice().eq(*other)
    }
}

#[cfg(feature = "std")]
impl<T: PartialEq, C: Container<T> + ?Sized, L: Length> PartialEq<std::vec::Vec<T>>
    for GenericVec<T, C, L>
{
    fn eq(&self, other: &std::vec::Vec<T>) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}
