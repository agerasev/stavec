use crate::traits::{Container, Length, SizedContainer};
use core::{
    iter::ExactSizeIterator,
    marker::PhantomData,
    mem::{self, MaybeUninit},
    ops::Range,
};

/// Iterator by values of static vector.
pub struct IntoIter<T, C: Container<T> + ?Sized, L: Length> {
    _phantom: PhantomData<T>,
    range: Range<L>,
    data: C,
}

impl<T, C: SizedContainer<T>, L: Length> IntoIter<T, C, L> {
    pub(crate) fn new(data: C, range: Range<L>) -> Self {
        debug_assert!(range.end <= L::from_usize(data.as_ref().len()).unwrap());
        Self {
            _phantom: PhantomData,
            data,
            range,
        }
    }

    pub fn len(&self) -> usize {
        (self.range.end - self.range.start).to_usize().unwrap()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<T, C: Container<T> + ?Sized, L: Length> Iterator for IntoIter<T, C, L> {
    type Item = T;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }

    fn next(&mut self) -> Option<T> {
        if self.range.start < self.range.end {
            let value = unsafe {
                mem::replace(
                    self.data
                        .as_mut()
                        .get_unchecked_mut(self.range.start.to_usize().unwrap()),
                    MaybeUninit::uninit(),
                )
                .assume_init()
            };
            self.range.start += L::one();
            Some(value)
        } else {
            None
        }
    }
}

impl<T, C: Container<T> + ?Sized, L: Length> ExactSizeIterator for IntoIter<T, C, L> {}

impl<T, C: Container<T> + ?Sized, L: Length> Drop for IntoIter<T, C, L> {
    fn drop(&mut self) {
        let range = self.range.start.to_usize().unwrap()..self.range.end.to_usize().unwrap();
        unsafe {
            for x in self.data.as_mut().get_unchecked_mut(range) {
                mem::drop(mem::replace(x, MaybeUninit::uninit()).assume_init());
            }
        }
    }
}
