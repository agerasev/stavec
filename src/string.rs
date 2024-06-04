use crate::{
    traits::{Container, Length},
    GenericVec,
};
use core::{
    borrow::{Borrow, BorrowMut},
    convert::{AsMut, AsRef},
    fmt,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
    str::{from_utf8_unchecked, from_utf8_unchecked_mut},
};

#[repr(transparent)]
pub struct GenericString<C: Container<Item = u8> + ?Sized, L: Length = usize> {
    bytes: GenericVec<C, L>,
}

impl<C: Container<Item = u8> + ?Sized, L: Length> GenericString<C, L> {
    /// Provides an access to underlying vector.
    pub fn as_vec(&self) -> &GenericVec<C, L> {
        &self.bytes
    }
    /// Provides a mutable access to underlying vector.
    ///
    /// # Safety
    ///
    /// Vector contents must remain valid UTF-8.
    pub unsafe fn as_vec_mut(&mut self) -> &mut GenericVec<C, L> {
        &mut self.bytes
    }
}

impl<C: Container<Item = u8> + ?Sized, L: Length> GenericString<C, L> {
    /// The number of elements in the string.
    pub fn len(&self) -> usize {
        self.as_str().len()
    }

    /// Checks whether the vector is empty.
    pub fn is_empty(&self) -> bool {
        self.as_str().is_empty()
    }

    /// Drop all elements in the vector and set its length to zero.
    pub fn clear(&mut self) {
        self.bytes.clear();
    }

    /// String slice.
    pub fn as_str(&self) -> &str {
        unsafe { from_utf8_unchecked(self.bytes.as_slice()) }
    }
    /// Mutable string slice.
    pub fn as_mut_str(&mut self) -> &mut str {
        unsafe { from_utf8_unchecked_mut(self.bytes.as_mut_slice()) }
    }
}

impl<C: Container<Item = u8> + ?Sized, L: Length> Hash for GenericString<C, L> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl<C: Container<Item = u8> + ?Sized, L: Length> fmt::Debug for GenericString<C, L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        self.as_str().fmt(f)
    }
}

impl<C: Container<Item = u8> + ?Sized, L: Length> Deref for GenericString<C, L> {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl<C: Container<Item = u8> + ?Sized, L: Length> DerefMut for GenericString<C, L> {
    fn deref_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}

impl<C: Container<Item = u8> + ?Sized, L: Length> AsRef<GenericString<C, L>>
    for GenericString<C, L>
{
    fn as_ref(&self) -> &GenericString<C, L> {
        self
    }
}

impl<C: Container<Item = u8> + ?Sized, L: Length> AsRef<str> for GenericString<C, L> {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl<C: Container<Item = u8> + ?Sized, L: Length> AsMut<GenericString<C, L>>
    for GenericString<C, L>
{
    fn as_mut(&mut self) -> &mut GenericString<C, L> {
        self
    }
}

impl<C: Container<Item = u8> + ?Sized, L: Length> AsMut<str> for GenericString<C, L> {
    fn as_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}

impl<C: Container<Item = u8> + ?Sized, L: Length> Borrow<str> for GenericString<C, L> {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl<C: Container<Item = u8> + ?Sized, L: Length> BorrowMut<str> for GenericString<C, L> {
    fn borrow_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}
