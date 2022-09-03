use super::GenericVec;
use crate::{
    traits::{Length, SizedContainer},
    IntoIter,
};
use core::iter::{FromIterator, IntoIterator};

impl<T, C: SizedContainer<T>, L: Length> FromIterator<T> for GenericVec<T, C, L> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut self_ = Self::new();
        self_.extend_from_iter(iter.into_iter());
        self_
    }
}

impl<T, C: SizedContainer<T>, L: Length> IntoIterator for GenericVec<T, C, L> {
    type Item = T;
    type IntoIter = IntoIter<T, C, L>;

    fn into_iter(self) -> Self::IntoIter {
        let (data, len) = unsafe { self.into_raw_parts() };
        IntoIter::new(data, L::zero()..len)
    }
}

impl<T, C: SizedContainer<T>, L: Length> Default for GenericVec<T, C, L> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone, C: SizedContainer<T>, L: Length> Clone for GenericVec<T, C, L> {
    fn clone(&self) -> Self {
        Self::from_iter(self.iter().cloned())
    }
}

impl<T: Clone, C: SizedContainer<T>, L: Length> GenericVec<T, C, L> {
    /// Creates a new vector with cloned elements from slice.
    ///
    /// If slice length is greater than the vector capacity then excess elements are simply ignored.
    pub fn from_slice(slice: &[T]) -> Self {
        Self::from_iter(slice.iter().cloned())
    }
}

impl<T, C: SizedContainer<T>, L: Length> From<&[T]> for GenericVec<T, C, L>
where
    T: Clone,
{
    fn from(slice: &[T]) -> Self {
        Self::from_slice(slice)
    }
}

impl<T, C: SizedContainer<T>, L: Length> From<&mut [T]> for GenericVec<T, C, L>
where
    T: Clone,
{
    fn from(slice: &mut [T]) -> Self {
        Self::from_slice(slice)
    }
}
