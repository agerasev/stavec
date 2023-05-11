use super::GenericVec;
use crate::traits::{DefaultContainer, Length};
use core::iter::{FromIterator, IntoIterator};

impl<C: DefaultContainer, L: Length> GenericVec<C, L> {
    /// Create a new empty vector.
    pub fn new() -> Self {
        unsafe { Self::from_raw_parts(C::default(), L::zero()) }
    }
}

impl<C: DefaultContainer, L: Length> FromIterator<C::Item> for GenericVec<C, L> {
    fn from_iter<I: IntoIterator<Item = C::Item>>(iter: I) -> Self {
        let mut self_ = Self::new();
        self_.extend_from_iter(iter.into_iter());
        self_
    }
}

impl<C: DefaultContainer, L: Length> Default for GenericVec<C, L> {
    fn default() -> Self {
        Self::new()
    }
}

impl<C: DefaultContainer, L: Length> Clone for GenericVec<C, L>
where
    C::Item: Clone,
{
    fn clone(&self) -> Self {
        Self::from_iter(self.iter().cloned())
    }
}

impl<C: DefaultContainer, L: Length> GenericVec<C, L>
where
    C::Item: Clone,
{
    /// Creates a new vector with cloned elements from slice.
    ///
    /// If slice length is greater than the vector capacity then excess elements are simply ignored.
    pub fn from_slice(slice: &[C::Item]) -> Self {
        Self::from_iter(slice.iter().cloned())
    }
}

impl<C: DefaultContainer, L: Length> From<&[C::Item]> for GenericVec<C, L>
where
    C::Item: Clone,
{
    fn from(slice: &[C::Item]) -> Self {
        Self::from_slice(slice)
    }
}

impl<C: DefaultContainer, L: Length> From<&mut [C::Item]> for GenericVec<C, L>
where
    C::Item: Clone,
{
    fn from(slice: &mut [C::Item]) -> Self {
        Self::from_slice(slice)
    }
}
