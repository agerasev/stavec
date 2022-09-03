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
