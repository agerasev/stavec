use crate::{
    traits::{Container, Length},
    GenericVec,
};
use core::cmp::Ordering;

impl<C: Container + ?Sized, L: Length> PartialOrd for GenericVec<C, L>
where
    C::Item: PartialOrd,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.as_slice().partial_cmp(other.as_slice())
    }
}

impl<C: Container + ?Sized, L: Length> Ord for GenericVec<C, L>
where
    C::Item: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_slice().cmp(other.as_slice())
    }
}

impl<C: Container + ?Sized, L: Length> PartialEq for GenericVec<C, L>
where
    C::Item: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}

impl<C: Container + ?Sized, L: Length> Eq for GenericVec<C, L> where C::Item: Eq {}

impl<const M: usize, C: Container + ?Sized, L: Length> PartialEq<[C::Item; M]> for GenericVec<C, L>
where
    C::Item: PartialEq,
{
    fn eq(&self, other: &[C::Item; M]) -> bool {
        self.as_slice().eq(&other[..])
    }
}

impl<C: Container + ?Sized, L: Length> PartialEq<[C::Item]> for GenericVec<C, L>
where
    C::Item: PartialEq,
{
    fn eq(&self, other: &[C::Item]) -> bool {
        self.as_slice().eq(other)
    }
}

impl<C: Container + ?Sized, L: Length> PartialEq<&[C::Item]> for GenericVec<C, L>
where
    C::Item: PartialEq,
{
    fn eq(&self, other: &&[C::Item]) -> bool {
        self.as_slice().eq(*other)
    }
}

impl<C: Container + ?Sized, L: Length> PartialEq<&mut [C::Item]> for GenericVec<C, L>
where
    C::Item: PartialEq,
{
    fn eq(&self, other: &&mut [C::Item]) -> bool {
        self.as_slice().eq(*other)
    }
}

#[cfg(feature = "std")]
impl<C: Container + ?Sized, L: Length> PartialEq<std::vec::Vec<C::Item>> for GenericVec<C, L>
where
    C::Item: PartialEq,
{
    fn eq(&self, other: &std::vec::Vec<C::Item>) -> bool {
        self.as_slice().eq(other.as_slice())
    }
}
