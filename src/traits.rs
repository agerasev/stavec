use core::{
    convert::{AsMut, AsRef},
    mem::MaybeUninit,
};
use num_traits::{Bounded, FromPrimitive, NumAssign, ToPrimitive, Unsigned};

/// Slot for `T` that may be empty or occupied.
///
/// Must have the same layout as `T`.
pub unsafe trait Slot<T: Sized>: Sized {
    fn empty() -> Self;
    fn occupied(item: T) -> Self;
    unsafe fn assume_occupied(self) -> T;
}
unsafe impl<T> Slot<T> for MaybeUninit<T> {
    fn empty() -> Self {
        Self::uninit()
    }
    fn occupied(item: T) -> Self {
        Self::new(item)
    }
    unsafe fn assume_occupied(self) -> T {
        self.assume_init()
    }
}

/// Abstract container. May be unsized.
///
/// # Safety
///
/// [`as_ref()`](`AsRef::as_ref`) and [`as_mut()`](`AsMut::as_mut`) must provide the same slices with the always the same content and unchanged length.
pub unsafe trait Container: AsRef<[Self::Slot]> + AsMut<[Self::Slot]> {
    type Item: Sized;
    type Slot: Slot<Self::Item>;
}

/// Default container.
///
/// Exists because [`Default`] is not implemented for generic-length array (`[T; N]`).
pub trait DefaultContainer: Container + Sized {
    fn default() -> Self;
}

unsafe impl<T, const N: usize> Container for [MaybeUninit<T>; N] {
    type Item = T;
    type Slot = MaybeUninit<T>;
}
impl<T, const N: usize> DefaultContainer for [MaybeUninit<T>; N] {
    fn default() -> Self {
        unsafe { MaybeUninit::uninit().assume_init() }
    }
}

unsafe impl<T> Container for [MaybeUninit<T>] {
    type Item = T;
    type Slot = MaybeUninit<T>;
}

unsafe impl<'a, T> Container for &'a mut [MaybeUninit<T>] {
    type Item = T;
    type Slot = MaybeUninit<T>;
}

/// Abstract type that could be used as vector length.
pub trait Length:
    Unsigned + NumAssign + Copy + Sized + Ord + Bounded + ToPrimitive + FromPrimitive
{
}

impl<T> Length for T where
    T: Unsigned + NumAssign + Copy + Sized + Ord + Bounded + ToPrimitive + FromPrimitive
{
}
