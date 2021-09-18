use core::{
    mem::{self, MaybeUninit},
    ops::Range,
};

/// Iterator by values.
pub struct IntoIter<T, const N: usize> {
    data: [MaybeUninit<T>; N],
    range: Range<usize>,
}

impl<T, const N: usize> IntoIter<T, N> {
    pub(crate) fn new(data: [MaybeUninit<T>; N], range: Range<usize>) -> Self {
        debug_assert!(range.end <= N);
        Self { data, range }
    }
}

impl<T, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.range.start < self.range.end {
            let value = unsafe {
                mem::replace(
                    self.data.get_unchecked_mut(self.range.start),
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

impl<T, const N: usize> Drop for IntoIter<T, N> {
    fn drop(&mut self) {
        unsafe {
            for x in self.data.get_unchecked_mut(self.range.clone()) {
                mem::drop(mem::replace(x, MaybeUninit::uninit()).assume_init());
            }
        }
    }
}
