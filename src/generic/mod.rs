mod maybe_sized;
mod sized;

use crate::traits::{Container, Length, SizedContainer};
use core::{marker::PhantomData, mem};

/// Fixed-capacity vector.
///
/// The type parametrized by:
/// + `T` - item type,
/// + `C` - type of the underlying container, may be unsized.
/// + `L` - type of the length of the vector, `usize` by default.
///   This would provide size optimization for small-capacity vectors, e.g. by using `u8` as length instead of `usize`.
///   Obviously, vector capacity is limited by [`L::max_value()`](`num_traits::Bounded::max_value`).
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct GenericVec<T, C: Container<T> + ?Sized, L: Length = usize> {
    _phantom: PhantomData<T>,
    len: L,
    data: C,
}

impl<T, C: SizedContainer<T>, L: Length> GenericVec<T, C, L> {
    /// Construct a vector from container and length.
    ///
    /// # Safety
    ///
    /// All `data` contents with index lower than `len` must be initialized, all other must be un-initialized.
    pub unsafe fn from_raw_parts(data: C, len: L) -> Self {
        Self {
            _phantom: PhantomData,
            len,
            data,
        }
    }

    /// Deconstruct the vector into underlying container and length.
    ///
    /// # Safety
    ///
    /// You need to manually drop items from container whose indices are lower than `len`.
    pub unsafe fn into_raw_parts(mut self) -> (C, L) {
        let ret = (mem::replace(&mut self.data, C::new_uninit()), self.len);
        mem::forget(self);
        ret
    }

    /// Create a new empty vector.
    pub fn new() -> Self {
        unsafe { Self::from_raw_parts(C::new_uninit(), L::zero()) }
    }
}
