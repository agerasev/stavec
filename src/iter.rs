use super::Container;
use core::{
    marker::PhantomData,
    mem::{self, MaybeUninit},
    ops::Range,
};

/// Iterator by values of static vector.
pub struct IntoIter<T, C: Container<T> + ?Sized> {
    _phantom: PhantomData<T>,
    range: Range<usize>,
    data: C,
}

impl<T, C: Container<T>> IntoIter<T, C> {
    pub(crate) fn new(data: C, range: Range<usize>) -> Self {
        debug_assert!(range.end <= data.as_ref().len());
        Self {
            _phantom: PhantomData,
            data,
            range,
        }
    }
}

impl<T, C: Container<T> + ?Sized> Iterator for IntoIter<T, C> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.range.start < self.range.end {
            let value = unsafe {
                mem::replace(
                    self.data.as_mut().get_unchecked_mut(self.range.start),
                    MaybeUninit::uninit(),
                )
                .assume_init()
            };
            self.range.start += 1;
            Some(value)
        } else {
            None
        }
    }
}

impl<T, C: Container<T> + ?Sized> Drop for IntoIter<T, C> {
    fn drop(&mut self) {
        unsafe {
            for x in self.data.as_mut().get_unchecked_mut(self.range.clone()) {
                mem::drop(mem::replace(x, MaybeUninit::uninit()).assume_init());
            }
        }
    }
}
