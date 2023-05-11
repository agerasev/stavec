use core::{
    convert::{AsMut, AsRef},
    mem::MaybeUninit,
};
use num_traits::{Bounded, FromPrimitive, NumAssign, ToPrimitive, Unsigned};

/// Slot for `T` that may be empty or occupied.
///
/// Must have the same layout as `T`.
pub unsafe trait Slot: Sized {
    type Item: Sized;

    fn empty() -> Self;
    fn occupied(item: Self::Item) -> Self;
    unsafe fn assume_occupied(self) -> Self::Item;
}
unsafe impl<T> Slot for MaybeUninit<T> {
    type Item = T;

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
    type Slot: Slot<Item = Self::Item>;
}

/// Default container.
///
/// Exists because [`Default`] is not implemented for generic-length array (`[T; N]`).
pub trait DefaultContainer: Container + Sized {
    fn default() -> Self;
}

unsafe impl<S: Slot, const N: usize> Container for [S; N] {
    type Item = S::Item;
    type Slot = S;
}
impl<S: Slot, const N: usize> DefaultContainer for [S; N] {
    fn default() -> Self {
        [(); N].map(|()| S::empty())
    }
}

unsafe impl<S: Slot> Container for [S] {
    type Item = S::Item;
    type Slot = S;
}

unsafe impl<'a, S: Slot> Container for &'a mut [S] {
    type Item = S::Item;
    type Slot = S;
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
