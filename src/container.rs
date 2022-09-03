use core::{
    convert::{AsMut, AsRef},
    mem::MaybeUninit,
};

pub unsafe trait Container<T>: AsRef<[MaybeUninit<T>]> + AsMut<[MaybeUninit<T>]> {}

pub unsafe trait SizedContainer<T>: Container<T> + Sized {
    fn new_uninit() -> Self;
}

unsafe impl<T, const N: usize> Container<T> for [MaybeUninit<T>; N] {}

unsafe impl<T, const N: usize> SizedContainer<T> for [MaybeUninit<T>; N] {
    fn new_uninit() -> Self {
        unsafe { MaybeUninit::uninit().assume_init() }
    }
}

unsafe impl<T> Container<T> for [MaybeUninit<T>] {}
