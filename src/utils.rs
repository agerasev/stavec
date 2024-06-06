use crate::traits::Slot;

/// Assume that slice of [`MaybeUninit`] is occupied.
///
/// # Safety
///
/// Slice contents must be occupied.
pub(crate) unsafe fn slice_assume_init_ref<S: Slot>(slice: &[S]) -> &[S::Item] {
    &*(slice as *const [S] as *const [S::Item])
}

/// Assume that mutable slice of [`MaybeUninit`] is occupied.
///
/// # Safety
///
/// Slice contents must be occupied.
pub(crate) unsafe fn slice_assume_init_mut<S: Slot>(slice: &mut [S]) -> &mut [S::Item] {
    &mut *(slice as *mut [S] as *mut [S::Item])
}

/// Clones the elements from `src` to `this`, returning a mutable reference to the now occupied contents of `this`.
/// Any already occupied elements will not be dropped.
pub fn uninit_write_slice_cloned<'a, S: Slot>(
    this: &'a mut [S],
    slice: &[S::Item],
) -> &'a mut [S::Item]
where
    S::Item: Clone,
{
    assert_eq!(this.len(), slice.len());
    for (dst, src) in this.iter_mut().zip(slice.iter().cloned()) {
        *dst = S::new(src);
    }
    unsafe { slice_assume_init_mut(this) }
}
