use crate::StaticVec;
use core::cmp::Ordering;

impl<T, const N: usize> PartialOrd for StaticVec<T, N>
where
    T: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl<T, const N: usize> Ord for StaticVec<T, N>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl<T, const N: usize> PartialEq for StaticVec<T, N>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl<T, const N: usize> Eq for StaticVec<T, N> where T: Eq {}

impl<T, const M: usize, const N: usize> PartialEq<[T; M]> for StaticVec<T, N>
where
    T: PartialEq,
{
    fn eq(&self, other: &[T; M]) -> bool {
        self.as_slice().eq(&other[..])
    }
}

impl<T, const N: usize> PartialEq<[T]> for StaticVec<T, N>
where
    T: PartialEq,
{
    fn eq(&self, other: &[T]) -> bool {
        self.as_slice().eq(other)
    }
}

impl<T, const N: usize> PartialEq<&[T]> for StaticVec<T, N>
where
    T: PartialEq,
{
    fn eq(&self, other: &&[T]) -> bool {
        self.as_slice().eq(*other)
    }
}

impl<T, const N: usize> PartialEq<&mut [T]> for StaticVec<T, N>
where
    T: PartialEq,
{
    fn eq(&self, other: &&mut [T]) -> bool {
        self.as_slice().eq(*other)
    }
}

#[cfg(feature = "std")]
impl<T, const N: usize> PartialEq<std::vec::Vec<T>> for StaticVec<T, N>
where
    T: PartialEq,
{
    fn eq(&self, other: &std::vec::Vec<T>) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}
