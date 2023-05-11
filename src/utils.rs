use crate::traits::Slot;

/// Assume that slice of [`MaybeUninit`] is occupied.
///
/// # Safety
///
/// Slice contents must be occupied.
pub(crate) unsafe fn slice_assume_occupied_ref<T, S: Slot<T>>(slice: &[S]) -> &[T] {
    &*(slice as *const [S] as *const [T])
}

/// Assume that mutable slice of [`MaybeUninit`] is occupied.
///
/// # Safety
///
/// Slice contents must be occupied.
pub(crate) unsafe fn slice_assume_occupied_mut<T, S: Slot<T>>(slice: &mut [S]) -> &mut [T] {
    &mut *(slice as *mut [S] as *mut [T])
}

/// Clones the elements from `src` to `this`, returning a mutable reference to the now occupied contents of `this`.
/// Any already occupied elements will not be dropped.
pub fn occupy_slice_cloned<'a, T: Clone, S: Slot<T>>(
    this: &'a mut [S],
    slice: &[T],
) -> &'a mut [T] {
    assert_eq!(this.len(), slice.len());
    for (dst, src) in this.iter_mut().zip(slice.iter().cloned()) {
        *dst = S::occupied(src);
    }
    unsafe { slice_assume_occupied_mut(this) }
}
