use super::GenericVec;
use crate::{
    traits::{Container, Length},
    IntoIter,
};
use core::{iter::IntoIterator, mem::ManuallyDrop, ptr};

impl<C: Container, L: Length> GenericVec<C, L> {
    pub fn from_empty(data: C) -> Self {
        unsafe { Self::from_raw_parts(data, L::zero()) }
    }

    /// Construct a vector from container and length.
    ///
    /// # Safety
    ///
    /// All `data` contents with index lower than `len` must be initialized, all other must be un-initialized.
    pub unsafe fn from_raw_parts(data: C, len: L) -> Self {
        Self { len, data }
    }

    /// Deconstruct the vector into underlying container and length.
    ///
    /// # Safety
    ///
    /// You need to manually drop items from container whose indices are lower than `len`.
    pub unsafe fn into_raw_parts(self) -> (C, L) {
        let self_ = ManuallyDrop::new(self);
        (ptr::read(&self_.data as *const _), self_.len)
    }
}

impl<C: Container, L: Length> IntoIterator for GenericVec<C, L> {
    type Item = C::Item;
    type IntoIter = IntoIter<C, L>;

    fn into_iter(self) -> Self::IntoIter {
        let (data, len) = unsafe { self.into_raw_parts() };
        IntoIter::new(data, L::zero()..len)
    }
}
