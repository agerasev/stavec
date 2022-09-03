mod maybe_sized;
mod sized;

use crate::traits::{Container, Length, SizedContainer};
use core::{marker::PhantomData, mem};

pub struct GenericVec<T, C: Container<T> + ?Sized, L: Length = usize> {
    _phantom: PhantomData<T>,
    len: L,
    data: C,
}

impl<T, C: SizedContainer<T>, L: Length> GenericVec<T, C, L> {
    pub unsafe fn from_raw_parts(data: C, len: L) -> Self {
        Self {
            _phantom: PhantomData,
            len,
            data,
        }
    }

    pub unsafe fn into_raw_parts(mut self) -> (C, L) {
        let ret = (mem::replace(&mut self.data, C::new_uninit()), self.len);
        mem::forget(self);
        ret
    }

    pub fn new() -> Self {
        unsafe { Self::from_raw_parts(C::new_uninit(), L::zero()) }
    }
}
