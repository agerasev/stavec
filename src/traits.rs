use core::{
    convert::{AsMut, AsRef},
    mem::MaybeUninit,
};
use num_traits::{Bounded, FromPrimitive, NumAssign, ToPrimitive, Unsigned};

/// Slot for `T` that may be empty or occupied.
///
/// # Safety
///
/// Must have the same layout as `T`.
pub unsafe trait Slot: Sized {
    type Item: Sized;

    fn new(item: Self::Item) -> Self;
    /// # Safety
    ///
    /// Data in the slot must be initialized.
    unsafe fn assume_init(self) -> Self::Item;
    /// # Safety
    ///
    /// Data in the slot must be initialized.
    unsafe fn assume_init_read(&self) -> Self::Item;
}
pub trait UninitSlot: Slot {
    fn uninit() -> Self;
}

unsafe impl<T> Slot for MaybeUninit<T> {
    type Item = T;

    fn new(item: T) -> Self {
        Self::new(item)
    }
    unsafe fn assume_init(self) -> Self::Item {
        self.assume_init()
    }
    unsafe fn assume_init_read(&self) -> Self::Item {
        self.assume_init_read()
    }
}
impl<T> UninitSlot for MaybeUninit<T> {
    fn uninit() -> Self {
        Self::uninit()
    }
}

unsafe impl Slot for u8 {
    type Item = u8;

    fn new(byte: u8) -> Self {
        byte
    }
    unsafe fn assume_init(self) -> Self::Item {
        self
    }
    unsafe fn assume_init_read(&self) -> Self::Item {
        *self
    }
}
impl UninitSlot for u8 {
    fn uninit() -> Self {
        0
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
impl<S: UninitSlot, const N: usize> DefaultContainer for [S; N] {
    fn default() -> Self {
        [(); N].map(|()| S::uninit())
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
