use crate::GenericVec;
use core::{
    convert::{AsMut, AsRef},
    iter::{FromIterator, IntoIterator},
    mem::{ManuallyDrop, MaybeUninit},
    ptr,
};

/// Stack-allocated vector with static capacity.
pub type StaticVec<T, const N: usize> = GenericVec<T, [MaybeUninit<T>; N]>;

impl<T, const N: usize> StaticVec<T, N> {
    pub const CAPACITY: usize = N;

    /// Creates the vector filling it with the value returned from closure up to specified length.
    ///
    /// The vector is filled sequentially by subsequent closure calls.
    ///
    /// *Panics if `len` is greater than the vector capacity.*
    pub fn fill_with<F: Fn() -> T>(len: usize, func: F) -> Self {
        assert!(len <= N);
        let mut self_ = Self::new();
        for _ in 0..len {
            unsafe { self_.push_unchecked(func()) };
        }
        self_
    }

    /// Constructs a new vector from array of values.
    ///
    /// *Panics if passed array size is greater than vector capacity.*
    pub fn from_array<const M: usize>(array: [T; M]) -> Self {
        assert!(M <= N); // TODO: Use static assert.
        Self::from_iter(IntoIterator::into_iter(array))
    }
}

impl<T: Clone, const N: usize> StaticVec<T, N> {
    /// Creates the vector filling it with the cloned value up to specified length.
    ///
    /// *Panics if `len` is greater than the vector capacity.*
    pub fn fill(len: usize, value: T) -> Self {
        assert!(len <= N);
        let mut self_ = Self::new();
        for _ in 0..len {
            unsafe { self_.push_unchecked(value.clone()) };
        }
        self_
    }
}

impl<T, const N: usize> AsRef<GenericVec<T, [MaybeUninit<T>]>> for StaticVec<T, N> {
    fn as_ref(&self) -> &GenericVec<T, [MaybeUninit<T>]> {
        self as &GenericVec<T, [MaybeUninit<T>]>
    }
}

impl<T, const N: usize> AsMut<GenericVec<T, [MaybeUninit<T>]>> for StaticVec<T, N> {
    fn as_mut(&mut self) -> &mut GenericVec<T, [MaybeUninit<T>]> {
        self as &mut GenericVec<T, [MaybeUninit<T>]>
    }
}

impl<T, const N: usize> From<[T; N]> for StaticVec<T, N> {
    fn from(array: [T; N]) -> Self {
        let array = ManuallyDrop::new(array);
        unsafe { Self::from_raw_parts(ptr::read(array.as_ptr() as *const [MaybeUninit<T>; N]), N) }
    }
}

impl<T, const N: usize> TryFrom<StaticVec<T, N>> for [T; N] {
    type Error = ();

    /// Converts the static vector into an array.
    ///
    /// This only succeedes if the vector is full and thus actually contains `N` initialized elements.
    fn try_from(vec: StaticVec<T, N>) -> Result<Self, Self::Error> {
        if vec.is_full() {
            unsafe { Ok(ptr::read(vec.into_raw_parts().0.as_ptr() as *const [T; N])) }
        } else {
            Err(())
        }
    }
}
