use super::GenericVec;
use crate::{IntoIter, SizedContainer};
use core::{
    iter::{FromIterator, IntoIterator},
    marker::PhantomData,
    mem,
};

impl<T, C: SizedContainer<T>> GenericVec<T, C> {
    pub unsafe fn from_raw_parts(data: C, len: usize) -> Self {
        Self {
            _phantom: PhantomData,
            len,
            data,
        }
    }

    pub unsafe fn into_raw_parts(mut self) -> (C, usize) {
        let ret = (mem::replace(&mut self.data, C::new_uninit()), self.len);
        mem::forget(self);
        ret
    }

    pub fn new() -> Self {
        unsafe { Self::from_raw_parts(C::new_uninit(), 0) }
    }
}

impl<T, C: SizedContainer<T>> FromIterator<T> for GenericVec<T, C> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut self_ = Self::new();
        self_.extend_from_iter(iter.into_iter());
        self_
    }
}

impl<T, C: SizedContainer<T>> IntoIterator for GenericVec<T, C> {
    type Item = T;
    type IntoIter = IntoIter<T, C>;

    fn into_iter(self) -> Self::IntoIter {
        let (data, len) = unsafe { self.into_raw_parts() };
        IntoIter::new(data, 0..len)
    }
}

impl<T, C: SizedContainer<T>> Default for GenericVec<T, C> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Clone, C: SizedContainer<T>> Clone for GenericVec<T, C> {
    fn clone(&self) -> Self {
        Self::from_iter(self.iter().cloned())
    }
}

impl<T: Clone, C: SizedContainer<T>> GenericVec<T, C> {
    /// Creates a new vector with cloned elements from slice.
    ///
    /// If slice length is greater than the vector capacity then excess elements are simply ignored.
    pub fn from_slice(slice: &[T]) -> Self {
        Self::from_iter(slice.iter().cloned())
    }
}

impl<T, C: SizedContainer<T>> From<&[T]> for GenericVec<T, C>
where
    T: Clone,
{
    fn from(slice: &[T]) -> Self {
        Self::from_slice(slice)
    }
}

impl<T, C: SizedContainer<T>> From<&mut [T]> for GenericVec<T, C>
where
    T: Clone,
{
    fn from(slice: &mut [T]) -> Self {
        Self::from_slice(slice)
    }
}
