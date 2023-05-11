use crate::{
    traits::{Container, Length, Slot},
    utils::{occupy_slice_cloned, slice_assume_occupied_mut, slice_assume_occupied_ref},
};
use core::{
    borrow::{Borrow, BorrowMut},
    convert::{AsMut, AsRef},
    fmt,
    hash::{Hash, Hasher},
    iter::{Extend, IntoIterator},
    mem,
    ops::{Deref, DerefMut, Index, IndexMut},
    ptr,
    slice::SliceIndex,
    slice::{Iter, IterMut},
};
use num_traits::clamp_max;

/// Fixed-capacity vector.
///
/// The type parametrized by:
/// + `C::Item` - item type,
/// + `C` - type of the underlying container, may be unsized.
/// + `L` - type of the length of the vector, `usize` by default.
///   This would provide size optimization for small-capacity vectors, e.g. by using `u8` as length instead of `usize`.
///   Obviously, vector capacity is limited by [`L::max_value()`](`num_traits::Bounded::max_value`).
#[cfg_attr(feature = "repr-c", repr(C))]
pub struct GenericVec<C: Container + ?Sized, L: Length = usize> {
    pub(crate) len: L,
    pub(crate) data: C,
}

impl<C: Container + ?Sized, L: Length> GenericVec<C, L> {
    /// Provides an access to underlying container.
    pub fn data(&self) -> &C {
        &self.data
    }

    /// Provides a mutable access to underlying container.
    ///
    /// # Safety
    ///
    /// Items with indices lower than [`len()`](`Self::len`) must remain initialized, other items must remain un-initialized.
    pub unsafe fn data_mut(&mut self) -> &mut C {
        &mut self.data
    }
}

impl<C: Container + ?Sized, L: Length> GenericVec<C, L> {
    pub fn capacity(&self) -> usize {
        clamp_max(self.data.as_ref().len(), L::max_value().to_usize().unwrap())
    }

    /// The number of elements in the vector. Must be less or equal to [`capacity()`](`Self::capacity`).
    pub fn len(&self) -> usize {
        self.len.to_usize().unwrap()
    }

    /// Number of remaining free places in the vector.
    pub fn remaining(&self) -> usize {
        self.capacity() - self.len.to_usize().unwrap()
    }

    /// Checks whether the vector is empty.
    pub fn is_empty(&self) -> bool {
        self.len.is_zero()
    }

    /// Checks whether the vector is full.
    pub fn is_full(&self) -> bool {
        debug_assert!(self.len() <= self.capacity());
        self.len() == self.capacity()
    }

    /// Appends a new element to the end of the vector *without checking whether the vector is already full*.
    ///
    /// # Safety
    ///
    /// `push_unchecked` to the vector which is full is **undefined behavior**.
    pub unsafe fn push_unchecked(&mut self, value: C::Item) {
        let len = self.len();
        let _ = mem::replace(
            self.data.as_mut().get_unchecked_mut(len),
            C::Slot::occupied(value),
        );
        self.len += L::one();
    }

    /// Appends a new element to the end of the vector.
    ///
    /// If the vector is already full then the element is returned.
    pub fn push(&mut self, value: C::Item) -> Result<(), C::Item> {
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
    pub unsafe fn pop_unchecked(&mut self) -> C::Item {
        self.len -= L::one();
        let len = self.len();
        mem::replace(self.data.as_mut().get_unchecked_mut(len), C::Slot::empty()).assume_occupied()
    }

    /// Removes and returns the last element of the vector.
    ///
    /// If the vector is empty then `None` is returned.
    pub fn pop(&mut self) -> Option<C::Item> {
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
        while self.len() > new_len {
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
    pub fn remove(&mut self, index: usize) -> C::Item {
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
            self.len -= L::one();
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
    pub fn swap_remove(&mut self, index: usize) -> C::Item {
        let len = self.len();
        assert!(index < len);
        unsafe {
            // We replace self[index] with the last element. Note that if the
            // bounds check above succeeds there must be a last element (which
            // can be self[index] itself).
            let value = ptr::read(self.as_ptr().add(index));
            let base_ptr = self.as_mut_ptr();
            ptr::copy(base_ptr.add(len - 1), base_ptr.add(index), 1);
            self.len -= L::one();
            value
        }
    }

    /// Drop all elements in the vector and set its length to zero.
    pub fn clear(&mut self) {
        self.truncate(0);
    }

    /// Slice of the vector content.
    pub fn as_slice(&self) -> &[C::Item] {
        unsafe { slice_assume_occupied_ref(self.data.as_ref().get_unchecked(..self.len())) }
    }

    /// Mutable slice of the vector content.
    pub fn as_mut_slice(&mut self) -> &mut [C::Item] {
        let len = self.len();
        unsafe { slice_assume_occupied_mut(self.data.as_mut().get_unchecked_mut(..len)) }
    }

    /// Slice of remaining free space in vector. All items are un-initialized.
    pub fn free_space_as_slice(&self) -> &[C::Slot] {
        unsafe {
            self.data
                .as_ref()
                .get_unchecked(self.len()..self.capacity())
        }
    }

    /// Mutable slice of remaining free space in vector. All items are un-initialized.
    pub fn free_space_as_mut_slice(&mut self) -> &mut [C::Slot] {
        let (len, cap) = (self.len(), self.capacity());
        unsafe { self.data.as_mut().get_unchecked_mut(len..cap) }
    }

    /// Appends elements from iterator to the vector until iterator ends or the vector is full.
    ///
    /// Returns a number of elements being appended.
    pub fn extend_from_iter<I: Iterator<Item = C::Item>>(&mut self, iter: I) -> usize {
        let mut counter = 0;
        for x in iter.take(self.capacity() - self.len()) {
            unsafe { self.push_unchecked(x) };
            counter += 1;
        }
        counter
    }

    /// Returns iterator over references of vector elements.
    pub fn iter(&self) -> Iter<C::Item> {
        self.as_slice().iter()
    }

    /// Returns iterator over mutable references of vector elements.
    pub fn iter_mut(&mut self) -> IterMut<C::Item> {
        self.as_mut_slice().iter_mut()
    }
}

impl<C: Container + ?Sized, L: Length> GenericVec<C, L>
where
    C::Item: Clone,
{
    /// Clones and appends elements in a slice to this vector until slice ends or vector capacity reached.
    ///
    /// If slice length is greater than free space in the vector then excess elements are simply ignored.
    ///
    /// Returns a number of elements being appended.
    pub fn extend_from_slice(&mut self, slice: &[C::Item]) -> usize {
        let free_space = self.free_space_as_mut_slice();
        let min_len = usize::min(free_space.len(), slice.len());
        unsafe {
            occupy_slice_cloned(
                free_space.get_unchecked_mut(..min_len),
                slice.get_unchecked(..min_len),
            );
        }
        self.len += L::from_usize(min_len).unwrap();
        min_len
    }

    /// Resizes the vector to the specified length.
    ///
    /// If `new_len` is less than vector length the the vector is truncated.
    ///
    /// If `new_len` is greater than the vector length then vector is filled with `value` up to `new_len` length.
    ///
    /// *Panics if `new_len` is greater than the vector capacity.*
    pub fn resize(&mut self, new_len: usize, value: C::Item) {
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

impl<C: Container + ?Sized, L: Length> Drop for GenericVec<C, L> {
    fn drop(&mut self) {
        for i in 0..self.len() {
            unsafe {
                mem::drop(
                    mem::replace(self.data.as_mut().get_unchecked_mut(i), C::Slot::empty())
                        .assume_occupied(),
                );
            }
        }
    }
}

impl<'a, C: Container + ?Sized, L: Length> IntoIterator for &'a GenericVec<C, L>
where
    C::Item: 'a,
{
    type Item = &'a C::Item;
    type IntoIter = Iter<'a, C::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, C: Container + ?Sized, L: Length> IntoIterator for &'a mut GenericVec<C, L>
where
    C::Item: 'a,
{
    type Item = &'a mut C::Item;
    type IntoIter = IterMut<'a, C::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<I, C: Container + ?Sized, L: Length> Index<I> for GenericVec<C, L>
where
    I: SliceIndex<[C::Item]>,
{
    type Output = I::Output;

    fn index(&self, index: I) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<I, C: Container + ?Sized, L: Length> IndexMut<I> for GenericVec<C, L>
where
    I: SliceIndex<[C::Item]>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        &mut self.as_mut_slice()[index]
    }
}

impl<C: Container + ?Sized, L: Length> Hash for GenericVec<C, L>
where
    C::Item: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_slice().hash(state)
    }
}

impl<C: Container + ?Sized, L: Length> fmt::Debug for GenericVec<C, L>
where
    C::Item: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.as_slice().fmt(f)
    }
}

impl<C: Container + ?Sized, L: Length> Deref for GenericVec<C, L> {
    type Target = [C::Item];

    fn deref(&self) -> &[C::Item] {
        self.as_slice()
    }
}

impl<C: Container + ?Sized, L: Length> DerefMut for GenericVec<C, L> {
    fn deref_mut(&mut self) -> &mut [C::Item] {
        self.as_mut_slice()
    }
}

impl<C: Container + ?Sized, L: Length> AsRef<GenericVec<C, L>> for GenericVec<C, L> {
    fn as_ref(&self) -> &GenericVec<C, L> {
        self
    }
}

impl<C: Container + ?Sized, L: Length> AsRef<[C::Item]> for GenericVec<C, L> {
    fn as_ref(&self) -> &[C::Item] {
        self.as_slice()
    }
}

impl<C: Container + ?Sized, L: Length> AsMut<GenericVec<C, L>> for GenericVec<C, L> {
    fn as_mut(&mut self) -> &mut GenericVec<C, L> {
        self
    }
}

impl<C: Container + ?Sized, L: Length> AsMut<[C::Item]> for GenericVec<C, L> {
    fn as_mut(&mut self) -> &mut [C::Item] {
        self.as_mut_slice()
    }
}

impl<C: Container + ?Sized, L: Length> Borrow<[C::Item]> for GenericVec<C, L> {
    fn borrow(&self) -> &[C::Item] {
        self.as_slice()
    }
}

impl<C: Container + ?Sized, L: Length> BorrowMut<[C::Item]> for GenericVec<C, L> {
    fn borrow_mut(&mut self) -> &mut [C::Item] {
        self.as_mut_slice()
    }
}

impl<C: Container + ?Sized, L: Length> Extend<C::Item> for GenericVec<C, L> {
    fn extend<I: IntoIterator<Item = C::Item>>(&mut self, iter: I) {
        self.extend_from_iter(iter.into_iter());
    }
}
