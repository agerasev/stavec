use core::{
    convert::{AsMut, AsRef},
    mem::MaybeUninit,
};
use num_traits::{Bounded, FromPrimitive, NumAssign, ToPrimitive, Unsigned};

/// Abstract container. May be unsized.
///
/// # Safety
///
/// [`as_ref()`](`AsRef::as_ref`) and [`as_mut()`](`AsMut::as_mut`) must provide the same slices with the always the same content and unchanged length.
pub unsafe trait Container<T>: AsRef<[MaybeUninit<T>]> + AsMut<[MaybeUninit<T>]> {}

/// Default container.
///
/// Exists because [`Default`] is not implemented for generic-length array (`[T; N]`).
pub trait DefaultContainer<T>: Container<T> + Sized {
    fn default() -> Self;
}

unsafe impl<T, const N: usize> Container<T> for [MaybeUninit<T>; N] {}
impl<T, const N: usize> DefaultContainer<T> for [MaybeUninit<T>; N] {
    fn default() -> Self {
        unsafe { MaybeUninit::uninit().assume_init() }
    }
}

unsafe impl<T> Container<T> for [MaybeUninit<T>] {}

unsafe impl<'a, T> Container<T> for &'a mut [MaybeUninit<T>] {}

/// Abstract type that could be used as vector length.
pub trait Length:
    Unsigned + NumAssign + Copy + Sized + Ord + Bounded + ToPrimitive + FromPrimitive
{
}

impl<T> Length for T where
    T: Unsigned + NumAssign + Copy + Sized + Ord + Bounded + ToPrimitive + FromPrimitive
{
}
