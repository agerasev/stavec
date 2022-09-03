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

impl<T: Clone, C: SizedContainer<T>, L: Length> GenericVec<T, C, L> {
    /// Appends elements from slice to the vector cloning them.
    ///
    /// If slice length is greater than free space in the vector then excess elements are simply ignored.
    pub fn extend_from_slice(&mut self, slice: &[T]) {
        self.extend_from_iter(slice.iter().cloned());
    }

    /// Resizes the vector to the specified length.
    ///
    /// If `new_len` is less than vector length the the vector is truncated.
    ///
    /// If `new_len` is greater than the vector length then vector is filled with `value` up to `new_len` length.
    ///
    /// *Panics if `new_len` is greater than the vector capacity.*
    pub fn resize(&mut self, new_len: usize, value: T) {
        if new_len <= self.len() {
            self.truncate(new_len);
        } else {
            assert!(new_len <= self.capacity());
            for _ in self.len()..new_len {
                unsafe { self.push_unchecked(value.clone()) };
            }
        }
    }
}

impl<T, C: SizedContainer<T>, L: Length> IntoIterator for GenericVec<T, C, L> {
    type Item = T;
    type IntoIter = IntoIter<T, C>;

    fn into_iter(self) -> Self::IntoIter {
        let (data, len) = unsafe { self.into_raw_parts() };
        IntoIter::new(data, 0..len.to_usize().unwrap())
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
