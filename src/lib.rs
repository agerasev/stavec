#![no_std]

#[cfg(feature = "std")]
extern crate std;

/// Traits for [`GenericVec`] parameters.
///
/// You probably won't need them is you use only [`StaticVec`].
pub mod traits;

mod cmp;
mod iter;
mod static_;
mod string;
mod utils;
mod vec;

#[cfg(test)]
mod tests;

pub use iter::IntoIter;
pub use static_::StaticVec;
pub use string::GenericString;
pub use vec::GenericVec;
