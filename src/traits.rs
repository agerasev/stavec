use core::{
    convert::{AsMut, AsRef},
    mem::MaybeUninit,
};
use num_traits::{Bounded, FromPrimitive, ToPrimitive, Unsigned};

/// Abstract container. May be unsized.
///
/// # Safety
///
/// [`as_ref()`](`AsRef::as_ref`) and [`as_mut()`](`AsMut::as_mut`) must provide the same slices with the always the same content and unchanged length.
pub unsafe trait Container<T>: AsRef<[MaybeUninit<T>]> + AsMut<[MaybeUninit<T>]> {}

/// Abstract sized container. Like [`Container`] but sized.
///
/// # Safety
///
/// Requirements are the same as for [`Container`].
pub unsafe trait SizedContainer<T>: Container<T> + Sized {
    /// Create a new container with uninitialized contents.
    fn new_uninit() -> Self;
}

unsafe impl<T, const N: usize> Container<T> for [MaybeUninit<T>; N] {}

unsafe impl<T, const N: usize> SizedContainer<T> for [MaybeUninit<T>; N] {
    fn new_uninit() -> Self {
        unsafe { MaybeUninit::uninit().assume_init() }
    }
}

unsafe impl<T> Container<T> for [MaybeUninit<T>] {}

/// Abstract type that could be used as vector length.
pub trait Length: Unsigned + Copy + Sized + Ord + Bounded + ToPrimitive + FromPrimitive {}

impl<T> Length for T where T: Unsigned + Copy + Sized + Ord + Bounded + ToPrimitive + FromPrimitive {}
