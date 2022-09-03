use super::GenericVec;
use crate::Container;
use core::{
    borrow::{Borrow, BorrowMut},
    convert::{AsMut, AsRef},
    fmt,
    hash::{Hash, Hasher},
    iter::IntoIterator,
    mem::{self, MaybeUninit},
    ops::{Deref, DerefMut, Index, IndexMut},
    ptr,
    slice::SliceIndex,
    slice::{Iter, IterMut},
};

impl<T, C: Container<T> + ?Sized> GenericVec<T, C> {
    pub fn capacity(&self) -> usize {
        self.data.as_ref().len()
    }

    /// The number of elements in the vector. Must be less or equal to `CAPACITY`.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Checks whether the vector is empty.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Checks whether the vector is full.
    pub fn is_full(&self) -> bool {
        debug_assert!(self.len <= self.capacity());
        self.len == self.capacity()
    }

    /// Appends a new element to the end of the vector *without checking whether the vector is already full*.
    ///
    /// # Safety
    ///
    /// `push_unchecked` to the vector which is full is **undefined behavior**.
    pub unsafe fn push_unchecked(&mut self, value: T) {
        let _ = mem::replace(
            self.data.as_mut().get_unchecked_mut(self.len),
            MaybeUninit::new(value),
        );
        self.len += 1;
    }

    /// Appends a new element to the end of the vector.
    ///
    /// If the vector is already full then the element is returned.
    pub fn push(&mut self, value: T) -> Result<(), T> {
        if self.is_full() {
            Err(value)
        } else {
            unsafe { self.push_unchecked(value) };
            Ok(())
        }
    }

    /// Takes a last element of the vector *without checking whether the vector is empty*.
    ///
    /// # Safety
    ///
    /// `pop_unchecked` from an empty vector is **undefined behavior**.
    pub unsafe fn pop_unchecked(&mut self) -> T {
        self.len -= 1;
        mem::replace(
            self.data.as_mut().get_unchecked_mut(self.len),
            MaybeUninit::uninit(),
        )
        .assume_init()
    }

    /// Removes and returns the last element of the vector.
    ///
    /// If the vector is empty then `None` is returned.
    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { self.pop_unchecked() })
        }
    }

    /// Truncates the vector. Excess elements are simply dropped.
    ///
    /// If `new_len` is greater then vector length the methods simply does nothing.
    pub fn truncate(&mut self, new_len: usize) {
        while self.len > new_len {
            unsafe { mem::drop(self.pop_unchecked()) };
        }
    }

    /// Removes and returns the element at position `index` within the vector,
    /// shifting all elements after it to the left.
    ///
    /// Note: Because this shifts over the remaining elements, it has a
    /// worst-case performance of *O*(*n*).
    ///
    /// # Examples
    ///
    /// ```
    /// let mut v = vec![1, 2, 3];
    /// assert_eq!(v.remove(1), 2);
    /// assert_eq!(v, [1, 3]);
    /// ```
    pub fn remove(&mut self, index: usize) -> T {
        let len = self.len();
        assert!(index < len);
        unsafe {
            let ret;
            {
                // the place we are taking from.
                let ptr = self.as_mut_ptr().add(index);
                // copy it out, unsafely having a copy of the value on
                // the stack and in the vector at the same time.
                ret = ptr::read(ptr);

                // Shift everything down to fill in that spot.
                ptr::copy(ptr.add(1), ptr, len - index - 1);
            }
            self.len -= 1;
            ret
        }
    }

    /// Removes an element from the vector and returns it.
    ///
    /// The removed element is replaced by the last element of the vector.
    ///
    /// This does not preserve ordering, but is *O*(1).
    /// If you need to preserve the element order, use `remove` instead.
    ///
    /// # Panics
    ///
    /// Panics if `index` is out of bounds.
    pub fn swap_remove(&mut self, index: usize) -> T {
        let len = self.len();
        assert!(index < len);
        unsafe {
            // We replace self[index] with the last element. Note that if the
            // bounds check above succeeds there must be a last element (which
            // can be self[index] itself).
            let value = ptr::read(self.as_ptr().add(index));
            let base_ptr = self.as_mut_ptr();
            ptr::copy(base_ptr.add(len - 1), base_ptr.add(index), 1);
            self.len -= 1;
            value
        }
    }

    /// Drops all elements in the vector and sets its length to zero.
    pub fn clear(&mut self) {
        self.truncate(0);
    }

    /// Slice of the vector content.
    pub fn as_slice(&self) -> &[T] {
        unsafe { ptr::read(&(&self.data.as_ref()[..self.len]) as *const _ as *const &[T]) }
    }

    /// Mutable slice of the vector content.
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { ptr::read(&(&mut self.data.as_mut()[..self.len]) as *const _ as *const &mut [T]) }
    }

    /// Appends elements from iterator to the vector until iterator ends or the vector is full.
    pub fn extend_from_iter<I: Iterator<Item = T>>(&mut self, iter: I) {
        for (x, _) in iter.zip(self.len..self.capacity()) {
            unsafe { self.push_unchecked(x) };
        }
    }

    /// Returns iterator over references of vector elements.
    pub fn iter(&self) -> Iter<T> {
        self.as_slice().iter()
    }

    /// Returns iterator over mutable references of vector elements.
    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.as_mut_slice().iter_mut()
    }
}

impl<T, C: Container<T> + ?Sized> Drop for GenericVec<T, C> {
    fn drop(&mut self) {
        for i in 0..self.len {
            unsafe {
                mem::drop(
                    mem::replace(
                        self.data.as_mut().get_unchecked_mut(i),
                        MaybeUninit::uninit(),
                    )
                    .assume_init(),
                );
            }
        }
    }
}

impl<'a, T: 'a, C: Container<T> + ?Sized> IntoIterator for &'a GenericVec<T, C> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T: 'a, C: Container<T> + ?Sized> IntoIterator for &'a mut GenericVec<T, C> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T, I, C: Container<T> + ?Sized> Index<I> for GenericVec<T, C>
where
    I: SliceIndex<[T]>,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<T, I, C: Container<T> + ?Sized> IndexMut<I> for GenericVec<T, C>
where
    I: SliceIndex<[T]>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.as_mut_slice()[index]
    }
}

impl<T: Clone, C: Container<T>> GenericVec<T, C> {
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
        if new_len <= self.len {
            self.truncate(new_len);
        } else {
            assert!(new_len <= self.capacity());
            for _ in self.len..new_len {
                unsafe { self.push_unchecked(value.clone()) };
            }
        }
    }
}

impl<T, C: Container<T> + ?Sized> Hash for GenericVec<T, C>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}

impl<T, C: Container<T> + ?Sized> fmt::Debug for GenericVec<T, C>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.as_slice().fmt(f)
    }
}

impl<T, C: Container<T> + ?Sized> Deref for GenericVec<T, C> {
    type Target = [T];

    fn deref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, C: Container<T> + ?Sized> DerefMut for GenericVec<T, C> {
    fn deref_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T, C: Container<T> + ?Sized> AsRef<GenericVec<T, C>> for GenericVec<T, C> {
    fn as_ref(&self) -> &GenericVec<T, C> {
        self
    }
}

impl<T, C: Container<T> + ?Sized> AsRef<[T]> for GenericVec<T, C> {
    fn as_ref(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, C: Container<T> + ?Sized> AsMut<GenericVec<T, C>> for GenericVec<T, C> {
    fn as_mut(&mut self) -> &mut GenericVec<T, C> {
        self
    }
}

impl<T, C: Container<T> + ?Sized> AsMut<[T]> for GenericVec<T, C> {
    fn as_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}

impl<T, C: Container<T> + ?Sized> Borrow<[T]> for GenericVec<T, C> {
    fn borrow(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T, C: Container<T> + ?Sized> BorrowMut<[T]> for GenericVec<T, C> {
    fn borrow_mut(&mut self) -> &mut [T] {
        self.as_mut_slice()
    }
}
