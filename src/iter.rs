use crate::traits::{Container, Length, Slot};
use core::{iter::ExactSizeIterator, mem, ops::Range};

/// Iterator by values of static vector.
pub struct IntoIter<C: Container + ?Sized, L: Length> {
    range: Range<L>,
    data: C,
}

impl<C: Container, L: Length> IntoIter<C, L> {
    pub(crate) fn new(data: C, range: Range<L>) -> Self {
        debug_assert!(range.end <= L::from_usize(data.as_ref().len()).unwrap());
        Self { data, range }
    }

    pub fn len(&self) -> usize {
        (self.range.end - self.range.start).to_usize().unwrap()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<C: Container + ?Sized, L: Length> Iterator for IntoIter<C, L> {
    type Item = C::Item;

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.len(), Some(self.len()))
    }

    fn next(&mut self) -> Option<C::Item> {
        if self.range.start < self.range.end {
            let value = unsafe {
                self.data
                    .as_mut()
                    .get_unchecked_mut(self.range.start.to_usize().unwrap())
                    .assume_init_read()
            };
            self.range.start += L::one();
            Some(value)
        } else {
            None
        }
    }
}

impl<C: Container + ?Sized, L: Length> ExactSizeIterator for IntoIter<C, L> {}

impl<C: Container + ?Sized, L: Length> Drop for IntoIter<C, L> {
    fn drop(&mut self) {
        let range = self.range.start.to_usize().unwrap()..self.range.end.to_usize().unwrap();
        unsafe {
            for x in self.data.as_mut().get_unchecked_mut(range) {
                mem::drop(x.assume_init_read());
            }
        }
    }
}
