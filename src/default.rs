use super::GenericVec;
use crate::{
    traits::{DefaultContainer, Length},
    utils::FullError,
};
use core::iter::IntoIterator;

impl<C: DefaultContainer, L: Length> GenericVec<C, L> {
    /// Create a new empty vector.
    pub fn new() -> Self {
        unsafe { Self::from_raw_parts(C::default(), L::zero()) }
    }
}

impl<C: DefaultContainer, L: Length> GenericVec<C, L> {
    pub fn try_from_iter<I: IntoIterator<Item = C::Item>>(iter: I) -> Result<Self, C::Item> {
        let mut self_ = Self::new();
        self_.extend_from_iter(iter.into_iter())?;
        Ok(self_)
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
        Self::try_from_slice(self.as_slice()).unwrap()
    }
}

impl<C: DefaultContainer, L: Length> GenericVec<C, L>
where
    C::Item: Clone,
{
    /// Creates a new vector with cloned elements from slice.
    ///
    /// If slice length is greater than the vector capacity then excess elements are simply ignored.
    pub fn try_from_slice(slice: &[C::Item]) -> Result<Self, FullError> {
        let mut self_ = Self::default();
        self_.extend_from_slice(slice)?;
        Ok(self_)
    }
}

impl<C: DefaultContainer, L: Length> TryFrom<&[C::Item]> for GenericVec<C, L>
where
    C::Item: Clone,
{
    type Error = FullError;

    fn try_from(slice: &[C::Item]) -> Result<Self, Self::Error> {
        Self::try_from_slice(slice)
    }
}
