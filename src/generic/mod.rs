mod maybe_sized;
mod sized;

use crate::Container;
use core::marker::PhantomData;

pub struct GenericVec<T, C: Container<T> + ?Sized> {
    _phantom: PhantomData<T>,
    len: usize,
    data: C,
}
