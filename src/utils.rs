use core::mem::MaybeUninit;

/// Assume that slice of [`MaybeUninit`] is initialized.
///
/// # Safety
///
/// Slice contents must be initialized.
//
// TODO: Remove on `maybe_uninit_slice` stabilization.
pub(crate) unsafe fn slice_assume_init_ref<T>(slice: &[MaybeUninit<T>]) -> &[T] {
    &*(slice as *const [MaybeUninit<T>] as *const [T])
}

/// Assume that mutable slice of [`MaybeUninit`] is initialized.
///
/// # Safety
///
/// Slice contents must be initialized.
//
// TODO: Remove on `maybe_uninit_slice` stabilization.
pub(crate) unsafe fn slice_assume_init_mut<T>(slice: &mut [MaybeUninit<T>]) -> &mut [T] {
    &mut *(slice as *mut [MaybeUninit<T>] as *mut [T])
}

/// Clones the elements from `src` to `this`, returning a mutable reference to the now initialized contents of `this`.
/// Any already initialized elements will not be dropped.
///
/// TODO: Remove on `maybe_uninit_write_slice` stabilization.
pub fn uninit_write_slice_cloned<'a, T: Clone>(
    this: &'a mut [MaybeUninit<T>],
    slice: &[T],
) -> &'a mut [T] {
    assert_eq!(this.len(), slice.len());
    for (dst, src) in this.iter_mut().zip(slice.iter().cloned()) {
        dst.write(src);
    }
    unsafe { slice_assume_init_mut(this) }
}
