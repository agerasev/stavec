use crate::{
    traits::{Container, DefaultContainer, Length},
    utils::FullError,
    GenericVec,
};
use core::{
    borrow::{Borrow, BorrowMut},
    convert::{AsMut, AsRef},
    fmt,
    hash::{Hash, Hasher},
    ops::{Deref, DerefMut},
    str::{from_utf8, from_utf8_unchecked, from_utf8_unchecked_mut, Utf8Error},
};

#[repr(transparent)]
#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct GenericString<C: Container<Item = u8> + ?Sized, L: Length = usize> {
    bytes: GenericVec<C, L>,
}

impl<C: DefaultContainer<Item = u8>, L: Length> Default for GenericString<C, L> {
    fn default() -> Self {
        Self {
            bytes: GenericVec::default(),
        }
    }
}
impl<C: DefaultContainer<Item = u8>, L: Length> Clone for GenericString<C, L> {
    fn clone(&self) -> Self {
        Self {
            bytes: self.bytes.clone(),
        }
    }
}

impl<C: DefaultContainer<Item = u8>, L: Length> GenericString<C, L> {
    fn try_from_str(s: &str) -> Result<Self, FullError> {
        let mut self_ = Self::default();
        self_.push_str(s)?;
        Ok(self_)
    }
}
impl<C: DefaultContainer<Item = u8>, L: Length> TryFrom<&str> for GenericString<C, L> {
    type Error = FullError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        Self::try_from_str(s)
    }
}

impl<C: DefaultContainer<Item = u8>, L: Length> GenericString<C, L> {
    pub fn try_from_vec(vec: GenericVec<C, L>) -> Result<Self, Utf8Error> {
        from_utf8(vec.as_slice())?;
        Ok(Self { bytes: vec })
    }
}
impl<C: DefaultContainer<Item = u8>, L: Length> TryFrom<GenericVec<C, L>> for GenericString<C, L> {
    type Error = Utf8Error;

    fn try_from(vec: GenericVec<C, L>) -> Result<Self, Self::Error> {
        Self::try_from_vec(vec)
    }
}

impl<C: Container<Item = u8> + ?Sized, L: Length> GenericString<C, L> {
    pub fn capacity(&self) -> usize {
        self.bytes.capacity()
    }
    pub fn len(&self) -> usize {
        self.bytes.len()
    }
    pub fn remaining(&self) -> usize {
        self.bytes.remaining()
    }
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }
    pub fn is_full(&self) -> bool {
        self.bytes.is_full()
    }

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

    /// String slice.
    pub fn as_str(&self) -> &str {
        unsafe { from_utf8_unchecked(self.bytes.as_slice()) }
    }
    /// Mutable string slice.
    pub fn as_mut_str(&mut self) -> &mut str {
        unsafe { from_utf8_unchecked_mut(self.bytes.as_mut_slice()) }
    }

    pub fn clear(&mut self) {
        self.bytes.clear();
    }

    pub fn push(&mut self, c: char) -> Result<(), FullError> {
        let mut bytes = [0; 4];
        c.encode_utf8(&mut bytes);
        self.bytes.extend_from_slice(&bytes[..c.len_utf8()])
    }
    pub fn push_str(&mut self, s: &str) -> Result<(), FullError> {
        self.bytes.extend_from_slice(s.as_bytes())
    }
}

impl<C: Container<Item = u8> + ?Sized, L: Length> Hash for GenericString<C, L> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl<C: Container<Item = u8> + ?Sized, L: Length> fmt::Debug for GenericString<C, L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

impl<C: Container<Item = u8> + ?Sized, L: Length> fmt::Display for GenericString<C, L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self.as_str(), f)
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
